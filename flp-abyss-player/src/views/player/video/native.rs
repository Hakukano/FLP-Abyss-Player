use std::{
    mem::{size_of, size_of_val},
    path::Path,
    sync::Arc,
};

use anyhow::Result;
use eframe::{egui, epaint::Vec2};
use gst::prelude::*;
use gstreamer::{self as gst, element_error};
use gstreamer_app as gst_app;
use gstreamer_video as gst_video;
use parking_lot::RwLock;

use crate::utils::helper::scale_fit_all;

use super::CONTROLLER_HEIGHT;

macro_rules! gl_strict {
    ($gl:expr, $stmt:stmt) => {
        $stmt
        {
            let error = $gl.get_error();
            if error != 0 {
                panic!("gl operation error: {error}")
            }
        }
    };
}

#[rustfmt::skip]
const VERTICES: &[f32] = &[
     1.0,  1.0, 0.0, 1.0, 0.0,
    -1.0,  1.0, 0.0, 0.0, 0.0,
    -1.0, -1.0, 0.0, 0.0, 1.0,
     1.0, -1.0, 0.0, 1.0, 1.0,
];

const INDICES: &[u16] = &[0, 1, 2, 0, 2, 3];

#[cfg(target_arch = "wasm32")]
const SHADER_VERSION: &str = "#version 300 es";
#[cfg(not(target_arch = "wasm32"))]
const SHADER_VERSION: &str = "#version 330";

const VS_SRC: &str = r#"
layout(location = 0) in vec4 a_position;
layout(location = 1) in vec2 a_texcoord;
uniform mat4 u_transformation;
out vec2 v_texcoord;

void main() {
    gl_Position = u_transformation * a_position;
    v_texcoord = a_texcoord;
}"#;

const FS_SRC: &str = r#"
#ifdef GL_ES
precision mediump float;
#endif
in vec2 v_texcoord;
uniform sampler2D tex;
layout(location = 0) out vec4 fragColor;

void main() {
    fragColor = texture(tex, v_texcoord);
}"#;

struct SourceElements {
    source: gst::Element,
    demux: gst::Element,
}

impl SourceElements {
    fn new(video_path: impl AsRef<Path>) -> Self {
        let extension = video_path
            .as_ref()
            .extension()
            .expect("No file extension found")
            .to_str()
            .expect("Invalid file extension");
        Self {
            source: gst::ElementFactory::make("filesrc")
                .name("source")
                .property("location", video_path.as_ref().display().to_string())
                .build()
                .expect("Cannot build source"),
            demux: gst::ElementFactory::make(match extension {
                "avi" => "avidemux",
                "mkv" | "webm" => "matroskademux",
                _ => "qtdemux",
            })
            .name("demux")
            .build()
            .expect("Cannot build demux"),
        }
    }

    fn add_to_pipeline(&self, pipeline: &gst::Pipeline) {
        pipeline
            .add_many(&[&self.source, &self.demux])
            .expect("Cannot add source elements to pipeline");
    }

    fn link(&self, video_queue: gst::Element, audio_queue: gst::Element) {
        gst::Element::link_many(&[&self.source, &self.demux]).expect("Cannot link source elements");

        self.demux.connect_pad_added(move |_src, src_pad| {
            let new_pad_type = src_pad
                .current_caps()
                .expect("Failed to get caps of new pad.")
                .structure(0)
                .expect("Failed to get first structure of caps.")
                .name();
            if new_pad_type.starts_with("video/") {
                let video_queue_sink = video_queue
                    .static_pad("sink")
                    .expect("Cannot get convert sink pad");
                if video_queue_sink.is_linked() {
                    return;
                }
                src_pad
                    .link(&video_queue_sink)
                    .expect("Cannot link video sink to demux src");
            } else if new_pad_type.starts_with("audio/") {
                let audio_queue_sink = audio_queue
                    .static_pad("sink")
                    .expect("Cannot get convert sink pad");
                if audio_queue_sink.is_linked() {
                    return;
                }
                src_pad
                    .link(&audio_queue_sink)
                    .expect("Cannot link audio sink to demux src");
            } else {
                return;
            }
        });
    }
}

struct VideoElements {
    queue: gst::Element,
    decode: gst::Element,
    convert: gst::Element,
    scale: gst::Element,
    sink: gst_app::AppSink,
}

impl VideoElements {
    fn new() -> Self {
        Self {
            queue: gst::ElementFactory::make("queue")
                .name("video_queue")
                .build()
                .expect("Cannot build queue"),
            decode: gst::ElementFactory::make("decodebin")
                .name("video_decode")
                .build()
                .expect("Cannot build decodebin"),
            convert: gst::ElementFactory::make("videoconvert")
                .name("video_convert")
                .build()
                .expect("Cannot build videoconvert"),
            scale: gst::ElementFactory::make("videoscale")
                .name("video_scale")
                .build()
                .expect("Cannot build videoscale"),
            sink: gst_app::AppSink::builder()
                .name("video_sink")
                .caps(
                    &gst_video::VideoCapsBuilder::new()
                        .format(gst_video::VideoFormat::Rgba)
                        .build(),
                )
                .build(),
        }
    }

    fn add_to_pipeline(&self, pipeline: &gst::Pipeline) {
        pipeline
            .add_many(&[
                &self.queue,
                &self.decode,
                &self.convert,
                &self.scale,
                self.sink.upcast_ref(),
            ])
            .expect("Cannot add video elements to pipeline");
    }

    fn link(&self) {
        gst::Element::link_many(&[&self.queue, &self.decode]).expect("Cannot link video elements");
        gst::Element::link_many(&[&self.convert, &self.scale, self.sink.upcast_ref()])
            .expect("Cannot link video elements");

        let convert = self.convert.clone();
        self.decode.connect_pad_added(move |_src, src_pad| {
            let convert_sink = convert
                .static_pad("sink")
                .expect("Cannot get convert sink pad");
            if convert_sink.is_linked() {
                return;
            }

            let new_pad_type = src_pad
                .current_caps()
                .expect("Failed to get caps of new pad.")
                .structure(0)
                .expect("Failed to get first structure of caps.")
                .name();
            if !new_pad_type.starts_with("video/x-raw") {
                return;
            }

            src_pad
                .link(&convert_sink)
                .expect("Cannot link convert sink to decode src");
        });
    }

    fn set_sink_callback(&self, video_frame: Arc<RwLock<VideoFrame>>, ctx: egui::Context) {
        self.sink.set_callbacks(
            gst_app::AppSinkCallbacks::builder()
                .new_sample(move |appsink| {
                    let sample = appsink.pull_sample().map_err(|_| gst::FlowError::Eos)?;
                    let structure = sample
                        .caps()
                        .expect("video sample has to have caps")
                        .structure(0)
                        .expect("video sample has to have structure");
                    let width = structure
                        .get::<i32>("width")
                        .expect("video sample has to have width");
                    let height = structure
                        .get::<i32>("height")
                        .expect("video sample has to have height");

                    let buffer = sample.buffer().ok_or_else(|| {
                        element_error!(
                            appsink,
                            gst::ResourceError::Failed,
                            ("Failed to get buffer from appsink")
                        );

                        gst::FlowError::Error
                    })?;

                    // At this point, buffer is only a reference to an existing memory region somewhere.
                    // When we want to access its content, we have to map it while requesting the required
                    // mode of access (read, read/write).
                    // This type of abstraction is necessary, because the buffer in question might not be
                    // on the machine's main memory itself, but rather in the GPU's memory.
                    // So mapping the buffer makes the underlying memory region accessible to us.
                    // See: https://gstreamer.freedesktop.org/documentation/plugin-development/advanced/allocation.html
                    let map = buffer.map_readable().map_err(|_| {
                        element_error!(
                            appsink,
                            gst::ResourceError::Failed,
                            ("Failed to map buffer readable")
                        );

                        gst::FlowError::Error
                    })?;

                    let rgbas = map.as_slice();

                    {
                        let mut video_frame_guard = video_frame.write();
                        video_frame_guard.width = width;
                        video_frame_guard.height = height;
                        video_frame_guard.rgbas = rgbas.to_vec();
                    }

                    ctx.request_repaint();

                    Ok(gst::FlowSuccess::Ok)
                })
                .build(),
        );
    }
}

struct AudioElements {
    queue: gst::Element,
    decode: gst::Element,
    convert: gst::Element,
    resample: gst::Element,
    volume: gst::Element,
    sink: gst::Element,
}

impl AudioElements {
    fn new() -> Self {
        Self {
            queue: gst::ElementFactory::make("queue")
                .name("audio_queue")
                .build()
                .expect("Cannot build queue"),
            decode: gst::ElementFactory::make("decodebin")
                .name("audio_decode")
                .build()
                .expect("Cannot build decodebin"),
            convert: gst::ElementFactory::make("audioconvert")
                .name("audio_convert")
                .build()
                .expect("Cannot build audioconvert"),
            resample: gst::ElementFactory::make("audioresample")
                .name("audio_resample")
                .build()
                .expect("Cannot build audioresample"),
            volume: gst::ElementFactory::make("volume")
                .name("audio_volume")
                .property("volume", 1.0)
                .build()
                .expect("Cannot build audio sink"),
            sink: gst::ElementFactory::make("autoaudiosink")
                .name("audio_sink")
                .build()
                .expect("Cannot build audio sink"),
        }
    }

    fn add_to_pipeline(&self, pipeline: &gst::Pipeline) {
        pipeline
            .add_many(&[
                &self.queue,
                &self.decode,
                &self.convert,
                &self.resample,
                &self.volume,
                &self.sink,
            ])
            .expect("Cannot add audio elements to pipeline");
    }

    fn link(&self) {
        gst::Element::link_many(&[&self.queue, &self.decode]).expect("Cannot link audio elements");
        gst::Element::link_many(&[&self.convert, &self.resample, &self.volume, &self.sink])
            .expect("Cannot link audio elements");

        let convert = self.convert.clone();
        self.decode.connect_pad_added(move |_src, src_pad| {
            let convert_sink = convert
                .static_pad("sink")
                .expect("Cannot get convert sink pad");
            if convert_sink.is_linked() {
                return;
            }

            let new_pad_type = src_pad
                .current_caps()
                .expect("Failed to get caps of new pad.")
                .structure(0)
                .expect("Failed to get first structure of caps.")
                .name();
            if !new_pad_type.starts_with("audio/x-raw") {
                return;
            }

            src_pad
                .link(&convert_sink)
                .expect("Cannot link convert sink to decode src");
        });
    }
}

struct VideoFrame {
    gl: Arc<glow::Context>,

    vertex_shader: glow::NativeShader,
    fragment_shader: glow::NativeShader,
    program: glow::Program,
    attr_position: u32,
    attr_texture: u32,
    vertex_array: glow::VertexArray,
    vertex_buffer: glow::Buffer,
    index_buffer: glow::Buffer,
    texture: glow::Texture,

    width: i32,
    height: i32,
    rgbas: Vec<u8>,
}

impl VideoFrame {
    fn new(gl: Arc<glow::Context>) -> Self {
        use glow::HasContext as _;

        unsafe {
            let vertex_shader = gl
                .create_shader(glow::VERTEX_SHADER)
                .expect("Cannot create shader");
            gl_strict!(
                gl,
                gl.shader_source(vertex_shader, &format!("{}\n{}", SHADER_VERSION, VS_SRC))
            );
            gl_strict!(gl, gl.compile_shader(vertex_shader));
            assert!(
                gl.get_shader_compile_status(vertex_shader),
                "Failed to compile {}: {}",
                glow::VERTEX_SHADER,
                gl.get_shader_info_log(vertex_shader)
            );

            let fragment_shader = gl
                .create_shader(glow::FRAGMENT_SHADER)
                .expect("Cannot create shader");
            gl_strict!(
                gl,
                gl.shader_source(fragment_shader, &format!("{}\n{}", SHADER_VERSION, FS_SRC))
            );
            gl_strict!(gl, gl.compile_shader(fragment_shader));
            assert!(
                gl.get_shader_compile_status(fragment_shader),
                "Failed to compile {}: {}",
                glow::FRAGMENT_SHADER,
                gl.get_shader_info_log(fragment_shader)
            );

            let program = gl.create_program().expect("Cannot create program");
            gl_strict!(gl, gl.attach_shader(program, vertex_shader));
            gl_strict!(gl, gl.attach_shader(program, fragment_shader));
            gl_strict!(gl, gl.link_program(program));
            if !gl.get_program_link_status(program) {
                panic!("{}", gl.get_program_info_log(program));
            }

            let attr_position = gl
                .get_attrib_location(program, "a_position")
                .expect("Cannot find attribute");
            let attr_texture = gl
                .get_attrib_location(program, "a_texcoord")
                .expect("Cannot find attribute");

            let vertex_array = gl
                .create_vertex_array()
                .expect("Cannot create vertex_array");
            gl_strict!(gl, gl.bind_vertex_array(Some(vertex_array)));

            let vertex_buffer = gl.create_buffer().expect("Cannot create buffer");
            gl_strict!(gl, gl.bind_buffer(glow::ARRAY_BUFFER, Some(vertex_buffer)));
            let data =
                std::slice::from_raw_parts(VERTICES.as_ptr() as *const u8, size_of_val(VERTICES));
            gl_strict!(
                gl,
                gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, data, glow::STATIC_DRAW)
            );

            let index_buffer = gl.create_buffer().expect("Cannot create buffer");
            gl_strict!(
                gl,
                gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(index_buffer))
            );
            let data = std::slice::from_raw_parts(
                INDICES.as_ptr() as *const u8,
                INDICES.len() * size_of::<usize>(),
            );
            gl_strict!(
                gl,
                gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, data, glow::STATIC_DRAW)
            );

            gl_strict!(
                gl,
                gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(index_buffer))
            );
            gl_strict!(gl, gl.bind_buffer(glow::ARRAY_BUFFER, Some(vertex_buffer)));

            gl_strict!(
                gl,
                gl.vertex_attrib_pointer_f32(
                    attr_position,
                    3,
                    glow::FLOAT,
                    false,
                    5 * size_of::<f32>() as i32,
                    0
                )
            );

            gl_strict!(
                gl,
                gl.vertex_attrib_pointer_f32(
                    attr_texture,
                    2,
                    glow::FLOAT,
                    false,
                    5 * size_of::<f32>() as i32,
                    3 * size_of::<f32>() as i32,
                )
            );

            gl_strict!(gl, gl.enable_vertex_attrib_array(attr_position));
            gl_strict!(gl, gl.enable_vertex_attrib_array(attr_texture));

            gl_strict!(gl, gl.bind_vertex_array(None));
            gl_strict!(gl, gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, None));
            gl_strict!(gl, gl.bind_buffer(glow::ARRAY_BUFFER, None));

            let texture = gl.create_texture().expect("Cannot create texture");

            Self {
                gl,
                vertex_shader,
                fragment_shader,
                program,
                attr_position,
                attr_texture,
                vertex_array,
                vertex_buffer,
                index_buffer,
                texture,
                width: 0,
                height: 0,
                rgbas: Vec::new(),
            }
        }
    }

    fn paint(&self) {
        if self.width == 0 || self.height == 0 {
            return;
        }
        use glow::HasContext as _;
        unsafe {
            let gl = &self.gl;

            gl_strict!(gl, gl.blend_color(0.0, 0.0, 0.0, 1.0));
            gl_strict!(
                gl,
                gl.blend_func_separate(
                    glow::SRC_ALPHA,
                    glow::CONSTANT_COLOR,
                    glow::ONE,
                    glow::ONE_MINUS_SRC_ALPHA,
                )
            );
            gl_strict!(gl, gl.blend_equation(glow::FUNC_ADD));
            gl_strict!(gl, gl.enable(glow::BLEND));

            gl_strict!(gl, gl.use_program(Some(self.program)));

            gl_strict!(gl, gl.bind_vertex_array(Some(self.vertex_array)));
            gl_strict!(
                gl,
                gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(self.index_buffer))
            );
            gl_strict!(
                gl,
                gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vertex_buffer))
            );

            gl_strict!(
                gl,
                gl.vertex_attrib_pointer_f32(
                    self.attr_position,
                    3,
                    glow::FLOAT,
                    false,
                    5 * size_of::<f32>() as i32,
                    0
                )
            );

            gl_strict!(
                gl,
                gl.vertex_attrib_pointer_f32(
                    self.attr_texture,
                    2,
                    glow::FLOAT,
                    false,
                    5 * size_of::<f32>() as i32,
                    3 * size_of::<f32>() as i32,
                )
            );

            gl_strict!(gl, gl.enable_vertex_attrib_array(self.attr_position));
            gl_strict!(gl, gl.enable_vertex_attrib_array(self.attr_texture));

            gl_strict!(gl, gl.active_texture(glow::TEXTURE0));
            gl_strict!(gl, gl.bind_texture(glow::TEXTURE_2D, Some(self.texture)));
            gl_strict!(
                gl,
                gl.tex_parameter_i32(
                    glow::TEXTURE_2D,
                    glow::TEXTURE_MIN_FILTER,
                    glow::LINEAR as i32
                )
            );
            gl_strict!(
                gl,
                gl.tex_parameter_i32(
                    glow::TEXTURE_2D,
                    glow::TEXTURE_MAG_FILTER,
                    glow::LINEAR as i32
                )
            );
            gl_strict!(
                gl,
                gl.tex_parameter_i32(
                    glow::TEXTURE_2D,
                    glow::TEXTURE_WRAP_S,
                    glow::CLAMP_TO_BORDER as i32
                )
            );
            gl_strict!(
                gl,
                gl.tex_parameter_i32(
                    glow::TEXTURE_2D,
                    glow::TEXTURE_WRAP_T,
                    glow::CLAMP_TO_BORDER as i32
                )
            );
            gl_strict!(
                gl,
                gl.tex_image_2d(
                    glow::TEXTURE_2D,
                    0,
                    glow::RGBA as i32,
                    self.width,
                    self.height,
                    0,
                    glow::RGBA,
                    glow::UNSIGNED_BYTE,
                    Some(self.rgbas.as_slice()),
                )
            );

            let texture_location = gl
                .get_uniform_location(self.program, "tex")
                .expect("Cannot find uniform location");
            gl_strict!(gl, gl.uniform_1_i32(Some(&texture_location), 0));

            let transform_location = gl
                .get_uniform_location(self.program, "u_transformation")
                .expect("Cannot find uniform location");
            #[rustfmt::skip]
            let transform = &[
                  1.0, 0.0, 0.0, 0.0,
                  0.0, 1.0, 0.0, 0.0,
                  0.0, 0.0, 1.0, 0.0,
                  0.0, 0.0, 0.0, 1.0,
            ];
            gl_strict!(
                gl,
                gl.uniform_matrix_4_f32_slice(Some(&transform_location), false, transform)
            );

            gl_strict!(
                gl,
                gl.draw_elements(glow::TRIANGLES, 6, glow::UNSIGNED_SHORT, 0)
            );

            gl_strict!(gl, gl.bind_texture(glow::TEXTURE_2D, None));
            gl_strict!(gl, gl.use_program(None));
            gl_strict!(gl, gl.bind_vertex_array(None));
            gl_strict!(gl, gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, None));
            gl_strict!(gl, gl.bind_buffer(glow::ARRAY_BUFFER, None));
        }
    }
}

impl Drop for VideoFrame {
    fn drop(&mut self) {
        use glow::HasContext as _;
        unsafe {
            self.gl.delete_texture(self.texture);
            self.gl.delete_buffer(self.index_buffer);
            self.gl.delete_buffer(self.vertex_buffer);
            self.gl.delete_vertex_array(self.vertex_array);
            self.gl.delete_program(self.program);
            self.gl.delete_shader(self.fragment_shader);
            self.gl.delete_shader(self.vertex_shader);
        }
    }
}

pub struct VideoPlayer {
    audio_volume: Arc<RwLock<Option<gst::Element>>>,
    pipeline: Arc<RwLock<Option<gst::Pipeline>>>,
    state: Arc<RwLock<gst::State>>,

    video_frame: Arc<RwLock<VideoFrame>>,
}

impl VideoPlayer {
    pub fn new(video_path: impl AsRef<Path>, gl: Arc<glow::Context>, ctx: &egui::Context) -> Self {
        let video_player = Self {
            audio_volume: Arc::new(RwLock::new(None)),
            pipeline: Arc::new(RwLock::new(None)),
            state: Arc::new(RwLock::new(gst::State::Playing)),
            video_frame: Arc::new(RwLock::new(VideoFrame::new(gl))),
        };

        let video_path = video_path.as_ref().to_path_buf();
        let audio_volume = video_player.audio_volume.clone();
        let pipeline = video_player.pipeline.clone();
        let state = video_player.state.clone();
        let video_frame = video_player.video_frame.clone();
        std::thread::spawn(move || {
            gst::init().expect("Cannot initialize gstream");

            let source_elements = SourceElements::new(video_path);
            let video_elements = VideoElements::new();
            let audio_elements = AudioElements::new();

            audio_volume.write().replace(audio_elements.volume.clone());
            pipeline.write().replace(gst::Pipeline::default());

            let bus = {
                let pipeline_guard = pipeline.read();
                let pipeline = pipeline_guard.as_ref().unwrap();
                source_elements.add_to_pipeline(pipeline);
                video_elements.add_to_pipeline(pipeline);
                audio_elements.add_to_pipeline(pipeline);

                video_elements.link();
                audio_elements.link();
                source_elements.link(video_elements.queue.clone(), audio_elements.queue);

                video_elements.set_sink_callback(video_frame, ctx);

                pipeline
                    .set_state(gst::State::Playing)
                    .expect("Unable to set the pipeline to the `Playing` state");

                pipeline.bus().unwrap()
            };

            for msg in bus.iter_timed(gst::ClockTime::NONE) {
                use gst::MessageView;

                match msg.view() {
                    MessageView::Error(err) => {
                        eprintln!(
                            "Error received from element {:?} {}",
                            err.src().map(|s| s.path_string()),
                            err.error()
                        );
                        eprintln!("Debugging information: {:?}", err.debug());
                        break;
                    }
                    MessageView::StateChanged(state_changed) => {
                        if state_changed
                            .src()
                            .map(|s| {
                                s == pipeline
                                    .read()
                                    .as_ref()
                                    .expect("pipeline should be available during message handling")
                            })
                            .unwrap_or(false)
                        {
                            *state.write() = state_changed.current();
                        }
                    }
                    MessageView::Eos(..) => break,
                    _ => (),
                }
            }

            audio_volume.write().take();
            pipeline
                .read()
                .as_ref()
                .expect("pipeline has not been taken yet")
                .set_state(gst::State::Null)
                .expect("Unable to set the pipeline to the `Null` state");
            pipeline.write().take();
            *state.write() = gst::State::Null;
        });

        video_player
    }

    pub fn update(&mut self, ui: &mut egui::Ui, _ctx: &egui::Context) {
        egui::Frame::canvas(ui.style()).show(ui, |ui| {
            let max_size = {
                let video_frame_guard = self.video_frame.read();
                if video_frame_guard.width > 0 && video_frame_guard.height > 0 {
                    scale_fit_all(
                        Vec2::new(
                            ui.available_width(),
                            ui.available_height() - CONTROLLER_HEIGHT,
                        ),
                        Vec2::new(
                            video_frame_guard.width as f32,
                            video_frame_guard.height as f32,
                        ),
                    )
                } else {
                    ui.available_size()
                }
            };
            let (rect, response) = ui.allocate_exact_size(max_size, egui::Sense::click());
            if response.clicked() {
                if self.is_paused() {
                    let _ = self.resume();
                } else {
                    let _ = self.pause();
                }
            }
            let video_frame = self.video_frame.clone();
            let callback = egui::PaintCallback {
                callback: Arc::new(egui_glow::CallbackFn::new(move |_info, _painter| {
                    video_frame.read().paint();
                })),
                rect,
            };
            ui.painter().add(callback);
        });
    }

    pub fn is_paused(&self) -> bool {
        *self.state.read() == gst::State::Paused
    }

    fn is_end(&self) -> bool {
        *self.state.read() == gst::State::Null
    }

    pub fn position(&self) -> u32 {
        if let Some(pipeline) = self.pipeline.read().as_ref() {
            pipeline
                .query_position::<gst::ClockTime>()
                .map(|c| c.seconds())
                .unwrap_or(0) as u32
        } else {
            0
        }
    }

    pub fn duration(&self) -> u32 {
        if let Some(pipeline) = self.pipeline.read().as_ref() {
            pipeline
                .query_duration::<gst::ClockTime>()
                .map(|c| c.seconds())
                .unwrap_or(0) as u32
        } else {
            0
        }
    }

    pub fn volume(&self) -> u8 {
        if let Some(audio_volume) = self.audio_volume.read().as_ref() {
            (audio_volume.property::<f64>("volume").max(0.0) * 100.0).min(u8::MAX as f64) as u8
        } else {
            0
        }
    }

    fn start(&mut self) -> Result<()> {
        if let Some(pipeline) = self.pipeline.read().as_ref() {
            pipeline.set_state(gst::State::Playing)?;
        }
        Ok(())
    }

    pub fn resume(&mut self) -> Result<()> {
        if let Some(pipeline) = self.pipeline.read().as_ref() {
            pipeline.set_state(gst::State::Playing)?;
        }
        Ok(())
    }

    pub fn pause(&mut self) -> Result<()> {
        if let Some(pipeline) = self.pipeline.read().as_ref() {
            pipeline.set_state(gst::State::Paused)?;
        }
        Ok(())
    }

    fn stop(&mut self) -> Result<()> {
        if let Some(pipeline) = self.pipeline.read().as_ref() {
            pipeline.set_state(gst::State::Null)?;
        }
        Ok(())
    }

    pub fn set_volume(&mut self, percent: u8) -> Result<()> {
        if let Some(audio_volume) = self.audio_volume.read().as_ref() {
            audio_volume.set_property("volume", percent as f64 / 100.0);
        }
        Ok(())
    }

    pub fn seek(&mut self, seconds: u32) -> Result<()> {
        if let Some(pipeline) = self.pipeline.read().as_ref() {
            pipeline.seek_simple(
                gst::SeekFlags::FLUSH | gst::SeekFlags::KEY_UNIT,
                seconds as u64 * gst::ClockTime::SECOND,
            )?;
        }
        Ok(())
    }

    pub fn fast_forward(&mut self, seconds: u32) -> Result<()> {
        if let Some(pipeline) = self.pipeline.read().as_ref() {
            pipeline.seek_simple(
                gst::SeekFlags::FLUSH | gst::SeekFlags::KEY_UNIT,
                (pipeline
                    .query_position::<gst::ClockTime>()
                    .map(|c| c.seconds())
                    .unwrap_or(0)
                    + seconds as u64)
                    * gst::ClockTime::SECOND,
            )?;
        }
        Ok(())
    }

    pub fn rewind(&mut self, seconds: u32) -> Result<()> {
        if let Some(pipeline) = self.pipeline.read().as_ref() {
            pipeline.seek_simple(
                gst::SeekFlags::FLUSH | gst::SeekFlags::KEY_UNIT,
                (pipeline
                    .query_position::<gst::ClockTime>()
                    .map(|c| c.seconds())
                    .unwrap_or(0)
                    .max(seconds as u64)
                    - seconds as u64)
                    * gst::ClockTime::SECOND,
            )?;
        }
        Ok(())
    }
}

impl Drop for VideoPlayer {
    fn drop(&mut self) {
        if let Some(pipeline) = self.pipeline.read().as_ref() {
            let _ = pipeline.set_state(gst::State::Null);
        }
    }
}

use std::path::Path;

use anyhow::Result;
use eframe::egui;
use gst::prelude::*;
use gstreamer as gst;

struct SourceElements {
    source: gst::Element,
    demux: gst::Element,
}

impl SourceElements {
    fn new(video_path: impl AsRef<Path>) -> Self {
        Self {
            source: gst::ElementFactory::make("filesrc")
                .property("location", video_path.as_ref().display().to_string())
                .build()
                .expect("Cannot build source"),
            demux: gst::ElementFactory::make("qtdemux")
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

    fn link(self, video_queue: gst::Element, audio_queue: gst::Element) -> gst::Element {
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

        self.source
    }
}

struct VideoElements {
    queue: gst::Element,
    decode: gst::Element,
    convert: gst::Element,
    scale: gst::Element,
    sink: gst::Element,
}

impl VideoElements {
    fn new() -> Self {
        Self {
            queue: gst::ElementFactory::make("queue")
                .build()
                .expect("Cannot build queue"),
            decode: gst::ElementFactory::make("decodebin")
                .build()
                .expect("Cannot build decodebin"),
            convert: gst::ElementFactory::make("videoconvert")
                .build()
                .expect("Cannot build videoconvert"),
            scale: gst::ElementFactory::make("videoscale")
                .build()
                .expect("Cannot build videoscale"),
            sink: gst::ElementFactory::make("fakesink")
                .build()
                .expect("Cannot build video sink"),
        }
    }

    fn add_to_pipeline(&self, pipeline: &gst::Pipeline) {
        pipeline
            .add_many(&[
                &self.queue,
                &self.decode,
                &self.convert,
                &self.scale,
                &self.sink,
            ])
            .expect("Cannot add video elements to pipeline");
    }

    fn link(self) -> gst::Element {
        gst::Element::link_many(&[&self.queue, &self.decode]).expect("Cannot link video elements");
        gst::Element::link_many(&[&self.convert, &self.scale, &self.sink])
            .expect("Cannot link video elements");

        self.decode.connect_pad_added(move |_src, src_pad| {
            let convert_sink = self
                .convert
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

        self.queue
    }
}

struct AudioElements {
    queue: gst::Element,
    decode: gst::Element,
    convert: gst::Element,
    resample: gst::Element,
    sink: gst::Element,
}

impl AudioElements {
    fn new() -> Self {
        Self {
            queue: gst::ElementFactory::make("queue")
                .build()
                .expect("Cannot build queue"),
            decode: gst::ElementFactory::make("decodebin")
                .build()
                .expect("Cannot build decodebin"),
            convert: gst::ElementFactory::make("audioconvert")
                .build()
                .expect("Cannot build audioconvert"),
            resample: gst::ElementFactory::make("audioresample")
                .build()
                .expect("Cannot build audioresample"),
            sink: gst::ElementFactory::make("autoaudiosink")
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
                &self.sink,
            ])
            .expect("Cannot add audio elements to pipeline");
    }

    fn link(self) -> gst::Element {
        gst::Element::link_many(&[&self.queue, &self.decode]).expect("Cannot link audio elements");
        gst::Element::link_many(&[&self.convert, &self.resample, &self.sink])
            .expect("Cannot link audio elements");

        self.decode.connect_pad_added(move |_src, src_pad| {
            let convert_sink = self
                .convert
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

        self.queue
    }
}

pub struct VideoPlayer {
    pipeline: gst::Pipeline,
}

impl VideoPlayer {
    pub fn new(video_path: impl AsRef<Path>) -> Self {
        gst::init().expect("Cannot initialize gstream");

        let source_elements = SourceElements::new(video_path);
        let video_elements = VideoElements::new();
        let audio_elements = AudioElements::new();

        let pipeline = gst::Pipeline::default();

        source_elements.add_to_pipeline(&pipeline);
        video_elements.add_to_pipeline(&pipeline);
        audio_elements.add_to_pipeline(&pipeline);

        let video_queue = video_elements.link();
        let audio_queue = audio_elements.link();
        let _source = source_elements.link(video_queue, audio_queue);

        Self { pipeline }
    }
}

impl Drop for VideoPlayer {
    fn drop(&mut self) {
        let _ = self.pipeline.set_state(gst::State::Null);
    }
}

impl super::VideoPlayer for VideoPlayer {
    fn is_paused(&self) -> bool {
        self.pipeline.current_state() == gst::State::Paused
    }

    fn is_end(&self) -> bool {
        self.pipeline.current_state() == gst::State::Null
    }

    fn show(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {}

    fn start(&mut self) -> Result<()> {
        self.pipeline.set_state(gst::State::Playing)?;
        Ok(())
    }

    fn resume(&mut self) -> Result<()> {
        self.pipeline.set_state(gst::State::Playing)?;
        Ok(())
    }

    fn pause(&mut self) -> Result<()> {
        self.pipeline.set_state(gst::State::Paused)?;
        Ok(())
    }

    fn stop(&mut self) -> Result<()> {
        self.pipeline.set_state(gst::State::Null)?;
        Ok(())
    }

    fn fast_forward(&mut self, seconds: u32) -> Result<()> {
        Ok(())
    }

    fn rewind(&mut self, seconds: u32) -> Result<()> {
        Ok(())
    }
}

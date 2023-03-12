use eframe::egui;

use crate::helper::scale_fit_all;

pub struct MediaPlayer {
    texture: Option<egui::TextureHandle>,
}

impl MediaPlayer {
    pub fn new() -> Self {
        Self { texture: None }
    }
}

impl super::MediaPlayer for MediaPlayer {
    fn is_loaded(&self) -> bool {
        self.texture.is_some()
    }

    fn is_end(&self) -> bool {
        true
    }

    fn support_extensions(&self) -> &[&str] {
        &["bmp", "gif", "jpeg", "jpg", "png"]
    }

    fn reload(&mut self, path: &dyn AsRef<std::path::Path>, ctx: &egui::Context) {
        let image = image::io::Reader::open(path)
            .expect("Cannot open image file")
            .with_guessed_format()
            .expect("Cannot guess image mime type")
            .decode()
            .expect("Cannot decode image file")
            .to_rgba8();
        self.texture.replace(ctx.load_texture(
            "current_image",
            egui::ColorImage::from_rgba_unmultiplied(
                [image.width() as _, image.height() as _],
                image.as_flat_samples().as_slice(),
            ),
            Default::default(),
        ));
    }

    fn show_central_panel(&mut self, ui: &mut egui::Ui, _ctx: &egui::Context, _can_input: bool) {
        if let Some(texture) = self.texture.as_ref() {
            ui.centered_and_justified(|ui| {
                ui.image(
                    texture,
                    scale_fit_all(ui.available_size(), texture.size_vec2()),
                );
            });
        }
    }
}

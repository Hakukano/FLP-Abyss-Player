use eframe::{egui, epaint::Vec2};

pub struct MediaPlayer {
    support_extensions: Vec<String>,

    image: Option<image::RgbaImage>,
    texture: Option<egui::TextureHandle>,
}

impl MediaPlayer {
    pub fn new() -> Self {
        Self {
            support_extensions: vec![
                "bmp".to_string(),
                "gif".to_string(),
                "jpeg".to_string(),
                "jpg".to_string(),
                "png".to_string(),
            ],
            image: None,
            texture: None,
        }
    }
}

impl super::MediaPlayer for MediaPlayer {
    fn loaded(&self) -> bool {
        self.image.is_some() && self.texture.is_some()
    }

    fn support_extensions(&self) -> &[String] {
        self.support_extensions.as_slice()
    }

    fn reload(&mut self, path: &dyn AsRef<std::path::Path>, ctx: &egui::Context) {
        self.image.replace(
            image::io::Reader::open(path)
                .expect("Cannot open image file")
                .with_guessed_format()
                .expect("Cannot guess image mime type")
                .decode()
                .expect("Cannot decode image file")
                .to_rgba8(),
        );
        let image = self
            .image
            .as_ref()
            .expect("Impossible: self.image was just replaced");
        self.texture.replace(ctx.load_texture(
            "current_image",
            egui::ColorImage::from_rgba_unmultiplied(
                [image.width() as _, image.height() as _],
                image.as_flat_samples().as_slice(),
            ),
            Default::default(),
        ));
    }

    fn show_central_panel(&mut self, ui: &mut egui::Ui, _ctx: &egui::Context) {
        if let Some(texture) = self.texture.as_ref() {
            let texture_size = texture.size_vec2();
            let ui_size = ui.available_size();
            let scale_x = ui_size.x / texture_size.x;
            let scale_y = ui_size.y / texture_size.y;
            let scale = scale_x.min(scale_y);
            ui.centered_and_justified(|ui| {
                ui.image(
                    texture,
                    Vec2::new(texture_size.x * scale, texture_size.y * scale),
                );
            });
        }
    }
}

use eframe::egui::{self, Image, ImageSource};

use crate::{models::player::Player, utils::helper::scale_fit_all};

pub struct MediaPlayer {
    texture: egui::TextureHandle,
}

impl MediaPlayer {
    pub fn new(player: &Player, ctx: &egui::Context) -> Self {
        let image = image::io::Reader::open(player.current_path())
            .expect("Cannot open image file")
            .with_guessed_format()
            .expect("Cannot guess image mime type")
            .decode()
            .expect("Cannot decode image file")
            .to_rgba8();
        let texture = ctx.load_texture(
            "current_image",
            egui::ColorImage::from_rgba_unmultiplied(
                [image.width() as _, image.height() as _],
                image.as_flat_samples().as_slice(),
            ),
            Default::default(),
        );
        Self { texture }
    }

    pub fn update(&mut self, ui: &mut egui::Ui) {
        ui.centered_and_justified(|ui| {
            ui.add(
                Image::new(ImageSource::Texture((&self.texture).into())).fit_to_exact_size(
                    scale_fit_all(ui.available_size(), self.texture.size_vec2()),
                ),
            );
        });
    }
}

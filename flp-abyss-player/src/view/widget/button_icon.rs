use std::path::Path;

use eframe::{
    egui,
    epaint::{ColorImage, TextureHandle, Vec2},
};
use image::RgbaImage;

use crate::library::helper::scale_fit_all;

/// All the icons should have same size for better look
pub struct ButtonIcon {
    texture: TextureHandle,
}

impl ButtonIcon {
    pub fn from_rgba_images(name: &str, image: &RgbaImage, ctx: &egui::Context) -> Self {
        Self {
            texture: ctx.load_texture(
                name,
                ColorImage::from_rgba_unmultiplied(
                    [image.width() as _, image.height() as _],
                    image.as_flat_samples().as_slice(),
                ),
                Default::default(),
            ),
        }
    }

    pub fn from_rgba_image_files(
        name: &str,
        image_path: impl AsRef<Path>,
        ctx: &egui::Context,
    ) -> Self {
        Self::from_rgba_images(
            name,
            &image::io::Reader::open(image_path)
                .expect("Cannot open image file")
                .with_guessed_format()
                .expect("Cannot guess image mime type")
                .decode()
                .expect("Cannot decode image file")
                .to_rgba8(),
            ctx,
        )
    }

    pub fn show(&self, max_size: Vec2, ui: &mut egui::Ui) -> egui::Response {
        ui.add(egui::ImageButton::new(
            self.texture.id(),
            scale_fit_all(max_size, self.texture.size_vec2()),
        ))
    }
}

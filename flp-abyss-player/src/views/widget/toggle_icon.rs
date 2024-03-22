use std::path::Path;

use eframe::{
    egui::{self, Image, ImageSource},
    epaint::{ColorImage, TextureHandle, Vec2},
};
use image::RgbaImage;

use crate::library::helper::scale_fit_all;

/// All the icons should have same size for better look
pub struct ToggleIcon {
    texture_on: TextureHandle,
    texture_off: TextureHandle,
}

impl ToggleIcon {
    pub fn from_rgba_images(
        name: &str,
        image_on: &RgbaImage,
        image_off: &RgbaImage,
        ctx: &egui::Context,
    ) -> Self {
        Self {
            texture_on: ctx.load_texture(
                format!("{name}_on"),
                ColorImage::from_rgba_unmultiplied(
                    [image_on.width() as _, image_on.height() as _],
                    image_on.as_flat_samples().as_slice(),
                ),
                Default::default(),
            ),
            texture_off: ctx.load_texture(
                format!("{name}_off"),
                ColorImage::from_rgba_unmultiplied(
                    [image_off.width() as _, image_off.height() as _],
                    image_off.as_flat_samples().as_slice(),
                ),
                Default::default(),
            ),
        }
    }

    pub fn from_rgba_image_files(
        name: &str,
        image_on_path: impl AsRef<Path>,
        image_off_path: impl AsRef<Path>,
        ctx: &egui::Context,
    ) -> Self {
        Self::from_rgba_images(
            name,
            &image::io::Reader::open(image_on_path)
                .expect("Cannot open image file")
                .with_guessed_format()
                .expect("Cannot guess image mime type")
                .decode()
                .expect("Cannot decode image file")
                .to_rgba8(),
            &image::io::Reader::open(image_off_path)
                .expect("Cannot open image file")
                .with_guessed_format()
                .expect("Cannot guess image mime type")
                .decode()
                .expect("Cannot decode image file")
                .to_rgba8(),
            ctx,
        )
    }

    pub fn show(&self, max_size: Vec2, ui: &mut egui::Ui, on: &mut bool) -> egui::Response {
        let mut response = if *on {
            ui.add(egui::ImageButton::new(
                Image::new(ImageSource::Texture((&self.texture_on).into()))
                    .fit_to_exact_size(scale_fit_all(max_size, self.texture_on.size_vec2())),
            ))
        } else {
            ui.add(egui::ImageButton::new(
                Image::new(ImageSource::Texture((&self.texture_off).into()))
                    .fit_to_exact_size(scale_fit_all(max_size, self.texture_off.size_vec2())),
            ))
        };

        if response.clicked() {
            *on = !*on;
            response.mark_changed();
        }

        response
    }
}

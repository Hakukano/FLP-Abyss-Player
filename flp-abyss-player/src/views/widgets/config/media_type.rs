use std::path::Path;

use eframe::{
    egui::{self, TextStyle::*},
    epaint::{Color32, Vec2},
};

use crate::{
    models::config::MediaType, utils::fonts::gen_rich_text,
    views::widgets::button_icon::ButtonIcon, CLI,
};

pub struct ConfigMediaType {
    checkmark: ButtonIcon,
}

impl ConfigMediaType {
    pub fn new(ctx: &egui::Context) -> Self {
        Self {
            checkmark: ButtonIcon::from_rgba_image_files(
                "media_type_checkmark",
                Path::new(CLI.assets_path.as_str())
                    .join("image")
                    .join("icon")
                    .join("checkmark.png"),
                ctx,
            ),
        }
    }

    pub fn show_config(&self, ui: &mut egui::Ui, ctx: &egui::Context, media_type: &mut MediaType) {
        egui::ComboBox::from_label(gen_rich_text(
            ctx,
            t!("ui.config.media_type.label"),
            Body,
            None,
        ))
        .selected_text(gen_rich_text(ctx, media_type.to_string(), Body, None))
        .show_ui(ui, |ui| {
            ui.selectable_value(
                media_type,
                MediaType::Unset,
                gen_rich_text(ctx, "--", Body, None),
            );
            ui.selectable_value(
                media_type,
                MediaType::Server,
                gen_rich_text(ctx, t!("ui.config.media_type.server"), Body, None),
            );
            ui.selectable_value(
                media_type,
                MediaType::Image,
                gen_rich_text(ctx, t!("ui.config.media_type.image"), Body, None),
            );
            ui.selectable_value(
                media_type,
                MediaType::Video,
                gen_rich_text(ctx, t!("ui.config.media_type.video"), Body, None),
            );
        });
    }

    pub fn show_hint(&self, ui: &mut egui::Ui, ctx: &egui::Context, media_type: &MediaType) {
        if media_type.is_unset() {
            ui.label(gen_rich_text(
                ctx,
                t!("ui.config.media_type.unset"),
                Body,
                Some(Color32::LIGHT_RED),
            ));
        } else {
            let max_height = ui.text_style_height(&Body);
            self.checkmark.show(Vec2::new(max_height, max_height), ui);
        }
    }
}

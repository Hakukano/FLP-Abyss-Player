use std::path::Path;

use eframe::{
    egui::{self, TextStyle::*},
    epaint::{Color32, Vec2},
};

use crate::{config::MediaType, font::gen_rich_text, locale, widget::button_icon::ButtonIcon, Cli};

pub struct ConfigMediaType {
    checkmark: ButtonIcon,
}

impl ConfigMediaType {
    pub fn new(ctx: &egui::Context, cli: &Cli) -> Self {
        Self {
            checkmark: ButtonIcon::from_rgba_image_files(
                "media_type_checkmark",
                Path::new(cli.assets_path.as_str())
                    .join("image")
                    .join("icon")
                    .join("checkmark.png"),
                ctx,
            ),
        }
    }

    pub fn show_config(
        &self,
        ui: &mut egui::Ui,
        ctx: &egui::Context,
        locale: &locale::ui::Config,
        media_type: &mut MediaType,
    ) {
        egui::ComboBox::from_label(gen_rich_text(
            ctx,
            locale.media_type.label.as_str(),
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
                gen_rich_text(ctx, locale.media_type.server.as_str(), Body, None),
            );
            ui.selectable_value(
                media_type,
                MediaType::Image,
                gen_rich_text(ctx, locale.media_type.image.as_str(), Body, None),
            );
            ui.selectable_value(
                media_type,
                MediaType::Video,
                gen_rich_text(ctx, locale.media_type.video.as_str(), Body, None),
            );
        });
    }

    pub fn show_hint(
        &self,
        ui: &mut egui::Ui,
        ctx: &egui::Context,
        locale: &locale::ui::Config,
        media_type: &MediaType,
    ) {
        if media_type.is_unset() {
            ui.label(gen_rich_text(
                ctx,
                locale.media_type.unset.as_str(),
                Body,
                Some(Color32::LIGHT_RED),
            ));
        } else {
            let max_height = ui.text_style_height(&Body);
            self.checkmark.show(Vec2::new(max_height, max_height), ui);
        }
    }
}

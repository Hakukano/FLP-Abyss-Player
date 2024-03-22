use std::path::Path;

use eframe::{
    egui::{self, TextStyle::*},
    epaint::{Color32, Vec2},
};

use crate::{config::Config, font::gen_rich_text, locale, widget::button_icon::ButtonIcon, Cli};

pub struct ConfigRootPath {
    checkmark: ButtonIcon,
}

impl ConfigRootPath {
    pub fn new(ctx: &egui::Context, cli: &Cli) -> Self {
        Self {
            checkmark: ButtonIcon::from_rgba_image_files(
                "root_path_checkmark",
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
        config: &mut Config,
    ) {
        if ui
            .button(gen_rich_text(
                ctx,
                locale.root_path.label.as_str(),
                Body,
                None,
            ))
            .clicked()
        {
            if let Some(path) = rfd::FileDialog::new().pick_folder() {
                config.root_path.replace(path.display().to_string());
            }
        }
    }

    pub fn show_hint(
        &self,
        ui: &mut egui::Ui,
        ctx: &egui::Context,
        locale: &locale::ui::Config,
        config: &Config,
    ) {
        if let Some(root_path) = &config.root_path {
            let max_height = ui.text_style_height(&Body);
            self.checkmark.show(Vec2::new(max_height, max_height), ui);
            ui.label(gen_rich_text(
                ctx,
                format!("{}: {root_path}", locale.root_path.set.as_str()),
                Body,
                None,
            ));
        } else {
            ui.label(gen_rich_text(
                ctx,
                locale.root_path.unset.as_str(),
                Body,
                Some(Color32::RED),
            ));
        }
    }
}

use std::path::Path;

use eframe::{
    egui::{self, TextStyle::*},
    epaint::{Color32, Vec2},
};

use crate::{
    config::VideoPlayer, font::gen_rich_text, locale, widget::button_icon::ButtonIcon, Cli,
};

pub struct ConfigVideoPlayer {
    checkmark: ButtonIcon,
}

impl ConfigVideoPlayer {
    pub fn new(ctx: &egui::Context, cli: &Cli) -> Self {
        Self {
            checkmark: ButtonIcon::from_rgba_image_files(
                "video_player_checkmark",
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
        video_player: &mut VideoPlayer,
    ) {
        egui::ComboBox::from_label(gen_rich_text(
            ctx,
            locale.video_player.label.as_str(),
            Body,
            None,
        ))
        .selected_text(gen_rich_text(ctx, video_player.to_string(), Body, None))
        .show_ui(ui, |ui| {
            ui.selectable_value(
                video_player,
                VideoPlayer::Unset,
                gen_rich_text(ctx, "--", Body, None),
            );
            #[cfg(feature = "native")]
            ui.selectable_value(
                video_player,
                VideoPlayer::Native,
                gen_rich_text(ctx, locale.video_player.native.as_str(), Body, None),
            );
            ui.selectable_value(
                video_player,
                VideoPlayer::Vlc,
                gen_rich_text(ctx, locale.video_player.vlc.as_str(), Body, None),
            );
        });
    }

    pub fn show_hint(
        &self,
        ui: &mut egui::Ui,
        ctx: &egui::Context,
        locale: &locale::ui::Config,
        video_player: &VideoPlayer,
    ) {
        if video_player.is_unset() {
            ui.label(gen_rich_text(
                ctx,
                locale.video_player.unset.as_str(),
                Body,
                Some(Color32::LIGHT_RED),
            ));
        } else {
            let max_height = ui.text_style_height(&Body);
            self.checkmark.show(Vec2::new(max_height, max_height), ui);
        }
    }
}

#![allow(clippy::too_many_arguments)]

use std::path::{Path, PathBuf};

use chrono::{DateTime, Local};
use eframe::{
    egui::{self, Layout, TextStyle},
    emath::Align,
    epaint::{Color32, Vec2},
};

#[cfg(feature = "native")]
use crate::models::config::VideoPlayer;
use crate::{
    library,
    library::fonts::gen_rich_text,
    models::{config::MediaType, player::Player},
    CLI,
};

use super::{
    button_icon::ButtonIcon,
    config::{video_player::ConfigVideoPlayer, video_player_path::ConfigVideoPlayerPath},
};

pub struct Playlist {
    search_icon: ButtonIcon,
    add_one_icon: ButtonIcon,
    add_many_icon: ButtonIcon,
    up_icon: ButtonIcon,
    down_icon: ButtonIcon,
    remove_icon: ButtonIcon,
    save_icon: ButtonIcon,
    load_icon: ButtonIcon,

    config_video_player: ConfigVideoPlayer,
    config_video_player_path: ConfigVideoPlayerPath,
}

impl Playlist {
    pub fn new(ctx: &egui::Context) -> Self {
        let icon_path = Path::new(CLI.assets_path.as_str())
            .join("image")
            .join("icon");
        Self {
            search_icon: ButtonIcon::from_rgba_image_files(
                "search",
                icon_path.join("search.png"),
                ctx,
            ),
            add_one_icon: ButtonIcon::from_rgba_image_files(
                "add_one",
                icon_path.join("add_one.png"),
                ctx,
            ),
            add_many_icon: ButtonIcon::from_rgba_image_files(
                "add_many",
                icon_path.join("add_many.png"),
                ctx,
            ),
            up_icon: ButtonIcon::from_rgba_image_files("up", icon_path.join("up.png"), ctx),
            down_icon: ButtonIcon::from_rgba_image_files("down", icon_path.join("down.png"), ctx),
            remove_icon: ButtonIcon::from_rgba_image_files(
                "remove",
                icon_path.join("remove.png"),
                ctx,
            ),
            save_icon: ButtonIcon::from_rgba_image_files("save", icon_path.join("save.png"), ctx),
            load_icon: ButtonIcon::from_rgba_image_files("load", icon_path.join("load.png"), ctx),

            config_video_player: ConfigVideoPlayer::new(ctx),
            config_video_player_path: ConfigVideoPlayerPath::new(ctx),
        }
    }

    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        ctx: &egui::Context,
        player_buffer: &mut Player,
        search: &mut bool,
        search_str: &mut String,
        filtered_paths: &[(usize, String)],
        save: &mut Option<PathBuf>,
        load: &mut Option<PathBuf>,
    ) {
        ui.with_layout(Layout::top_down(Align::TOP), |ui| {
            ui.group(|ui| {
                ui.with_layout(
                    Layout::left_to_right(Align::TOP).with_cross_justify(true),
                    |ui| {
                        let max_height = 20.0;
                        ui.set_height(max_height);
                        let response = ui.text_edit_singleline(search_str);
                        if response.lost_focus()
                            && response.ctx.input(|i| i.key_pressed(egui::Key::Enter))
                        {
                            *search = true;
                        }
                        if self
                            .search_icon
                            .show(Vec2::new(max_height, max_height), ui)
                            .clicked()
                        {
                            *search = true;
                        }

                        ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                            if self
                                .add_many_icon
                                .show(Vec2::new(max_height, max_height), ui)
                                .clicked()
                            {
                                if let Some(paths) = rfd::FileDialog::new().pick_folders() {
                                    for path in paths.into_iter() {
                                        player_buffer.playlist.body.item_paths.append(
                                            &mut player_buffer
                                                .playlist
                                                .header
                                                .media_type
                                                .find_all_paths(&path)
                                                .into_iter()
                                                .map(|p| p.to_str().unwrap().to_string())
                                                .collect(),
                                        );
                                    }
                                }
                            }

                            if self
                                .add_one_icon
                                .show(Vec2::new(max_height, max_height), ui)
                                .clicked()
                            {
                                if let Some(paths) = rfd::FileDialog::new()
                                    .add_filter(
                                        "Media",
                                        player_buffer
                                            .playlist
                                            .header
                                            .media_type
                                            .supported_extensions(),
                                    )
                                    .pick_files()
                                {
                                    player_buffer.playlist.body.item_paths.append(
                                        &mut paths
                                            .into_iter()
                                            .map(|p| p.to_str().unwrap().to_string())
                                            .collect(),
                                    );
                                }
                            }
                        })
                    },
                );
            });

            ui.add_space(5.0);
            ui.group(|ui| {
                let text_style = TextStyle::Small;
                let max_height = ui.text_style_height(&text_style);
                ui.spacing_mut().interact_size.y = max_height;
                egui::ScrollArea::vertical()
                    .max_height(30.0 * max_height)
                    .show_rows(ui, max_height, filtered_paths.len(), |ui, row_range| {
                        for row in row_range {
                            ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                                let (index, path) =
                                    filtered_paths.get(row).expect("Out of range: paths");
                                if ui
                                    .button(gen_rich_text(
                                        ctx,
                                        path,
                                        text_style.clone(),
                                        if *index == player_buffer.index {
                                            Some(Color32::LIGHT_GREEN)
                                        } else {
                                            None
                                        },
                                    ))
                                    .clicked()
                                {
                                    player_buffer.index = *index;
                                }

                                ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                                    ui.spacing_mut().item_spacing.x = 5.0;
                                    if self
                                        .remove_icon
                                        .show(Vec2::new(max_height, max_height), ui)
                                        .clicked()
                                    {
                                        if *index != player_buffer.index {
                                            player_buffer.playlist.body.item_paths.remove(*index);
                                        }
                                        if *index < player_buffer.index {
                                            player_buffer.index -= 1;
                                        }
                                    }
                                    if self
                                        .down_icon
                                        .show(Vec2::new(max_height, max_height), ui)
                                        .clicked()
                                        && *index < player_buffer.playlist.body.item_paths.len() - 1
                                    {
                                        player_buffer
                                            .playlist
                                            .body
                                            .item_paths
                                            .swap(*index, *index + 1);
                                        if *index == player_buffer.index {
                                            player_buffer.index += 1;
                                        } else if *index + 1 == player_buffer.index {
                                            player_buffer.index -= 1;
                                        }
                                    }
                                    if self
                                        .up_icon
                                        .show(Vec2::new(max_height, max_height), ui)
                                        .clicked()
                                        && *index > 0
                                    {
                                        player_buffer
                                            .playlist
                                            .body
                                            .item_paths
                                            .swap(*index, *index - 1);
                                        if *index == player_buffer.index {
                                            player_buffer.index -= 1;
                                        } else if *index - 1 == player_buffer.index {
                                            player_buffer.index += 1;
                                        }
                                    }
                                });
                            });
                        }
                    });
            });

            ui.add_space(5.0);
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    let max_height = 20.0;
                    ui.set_height(max_height);

                    if self
                        .save_icon
                        .show(Vec2::new(max_height, max_height), ui)
                        .clicked()
                    {
                        if let Some(path) = rfd::FileDialog::new()
                            .set_title("SAVE")
                            .add_filter(
                                library::playlist::EXTENSION,
                                &[library::playlist::EXTENSION],
                            )
                            .save_file()
                        {
                            save.replace(path);
                        }
                    }

                    ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                        if self
                            .load_icon
                            .show(Vec2::new(max_height, max_height), ui)
                            .clicked()
                        {
                            if let Some(path) = rfd::FileDialog::new()
                                .set_title("LOAD")
                                .add_filter(
                                    library::playlist::EXTENSION,
                                    &[library::playlist::EXTENSION],
                                )
                                .pick_file()
                            {
                                load.replace(path);
                            }
                        }
                    });
                });
            });

            let header = &mut player_buffer.playlist.header;
            ui.add_space(5.0);
            ui.group(|ui| {
                egui::Grid::new("header")
                    .spacing(Vec2::new(8.0, 8.0))
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label(gen_rich_text(
                            ctx,
                            header.version.to_string(),
                            TextStyle::Body,
                            None,
                        ));
                        ui.label(gen_rich_text(
                            ctx,
                            DateTime::<Local>::from(header.time).to_rfc3339(),
                            TextStyle::Body,
                            None,
                        ));
                        ui.label("");
                        ui.end_row();

                        ui.label(gen_rich_text(
                            ctx,
                            t!("ui.config.media_type.label"),
                            TextStyle::Body,
                            None,
                        ));
                        ui.label(gen_rich_text(
                            ctx,
                            header.media_type.to_string(),
                            TextStyle::Body,
                            None,
                        ));
                        ui.label("");
                        ui.end_row();

                        if header.media_type == MediaType::Video {
                            ui.label(gen_rich_text(
                                ctx,
                                t!("ui.config.video_player.label"),
                                TextStyle::Body,
                                None,
                            ));
                            self.config_video_player
                                .show_config(ui, ctx, &mut header.video_player);
                            self.config_video_player
                                .show_hint(ui, ctx, &header.video_player);
                            ui.end_row();

                            match header.video_player {
                                #[cfg(feature = "native")]
                                VideoPlayer::Native => {}
                                _ => {
                                    ui.label(gen_rich_text(
                                        ctx,
                                        t!("ui.config.video_player_path.label"),
                                        TextStyle::Body,
                                        None,
                                    ));
                                    self.config_video_player_path.show_config(
                                        ui,
                                        ctx,
                                        &mut header.video_player_path,
                                    );
                                    self.config_video_player_path.show_hint(
                                        ui,
                                        ctx,
                                        &header.video_player_path,
                                    );
                                    ui.end_row();
                                }
                            }
                        }
                    });
            });
        });
    }
}

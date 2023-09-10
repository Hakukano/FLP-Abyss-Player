#![allow(clippy::too_many_arguments)]

use std::path::{Path, PathBuf};

use chrono::{DateTime, Local};
use eframe::{
    egui::{self, Layout, TextStyle},
    emath::Align,
    epaint::{Color32, Vec2},
};
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};

#[cfg(feature = "native")]
use crate::model::config::VideoPlayer;
use crate::{
    app::view::MediaPlayer,
    library::{
        fonts::gen_rich_text,
        helper::message_dialog_error,
        playlist::{self, Body, Header},
    },
    model::config::{Config, MediaType},
    CLI,
};

use super::{
    button_icon::ButtonIcon,
    config::{video_player::ConfigVideoPlayer, video_player_path::ConfigVideoPlayerPath},
};

pub struct PlaylistState {
    header: Option<Header>,
    paths: Vec<PathBuf>,
    paths_changed: bool,
    index: usize,
    index_changed: bool,
    search_string: String,
    search: bool,
}

impl PlaylistState {
    pub fn new(header: Option<Header>, paths: Vec<PathBuf>, index: usize) -> Self {
        Self {
            header,
            paths,
            paths_changed: false,
            index,
            index_changed: false,
            search_string: String::new(),
            search: false,
        }
    }

    fn can_change(&self) -> bool {
        !(self.paths_changed || self.index_changed)
    }

    pub fn set_index(&mut self, index: usize) {
        self.index = index;
    }

    pub fn consume_paths_change(&mut self) -> Option<&[PathBuf]> {
        if self.paths_changed {
            self.paths_changed = false;
            Some(self.paths.as_slice())
        } else {
            None
        }
    }

    pub fn consume_index_change(&mut self) -> Option<usize> {
        if self.index_changed {
            self.index_changed = false;
            Some(self.index)
        } else {
            None
        }
    }
}

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
        state: &mut PlaylistState,
        ui: &mut egui::Ui,
        ctx: &egui::Context,
        current_index: Option<usize>,
        media_player: &dyn MediaPlayer,
    ) {
        ui.with_layout(Layout::top_down(Align::TOP), |ui| {
            ui.group(|ui| {
                ui.with_layout(
                    Layout::left_to_right(Align::TOP).with_cross_justify(true),
                    |ui| {
                        let max_height = 20.0;
                        ui.set_height(max_height);
                        let response = ui.text_edit_singleline(&mut state.search_string);
                        if response.lost_focus()
                            && response.ctx.input(|i| i.key_pressed(egui::Key::Enter))
                        {
                            if state.search_string.trim().is_empty() {
                                state.search = false;
                            } else {
                                state.search = true;
                            }
                        }
                        if self
                            .search_icon
                            .show(Vec2::new(max_height, max_height), ui)
                            .clicked()
                        {
                            if state.search_string.trim().is_empty() {
                                state.search = false;
                            } else {
                                state.search = true;
                            }
                        }

                        ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                            if self
                                .add_many_icon
                                .show(Vec2::new(max_height, max_height), ui)
                                .clicked()
                                && state.can_change()
                            {
                                if let Some(paths) = rfd::FileDialog::new().pick_folders() {
                                    for path in paths.into_iter() {
                                        state
                                            .paths
                                            .append(&mut media_player.get_all_matched_paths(&path));
                                    }
                                    state.paths_changed = true;
                                }
                            }

                            if self
                                .add_one_icon
                                .show(Vec2::new(max_height, max_height), ui)
                                .clicked()
                                && state.can_change()
                            {
                                if let Some(mut paths) = rfd::FileDialog::new()
                                    .add_filter("Media", media_player.support_extensions())
                                    .pick_files()
                                {
                                    state.paths.append(&mut paths);
                                    state.paths_changed = true;
                                }
                            }
                        })
                    },
                );
            });

            let matcher = SkimMatcherV2::default();
            let paths: Vec<(usize, String)> = state
                .paths
                .iter()
                .enumerate()
                .filter_map(|(i, p)| {
                    let p = p.to_str().expect("Invalid path").to_string();
                    if state.search {
                        matcher
                            .fuzzy_match(p.as_str(), state.search_string.as_str())
                            .map(|_| (i, p))
                    } else {
                        Some((i, p))
                    }
                })
                .collect();
            ui.add_space(5.0);
            ui.group(|ui| {
                let text_style = TextStyle::Small;
                let max_height = ui.text_style_height(&text_style);
                ui.spacing_mut().interact_size.y = max_height;
                egui::ScrollArea::vertical()
                    .max_height(30.0 * max_height)
                    .show_rows(ui, max_height, paths.len(), |ui, row_range| {
                        for row in row_range {
                            ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                                let (index, path) = paths.get(row).expect("Out of range: paths");
                                if ui
                                    .button(gen_rich_text(
                                        ctx,
                                        path,
                                        text_style.clone(),
                                        if Some(*index) == current_index {
                                            Some(Color32::LIGHT_GREEN)
                                        } else {
                                            None
                                        },
                                    ))
                                    .clicked()
                                    && state.can_change()
                                {
                                    state.index_changed = true;
                                    state.index = *index;
                                }

                                ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                                    ui.spacing_mut().item_spacing.x = 5.0;
                                    if self
                                        .remove_icon
                                        .show(Vec2::new(max_height, max_height), ui)
                                        .clicked()
                                        && state.can_change()
                                    {
                                        if let Some(current_index) = current_index {
                                            if *index != current_index {
                                                state.paths.remove(*index);
                                                state.paths_changed = true;
                                            }
                                            if *index < current_index {
                                                state.index -= 1;
                                                state.index_changed = true;
                                            }
                                        } else {
                                            state.paths.remove(*index);
                                            state.paths_changed = true;
                                        }
                                    }
                                    if self
                                        .down_icon
                                        .show(Vec2::new(max_height, max_height), ui)
                                        .clicked()
                                        && state.can_change()
                                        && *index < state.paths.len() - 1
                                    {
                                        if let Some(current_index) = current_index {
                                            state.paths.swap(*index, *index + 1);
                                            state.paths_changed = true;
                                            if *index == current_index {
                                                state.index += 1;
                                                state.index_changed = true;
                                            } else if *index + 1 == current_index {
                                                state.index -= 1;
                                                state.index_changed = true;
                                            }
                                        } else {
                                            state.paths.swap(*index, *index + 1);
                                            state.paths_changed = true;
                                        }
                                    }
                                    if self
                                        .up_icon
                                        .show(Vec2::new(max_height, max_height), ui)
                                        .clicked()
                                        && state.can_change()
                                        && *index > 0
                                    {
                                        if let Some(current_index) = current_index {
                                            state.paths.swap(*index, *index - 1);
                                            state.paths_changed = true;
                                            if *index == current_index {
                                                state.index -= 1;
                                                state.index_changed = true;
                                            } else if *index - 1 == current_index {
                                                state.index += 1;
                                                state.index_changed = true;
                                            }
                                        } else {
                                            state.paths.swap(*index, *index - 1);
                                            state.paths_changed = true;
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
                        && state.can_change()
                    {
                        if let Some(path) = rfd::FileDialog::new()
                            .set_title("SAVE")
                            .add_filter(playlist::EXTENSION, &[playlist::EXTENSION])
                            .save_file()
                        {
                            let mut new_header = Header::from_config();
                            if let Some(header) = state.header.as_ref() {
                                new_header.video_player = header.video_player;
                                new_header.video_player_path = header.video_player_path.clone();
                            }
                            let buffer = new_header.save();
                            if let Err(err) =
                                Body::from_paths(state.paths.as_slice()).save(buffer, path)
                            {
                                message_dialog_error(err);
                            } else {
                                state.header.replace(new_header);
                            }
                        }
                    }

                    ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                        if self
                            .load_icon
                            .show(Vec2::new(max_height, max_height), ui)
                            .clicked()
                            && state.can_change()
                        {
                            if let Some(path) = rfd::FileDialog::new()
                                .set_title("LOAD")
                                .add_filter(playlist::EXTENSION, &[playlist::EXTENSION])
                                .pick_file()
                            {
                                match Header::load(path) {
                                    Err(err) => {
                                        message_dialog_error(err);
                                    }
                                    Ok((data, header)) => match Body::load(data) {
                                        Err(err) => {
                                            message_dialog_error(err);
                                        }
                                        Ok(body) => {
                                            if header.media_type != Config::media_type() {
                                                message_dialog_error(
                                                    "This playlist has a different media type!",
                                                );
                                            } else {
                                                state.header.replace(header);
                                                body.write_paths(&mut state.paths);
                                                state.paths_changed = true;
                                            }
                                        }
                                    },
                                }
                            }
                        }
                    });
                });
            });

            if let Some(header) = state.header.as_mut() {
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
                                    if header.video_player != Config::video_player() {
                                        Some(Color32::LIGHT_RED)
                                    } else {
                                        None
                                    },
                                ));
                                self.config_video_player.show_config(ui, ctx);
                                self.config_video_player.show_hint(ui, ctx);
                                ui.end_row();

                                match header.video_player {
                                    #[cfg(feature = "native")]
                                    VideoPlayer::Native => {}
                                    _ => {
                                        ui.label(gen_rich_text(
                                            ctx,
                                            t!("ui.config.video_player_path.label"),
                                            TextStyle::Body,
                                            if header.video_player_path
                                                != Config::video_player_path()
                                            {
                                                Some(Color32::LIGHT_RED)
                                            } else {
                                                None
                                            },
                                        ));
                                        self.config_video_player_path.show_config(ui, ctx);
                                        self.config_video_player_path.show_hint(ui, ctx);
                                        ui.end_row();
                                    }
                                }
                            }
                        });
                });
            }
        });
    }
}

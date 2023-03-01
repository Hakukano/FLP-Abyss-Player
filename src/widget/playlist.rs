use std::path::{Path, PathBuf};

use eframe::{
    egui::{self, Layout, TextStyle},
    emath::Align,
    epaint::{Color32, Vec2},
};

use crate::{config, font::gen_rich_text, get_cli};

use super::button_icon::ButtonIcon;

pub struct PlaylistState {
    paths: Vec<PathBuf>,
    paths_changed: bool,
    index: usize,
    index_changed: bool,
    search_string: String,
    search: bool,
}

impl PlaylistState {
    pub fn new(paths: Vec<PathBuf>, index: usize) -> Self {
        Self {
            paths,
            paths_changed: false,
            index,
            index_changed: false,
            search_string: String::new(),
            search: false,
        }
    }

    pub fn set_paths(&mut self, paths: Vec<PathBuf>) {
        self.paths = paths;
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
    add_icon: ButtonIcon,
    up_icon: ButtonIcon,
    down_icon: ButtonIcon,
    remove_icon: ButtonIcon,
    save_icon: ButtonIcon,
    load_icon: ButtonIcon,
}

impl Playlist {
    pub fn new(ctx: &egui::Context) -> Self {
        let icon_path = Path::new(get_cli().assets_path.as_str())
            .join("image")
            .join("icon");
        Self {
            search_icon: ButtonIcon::from_rgba_image_files(
                "search",
                icon_path.join("search.png"),
                ctx,
            ),
            add_icon: ButtonIcon::from_rgba_image_files("add", icon_path.join("add.png"), ctx),
            up_icon: ButtonIcon::from_rgba_image_files("up", icon_path.join("up.png"), ctx),
            down_icon: ButtonIcon::from_rgba_image_files("down", icon_path.join("down.png"), ctx),
            remove_icon: ButtonIcon::from_rgba_image_files(
                "remove",
                icon_path.join("remove.png"),
                ctx,
            ),
            save_icon: ButtonIcon::from_rgba_image_files("save", icon_path.join("save.png"), ctx),
            load_icon: ButtonIcon::from_rgba_image_files("load", icon_path.join("load.png"), ctx),
        }
    }

    pub fn show(
        &mut self,
        state: &mut PlaylistState,
        config: &mut config::Config,
        ui: &mut egui::Ui,
        ctx: &egui::Context,
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
                                .add_icon
                                .show(Vec2::new(max_height, max_height), ui)
                                .clicked()
                            {
                                // TODO
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
                egui::ScrollArea::vertical().show_rows(
                    ui,
                    max_height,
                    state.paths.len(),
                    |ui, row_range| {
                        for row in row_range {
                            ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                                if ui
                                    .button(gen_rich_text(
                                        ctx,
                                        state
                                            .paths
                                            .get(row)
                                            .map(|p| p.to_str().expect("Invalid path").to_string())
                                            .expect("Out of range: paths"),
                                        text_style.clone(),
                                        if row == state.index {
                                            Some(Color32::LIGHT_GREEN)
                                        } else {
                                            None
                                        },
                                    ))
                                    .clicked()
                                {
                                    state.index_changed = true;
                                    state.index = row;
                                }

                                ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                                    ui.spacing_mut().item_spacing.x = 5.0;
                                    if self
                                        .remove_icon
                                        .show(Vec2::new(max_height, max_height), ui)
                                        .clicked()
                                    {
                                        // TODO
                                    }
                                    if self
                                        .down_icon
                                        .show(Vec2::new(max_height, max_height), ui)
                                        .clicked()
                                    {
                                        // TODO
                                    }
                                    if self
                                        .up_icon
                                        .show(Vec2::new(max_height, max_height), ui)
                                        .clicked()
                                    {
                                        // TODO
                                    }
                                });
                            });
                        }
                    },
                );
            });
        });
    }
}

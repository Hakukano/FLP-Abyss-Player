use std::path::Path;

use eframe::{egui, epaint::Vec2};

use crate::{
    config::{self, AUTO_INTERVAL_RANGE},
    get_cli,
};

use super::toggle_icon::ToggleIcon;

pub struct PlayerBar {
    repeat_icon: ToggleIcon,
    auto_icon: ToggleIcon,
    loop_icon: ToggleIcon,
    random_icon: ToggleIcon,
}

impl PlayerBar {
    pub fn new(ctx: &egui::Context) -> Self {
        let icon_path = Path::new(get_cli().assets_path.as_str())
            .join("image")
            .join("icon");
        Self {
            repeat_icon: ToggleIcon::from_rgba_image_files(
                "repeat",
                icon_path.join("repeat-on.png"),
                icon_path.join("repeat-off.png"),
                ctx,
            ),
            auto_icon: ToggleIcon::from_rgba_image_files(
                "auto",
                icon_path.join("auto-on.png"),
                icon_path.join("auto-off.png"),
                ctx,
            ),
            loop_icon: ToggleIcon::from_rgba_image_files(
                "loop",
                icon_path.join("loop-on.png"),
                icon_path.join("loop-off.png"),
                ctx,
            ),
            random_icon: ToggleIcon::from_rgba_image_files(
                "random",
                icon_path.join("random-on.png"),
                icon_path.join("random-off.png"),
                ctx,
            ),
        }
    }

    pub fn show(&mut self, max_cross: f32, config: &mut config::Config, ui: &mut egui::Ui) {
        ui.spacing_mut().item_spacing = Vec2::new(1.0, 1.0);
        let spacing = max_cross / 4.0;
        let max_icon_size = Vec2::new(max_cross, max_cross);

        let mut repeat = config.repeat;
        if self
            .repeat_icon
            .show(max_icon_size, ui, &mut repeat)
            .changed()
        {
            config.repeat = repeat;
        }
        ui.add_space(spacing);
        let mut auto = config.auto;
        if self.auto_icon.show(max_icon_size, ui, &mut auto).changed() {
            config.auto = auto;
        }
        let mut auto_interval = config.auto_interval;
        if ui
            .add(
                egui::DragValue::new(&mut auto_interval)
                    .speed(1)
                    .clamp_range(AUTO_INTERVAL_RANGE)
                    .suffix("s"),
            )
            .changed()
        {
            config.auto_interval = auto_interval;
        }
        ui.add_space(spacing);
        let mut lop = config.lop;
        if self.loop_icon.show(max_icon_size, ui, &mut lop).changed() {
            config.lop = lop;
        }
        ui.add_space(spacing);
        let mut random = config.random;
        if self
            .random_icon
            .show(max_icon_size, ui, &mut random)
            .changed()
        {
            config.random = random;
        }
    }
}

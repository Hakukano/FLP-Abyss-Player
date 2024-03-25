#![allow(clippy::too_many_arguments)]

use std::{path::Path, sync::mpsc::Sender};

use eframe::{egui, epaint::Vec2};

use crate::{models::config::AUTO_INTERVAL_RANGE, utils::cli::CLI, views::timer::TimerSignal};

use super::toggle_icon::ToggleIcon;

pub struct PlayerBar {
    repeat_icon: ToggleIcon,
    auto_icon: ToggleIcon,
    loop_icon: ToggleIcon,
    random_icon: ToggleIcon,
}

impl PlayerBar {
    pub fn new(ctx: &egui::Context) -> Self {
        let icon_path = Path::new(CLI.assets_path.as_str())
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

    pub fn update(
        &mut self,
        timer_signal_tx: &Sender<TimerSignal>,
        max_cross: f32,
        ui: &mut egui::Ui,
        repeat: &mut bool,
        auto: &mut bool,
        auto_interval: &mut u32,
        lop: &mut bool,
        random: &mut bool,
    ) {
        let auto_original = *auto;
        let auto_interval_original = *auto_interval;

        ui.spacing_mut().item_spacing = Vec2::new(1.0, 1.0);
        let spacing = max_cross / 4.0;
        let max_icon_size = Vec2::new(max_cross, max_cross);

        self.repeat_icon.update(max_icon_size, ui, repeat);

        ui.add_space(spacing);

        self.auto_icon.update(max_icon_size, ui, auto);
        ui.add(
            egui::DragValue::new(auto_interval)
                .speed(1)
                .clamp_range(AUTO_INTERVAL_RANGE)
                .suffix("s"),
        );

        ui.add_space(spacing);

        self.loop_icon.update(max_icon_size, ui, lop);

        ui.add_space(spacing);

        self.random_icon.update(max_icon_size, ui, random);

        // Check auto change
        if *auto != auto_original {
            if *auto {
                let _ = timer_signal_tx.send(TimerSignal::Start(*auto_interval));
            } else {
                let _ = timer_signal_tx.send(TimerSignal::Stop);
            }
        } else if *auto_interval != auto_interval_original {
            let _ = timer_signal_tx.send(TimerSignal::Update(*auto_interval));
        }
    }
}

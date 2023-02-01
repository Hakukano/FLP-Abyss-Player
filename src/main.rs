#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod app;
mod locale;

use eframe::egui;

#[macro_export]
macro_rules! sized_text {
    ($text:expr, $size:expr, $color:expr) => {
        RichText::new($text)
            .font(FontId::proportional($size))
            .color($color)
    };
    ($text:expr, $size:expr) => {
        RichText::new($text)
            .font(FontId::proportional($size))
            .color(Color32::WHITE)
    };
}

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1600.0, 900.0)),
        ..Default::default()
    };
    eframe::run_native(
        locale::get().ui.app_name.as_str(),
        options,
        Box::new(|_cc| Box::<app::App>::default()),
    )
}

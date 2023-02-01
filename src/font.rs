use std::{env, fs, path::Path};

use eframe::egui;

pub fn init(ctx: &egui::Context) {
    let font_path = env::var("FONT_PATH").unwrap_or("./font".to_string());
    let fonts = env::var("FONTS").unwrap_or("NotoSansCJKjp-Regular.otf;Inter-Regular.ttf".to_string());
    let fonts = fonts.split(';').collect::<Vec<_>>();

    let mut fd = egui::FontDefinitions::default();
    for (i, font) in fonts.iter().enumerate() {
        fd.font_data.insert(
            font.to_string(),
            egui::FontData::from_owned(
                fs::read(Path::new(font_path.as_str()).join(font)).expect("Cannot read font file"),
            ),
        );
        fd.families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(i, font.to_string());
    }
    ctx.set_fonts(fd);
}

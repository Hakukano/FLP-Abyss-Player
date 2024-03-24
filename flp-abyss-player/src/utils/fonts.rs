use std::{
    fs::{self, read_dir},
    path::Path,
};

use eframe::{
    egui::{
        self, FontId, RichText,
        TextStyle::{self, *},
    },
    epaint::Color32,
};

use crate::utils::cli::CLI;

pub fn init(ctx: &egui::Context) {
    let mut fd = egui::FontDefinitions::default();
    let assets_path = CLI.assets_path.as_str();
    let font_path = Path::new(assets_path).join("font");
    for (i, entry) in read_dir(font_path)
        .expect("Cannot read font directory")
        .flatten()
        .enumerate()
    {
        let font = entry
            .file_name()
            .to_str()
            .expect("Invalid path")
            .to_string();
        fd.font_data.insert(
            font.clone(),
            egui::FontData::from_owned(fs::read(entry.path()).expect("Cannot read font file")),
        );
        fd.families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(i, font);
    }
    ctx.set_fonts(fd);

    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (Heading, FontId::proportional(32.0)),
        (Body, FontId::proportional(16.0)),
        (Monospace, FontId::proportional(16.0)),
        (Button, FontId::proportional(16.0)),
        (Small, FontId::proportional(12.0)),
    ]
    .into();
    ctx.set_style(style);
}

pub fn gen_rich_text(
    ctx: &egui::Context,
    text: impl Into<String>,
    text_style: TextStyle,
    color: Option<Color32>,
) -> RichText {
    let mut rich_text = RichText::new(text).font(text_style.resolve(ctx.style().as_ref()));
    if let Some(color) = color {
        rich_text = rich_text.color(color);
    }
    rich_text
}

use std::{fs, path::Path};

use eframe::{
    egui::{
        self, FontId, RichText,
        TextStyle::{self, *},
    },
    epaint::Color32,
};

use crate::get_cli;

pub fn init(ctx: &egui::Context) {
    let cli = get_cli();
    let mut fd = egui::FontDefinitions::default();
    for (i, font) in cli.fonts.split(';').enumerate() {
        fd.font_data.insert(
            font.to_string(),
            egui::FontData::from_owned(
                fs::read(Path::new(cli.font_path.as_str()).join(font))
                    .expect("Cannot read font file"),
            ),
        );
        fd.families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(i, font.to_string());
    }
    ctx.set_fonts(fd);

    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (Heading, FontId::proportional(32.0)),
        (Body, FontId::proportional(16.0)),
        (Monospace, FontId::proportional(16.0)),
        (Button, FontId::proportional(16.0)),
        (Small, FontId::proportional(8.0)),
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

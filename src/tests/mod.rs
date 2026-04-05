use crate::ptui::Ptui;
use crate::tiling::text::TextTile;
use crate::traits::{Color, Colored};

#[test]
fn color_text() {
    Ptui::init("Test".parse().unwrap(), None, None, 33);
    Ptui::push(TextTile::from_static("test".foreground(&Color::Custom("\x1B[48;5;32m".into()))).into());
}

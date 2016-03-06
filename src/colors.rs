use piston_window::color;
use piston_window::types::Color;

// Lifted and modified from Conrod
macro_rules! make_color {
    ($r:expr, $g:expr, $b:expr) => ([$r as f32 / 255.0, $g as f32 / 255.0, $b as f32 / 255.0, 1.0]);
    ($r:expr, $g:expr, $b:expr, $a:expr) => ([$r as f32 / 255.0, $g as f32 / 255.0, $b as f32 / 255.0, $a as f32 / 255.0]);
}

// Tetromino colors
pub const CYAN: Color = make_color!(0x00, 0xff, 0xff);
pub const BLUE: Color = make_color!(0x34, 0x65, 0xA4);
pub const ORANGE: Color = make_color!(0xF5, 0x79, 0x00);
pub const YELLOW: Color = make_color!(0xED, 0xD4, 0x00);
pub const LIME: Color = make_color!(0x80, 0xFF, 0x00);
pub const PURPLE: Color = make_color!(0x75, 0x50, 0x7B);
pub const RED: Color = make_color!(0xCC, 0x00, 0x00);

// Grid color
pub const GREY: Color = [0.15, 0.15, 0.15, 1.0];

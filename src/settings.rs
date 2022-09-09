use std::collections::BTreeMap;
use crossterm::style;

/// Struct which holds informations about canvas, width and height in this case
pub struct Canvas {
    pub width: u16,
    pub height: u16
}

/// Struct which holds following information:
/// cursor X and Y axis,
/// color for next block,
/// last pressed key (1 - 4, W, S, A, D),
/// cursor X and Y axis and color for block
/// which is already placed
pub struct Runtime {
    pub cursor_x: f64,
    pub cursor_y: f64,
    pub color: style::Color,
    pub last_pressed_key: u32,
    pub placed: BTreeMap<(u32, u32), style::Color>
}

/// Struct which holds runtime information which are not about cursors and blocks
pub struct State {
    pub window_open: bool,
    pub window_open_name: String
}

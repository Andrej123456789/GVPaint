use crossterm::style;
use std::collections::BTreeMap;

use crate::paint::KEY;

/// Struct holding information about canvas's width and height
pub struct Canvas {
    pub width: u16,
    pub height: u16,
}

/// Struct holding following information:
/// cursor's X and Y axis,
/// cursor's color
/// color for next block,
/// last pressed key (1 - 4, W, S, A, D),
/// placed blocks
pub struct Runtime {
    pub cursor_x: f64,
    pub cursor_y: f64,
    pub cursor_color: style::Color,
    pub color: style::Color,
    pub last_pressed_key: KEY,
    pub placed: BTreeMap<(u32, u32), style::Color>,
}

/// Struct which holds runtime information about windows
pub struct State {
    pub window_open: bool,
    pub window_open_name: String,
}

/// Struct holding temporary information required for file menu
pub struct FileMenu {
    pub file_content: String,
}

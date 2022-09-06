use std::collections::BTreeMap;

/// Struct which holds following information:
/// cursor X and Y axis,
/// color for next block,
/// last pressed key (1 - 4, W, S, A, D),
/// cursor X and Y axis and color for block
/// which is already placed
pub struct Runtime {
    pub cursor_x: f64,
    pub cursor_y: f64,
    pub color: u32,
    pub last_pressed_key: u32,
    pub placed: BTreeMap<(u32, u32), u32>,
}

#![allow(unused)]

use std::{
    collections::BTreeMap,
    io::{stdout, Write}, process::exit
};

use crossterm::{style, terminal, QueueableCommand};

mod paint;
mod variables;
mod window;

/// Entry point for program
fn main() {
    clearscreen::clear().expect("Failed to clean screen!");
    let mut stdout = stdout();

    let size = terminal::size().unwrap();
    let x = size.0;
    let y = size.1;

    let mut i = 0;
    let mut j = 0;

    stdout.queue(style::SetBackgroundColor(style::Color::White));

    while i < y {
        while j < x {
            println!(" ");
            i += 1;
            j += 1;
        }
    }

    let mut canvas = variables::Canvas {
        width: x,
        height: y,
    };

    let x_2 = (x as f64) / (2.2 as f64);
    let y_2 = (y as f64) / (2.2 as f64);

    let mut placed: BTreeMap<(u32, u32), style::Color> = BTreeMap::new();
    let mut runtime = variables::Runtime {
        cursor_x: x_2,
        cursor_y: y_2,
        cursor_color: style::Color::Black,
        color: style::Color::Green,
        last_pressed_key: paint::KEY::NONE,
        placed: placed,
    };

    let mut state = variables::State {
        window_open: false,
        window_open_name: "none".to_string(),
    };

    let mut file_menu = variables::FileMenu {
        file_content: " ".to_string(),
    };

    paint::paint(&mut canvas, &mut runtime, &mut state, &mut file_menu);

    stdout.queue(style::SetBackgroundColor(style::Color::Reset));
    stdout.queue(style::SetForegroundColor(style::Color::Reset));
    clearscreen::clear().expect("Failed to clean screen!");
}

#![allow(unused)]

use std::{io::{Write, stdout}};
use std::collections::BTreeMap;
use crossterm::{terminal, style, QueueableCommand};

mod logic;
mod settings;

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
            i+=1;
            j+=1;
        }
    }

    let x_2 = (x as f64) / (2.2 as f64);
    let y_2 = (y as f64) / (2.2 as f64);

    let mut placed: BTreeMap<(u32, u32), u32> = BTreeMap::new();
    let mut i = 0;

    for i in (0..16) {
        placed.insert((i as u32, 0), 3);
    }

    let mut runtime = settings::Runtime {
        cursor_x: x_2,
        cursor_y: y_2,
        color: 3,
        last_pressed_key: 0,
        placed: placed
    };

    logic::logic(x, y, &mut runtime);

    stdout.queue(style::SetBackgroundColor(style::Color::Reset));
    stdout.queue(style::SetForegroundColor(style::Color::Reset));
    clearscreen::clear().expect("Failed to clean screen!");
}

#![allow(unused)]

use std::{io::{Write, stdout}, thread, time::Duration};
use crossterm::{terminal, style, QueueableCommand};

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

    thread::sleep(Duration::from_secs(2));

    stdout.queue(style::SetBackgroundColor(style::Color::Reset));
    clearscreen::clear().expect("Failed to clean screen!");
}
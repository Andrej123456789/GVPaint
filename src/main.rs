#![allow(unused)]

use std::{io::Write, process::Termination};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

extern crate termsize;

fn main() {
    clearscreen::clear().expect("Failed to clean screen!");

    let mut colorspec = ColorSpec::new();
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let (x, y) = termion::terminal_size().unwrap();

    let mut i= 0;
    let mut j = 0;

    while i < y {
        while j < x {
            stdout.set_color(&colorspec.set_bg(Some(Color::Rgb((255), (255), (255)))));
            writeln!(stdout, "{} ",
            termion::cursor::Goto(j, i));
            i+=1;
            j+=1;
        }
    }   

    stdout.set_color(&colorspec.set_bg(Some(Color::Black)));
}

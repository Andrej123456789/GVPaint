use std::{
    collections::BTreeMap,
    io::{self, prelude::*, stdout, Stdout},
};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    style::{self, Color, SetForegroundColor, SetStyle},
    terminal::{self, disable_raw_mode, enable_raw_mode}, QueueableCommand,
};

use crate::variables;
use crate::window;

#[derive(Clone)]
pub enum KEY {
    NONE,
    W,
    S,
    A,
    D,
    FILE,
    HELP,
    PLACE,
    ERASE,
    QUIT,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    C0,
}

/// BIOS color attribute equivalent of crossterm colors used below.
/// L - light; D - dark
enum COLOR {
    BLACK,
    DBLUE,
    LGREEN,
    LCYAN,
    LRED,
    LMAGENTA,
    BROWN,
    LGREY,
    YELLOW,
    WHITE,
}

/// Read a character from user input
fn read_user_input_character() -> Option<char> {
    enable_raw_mode().expect("Failed to enable raw mode");

    let result = if let Event::Key(key_event) = event::read().expect("Failed to read event") {
        match key_event.code {
            KeyCode::Char(c) => Some(c),
            _ => None,
        }
    } else {
        None
    };

    disable_raw_mode().expect("Failed to disable raw mode");

    result
}

/// Return an enum value of a pressed key
fn cursor_input() -> KEY {
    if let Some(ch) = read_user_input_character()
    {
        let ch_u32: u32 = ch as u32;
        match ch_u32 {
            119 | 87 => return KEY::W,
            115 | 83 => return KEY::S,
            97 | 65 => return KEY::A,
            100 | 68 => return KEY::D,
            102 | 70 => return KEY::FILE,
            104 | 72 => return KEY::HELP,
            112 | 80 => return KEY::PLACE,
            101 | 69 => return KEY::ERASE,
            113 | 81 => return KEY::QUIT,
            49 => return KEY::C1,
            50 => return KEY::C2,
            51 => return KEY::C3,
            52 => return KEY::C4,
            53 => return KEY::C5,
            54 => return KEY::C6,
            55 => return KEY::C7,
            56 => return KEY::C8,
            57 => return KEY::C9,
            48 => return KEY::C0,
            _ => return KEY::NONE,
        }
    }

    return KEY::NONE;
}

/// Convert KEY to crossterm::style::Color
pub fn return_color(key: KEY) -> Color {
    match key {
        KEY::C1 => return style::Color::Black,
        KEY::C2 => return style::Color::Blue,
        KEY::C3 => return style::Color::Green,
        KEY::C4 => return style::Color::Cyan,
        KEY::C5 => return style::Color::Red,
        KEY::C6 => return style::Color::Magenta,
        KEY::C7 => return style::Color::Rgb { // brown
            r: 170, 
            g: 85, 
            b: 0 
        },
        KEY::C8 => return style::Color::Grey,
        KEY::C9 => return style::Color::Yellow,
        KEY::C0 => return style::Color::White,
        _ => return style::Color::Black,
    }
}

/// Convert crossterm::style::Color to KEY
pub fn return_color_int(color: style::Color) -> KEY {
    match color {
        style::Color::Black => return KEY::C1,
        style::Color::Blue => return KEY::C2,
        style::Color::Green => return KEY::C3,
        style::Color::Cyan => return KEY::C4,
        style::Color::Red => return KEY::C5,
        style::Color::Magenta => return KEY::C6,
        style::Color::Rgb { // brown
            r: 170,
            g: 85,
            b: 0,
        } => return KEY::C7,
        style::Color::Grey => return KEY::C8,
        style::Color::Yellow => return KEY::C9,
        style::Color::White => return KEY::C0,
        _ => return KEY::C1,
    }
}

/// Update cursor position, but DON'T draw it on the new position. 
/// Based on last pressed key (W, S, A, D)
fn move_cursor_blkey(runtime: &mut variables::Runtime) {
    match runtime.last_pressed_key {
        KEY::W => runtime.cursor_y -= 1.0,
        KEY::S => runtime.cursor_y += 1.0,
        KEY::D => runtime.cursor_x -= 1.0,
        KEY::A => runtime.cursor_x += 1.0,
        _ => runtime.cursor_x -= 1.0,
    }
}

/// Remove an old cursor. If painting is below, redraw it
fn remove_old_cursor(stdout: &mut Stdout, runtime: &mut variables::Runtime) {
    stdout.queue(cursor::MoveTo(
        runtime.cursor_x as u16,
        runtime.cursor_y as u16,
    ));
    stdout.queue(style::SetForegroundColor(style::Color::White));
    println!("\u{2588}");

    let mut placed = runtime.placed.clone();

    for (k, v) in placed {
        if (runtime.cursor_x as u32) == k.0 {
            /* we can just check any axis, x or y */
            stdout.queue(cursor::MoveTo(k.0 as u16, k.1 as u16));
            stdout.queue(style::SetForegroundColor(v));
            println!("\u{2588}");
        }
    }
}

/// Place a new cursor
fn place_new_cursor(
    stdout: &mut Stdout,
    canvas: &mut variables::Canvas,
    runtime: &mut variables::Runtime,
) {
    if (runtime.cursor_y as u16) != (canvas.height - 2) {
        stdout.queue(cursor::MoveTo(
            runtime.cursor_x as u16,
            runtime.cursor_y as u16,
        ));
        stdout.queue(style::SetForegroundColor(runtime.cursor_color));
        println!("\u{2588}");
    } else {
        remove_old_cursor(stdout, runtime);

        runtime.cursor_y -= 2.0;
        place_new_cursor(stdout, canvas, runtime);
    }
}

/// Place a blok
pub fn place_blok(stdout: &mut Stdout, runtime: &mut variables::Runtime) {
    runtime.placed.insert(
        (runtime.cursor_x as u32, runtime.cursor_y as u32),
        runtime.color,
    );
    stdout.queue(cursor::MoveTo(
        runtime.cursor_x as u16,
        runtime.cursor_y as u16,
    ));
    stdout.queue(style::SetForegroundColor(runtime.color));
    println!("\u{2588}");
}

/// Close a window or exit the program
fn close(
    stdout: &mut Stdout,
    canvas: &mut variables::Canvas,
    runtime: &mut variables::Runtime,
    state: &mut variables::State,
) {
    if state.window_open == true && state.window_open_name == "help" {
        window::help_window(stdout, canvas, runtime, state);
    } else if state.window_open == true && state.window_open_name == "file" {
        window::file_window(stdout, canvas, runtime, state);
    } else {
        std::process::exit(0);
    }
}

/// Redraw a canvas, keep a painting intact
pub fn redraw_canvas(stdout: &mut Stdout, runtime: &mut variables::Runtime) {
    clearscreen::clear().expect("Failed to clear screen!");

    stdout.queue(style::SetForegroundColor(style::Color::Red));
    stdout.queue(cursor::MoveTo(0, 0));
    println!("Press 'H' or 'h' for help!");

    let mut placed = runtime.placed.clone();
    for (k, v) in placed {
        if k.0 != 0 {
            /* we can just check any axis, x or y */
            stdout.queue(cursor::MoveTo(k.0 as u16, k.1 as u16));
            stdout.queue(style::SetForegroundColor(v));
            println!("\u{2588}");
        }
    }

    stdout.queue(cursor::EnableBlinking);
    stdout.queue(cursor::MoveTo(
        runtime.cursor_x as u16,
        runtime.cursor_y as u16,
    ));
    stdout.queue(style::SetForegroundColor(runtime.cursor_color));
    println!("\u{2588}");

    return;
}

/// Entry function for drawing
pub fn paint(
    canvas: &mut variables::Canvas,
    runtime: &mut variables::Runtime,
    state: &mut variables::State,
    file_menu: &mut variables::FileMenu,
) {
    let mut stdout: Stdout = stdout();

    stdout.queue(style::SetForegroundColor(style::Color::Red));
    stdout.queue(cursor::MoveTo(0, 0));
    println!("Press 'H' or 'h' for help!");

    stdout.queue(cursor::EnableBlinking);
    stdout.queue(cursor::MoveTo(
        runtime.cursor_x as u16,
        runtime.cursor_y as u16,
    ));
    stdout.queue(style::SetForegroundColor(runtime.cursor_color));
    println!("\u{2588}");

    loop {
        let key: KEY = cursor_input();

        match key {
            KEY::W => {
                remove_old_cursor(&mut stdout, runtime);
                runtime.cursor_y -= 1.0;

                place_new_cursor(&mut stdout, canvas, runtime);
                runtime.last_pressed_key = KEY::W;
            }
            KEY::S => {
                remove_old_cursor(&mut stdout, runtime);
                runtime.cursor_y += 1.0;

                place_new_cursor(&mut stdout, canvas, runtime);
                runtime.last_pressed_key = KEY::S;
            }
            KEY::A => {
                remove_old_cursor(&mut stdout, runtime);
                runtime.cursor_x -= 1.0;

                place_new_cursor(&mut stdout, canvas, runtime);
                runtime.last_pressed_key = KEY::D;
            }
            KEY::D => {
                remove_old_cursor(&mut stdout, runtime);
                runtime.cursor_x += 1.0;

                place_new_cursor(&mut stdout, canvas, runtime);
                runtime.last_pressed_key = KEY::A;
            }
            KEY::FILE => window::file_window(&mut stdout, canvas, runtime, state),
            KEY::HELP => window::help_window(&mut stdout, canvas, runtime, state),
            KEY::PLACE => {
                if (state.window_open == true && state.window_open_name == "file") {
                    window::file_window_actions(&mut stdout, canvas, runtime, state, file_menu);
                } else {
                    place_blok(&mut stdout, runtime);

                    move_cursor_blkey(runtime);
                    place_new_cursor(&mut stdout, canvas, runtime);
                }
            }
            KEY::ERASE => {
                let current_color = runtime.color;
                runtime.color = style::Color::White;

                place_blok(&mut stdout, runtime);
                runtime.color = current_color;

                move_cursor_blkey(runtime);
                place_new_cursor(&mut stdout, canvas, runtime);
            }
            KEY::QUIT => {
                close(&mut stdout, canvas, runtime, state);
            }
            _ => {
                if (key.clone() as u32 >= KEY::C1 as u32 && key.clone() as u32 <= KEY::C0 as u32) {
                    runtime.color = return_color(key);
                }
            }
        }
    }
}

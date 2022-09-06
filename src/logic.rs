use std::{{io, io::{Read, Write, Stdout, stdout}}};
use std::collections::BTreeMap;
use crossterm::{terminal, style::{self, Color}, cursor, QueueableCommand};
use termios::{Termios, TCSANOW, ECHO, ICANON, tcsetattr};

use crate::settings;

static mut ALREADY: bool = false;

enum KEY {
    ERROR,
    W,
    S,
    A,
    D,
    FILE,
    HELP,
    PLACE,
    ERASE,
    QUIT
}

enum COLOR {
    BLACK,
    GREY,
    RED,
    GREEN,
    BLUE,
    MAGENTA,
    LGREEN,
    AQUA,
    YELLOW,
    ORANGE,
    BROWN,
    WHITE
}

/// Reads a character from user input. If multiple characters are given,
/// character at first index is returned. In any problematic cases, return
/// an asterisk (*).
fn read_user_input_character() -> char {
    let stdin = 0;
    let termios = Termios::from_fd(stdin).unwrap();
    let mut new_termios = termios.clone();


    new_termios.c_lflag &= !(ICANON | ECHO); 
    tcsetattr(stdin, TCSANOW, &mut new_termios).unwrap();

    let stdout = io::stdout();
    let mut reader = io::stdin();
    let mut buffer = [0;1];

    stdout.lock().flush().unwrap();
    reader.read_exact(&mut buffer).unwrap();
    tcsetattr(stdin, TCSANOW, & termios).unwrap();

    return buffer[0] as char;
}

/// Returns enum value of keys which user enter
fn cursor_input() -> u32 {
    let ch = read_user_input_character();

    unsafe {
        if ch == '*' && ALREADY == false {
            ALREADY = true;
            cursor_input();
        }

        else if ch == '*' && ALREADY == true {
            return KEY::ERROR as u32;
        }
    }

    let ch_u32: u32 = ch as u32;

    match ch_u32 {
        119 | 87 => return KEY::W as u32,
        115 | 83 => return KEY::S as u32,
        97  | 65 => return KEY::A as u32,
        100 | 68 => return KEY::D as u32,
        102 | 70 => return KEY::FILE as u32,
        104 | 72 => return KEY::HELP as u32,
        112 | 80 => return KEY::PLACE as u32,
        101 | 69 => return KEY::ERASE as u32,
        113 | 81 => return KEY::QUIT as u32,
        _ => return KEY::ERROR as u32,
    }
}

/// Returns color in style::Color based on number (0 - 11)
fn return_color(color: u32) -> Color {
    let mut crossterm_color = style::Color::Black;
    match color {
        0 => crossterm_color =  style::Color::Black,
        1 => crossterm_color =  style::Color::Grey,
        2 => crossterm_color =  style::Color::Red,
        3 => crossterm_color =  style::Color::Green,
        4 => crossterm_color =  style::Color::Blue,
        5 => crossterm_color =  style::Color::Magenta,
        6 => crossterm_color =  style::Color::Rgb{r: 14, g: 237, b: 22},
        7 => crossterm_color =  style::Color::Rgb{r: 24, g: 194, b: 137},
        8 => crossterm_color =  style::Color::Yellow,
        9 => crossterm_color =  style::Color::Rgb{r: 237, g: 116, b: 24},
        10 => crossterm_color =  style::Color::Rgb{r: 102, g: 0, b: 0},
        11 => crossterm_color = style::Color::White,
        _ => crossterm_color = style::Color::Black
    }

    return crossterm_color;
}

/// Removing old cursor, if painting is there, redraw it
fn remove_old_cursor(stdout: &mut Stdout, runtime: &mut settings::Runtime) {
    stdout.queue(cursor::MoveTo(runtime.cursor_x as u16, runtime.cursor_y as u16));
    stdout.queue(style::SetForegroundColor(style::Color::White));
    println!("\u{2588}");

    let mut placed = runtime.placed.clone();

    for (k, v) in placed {
        if (runtime.cursor_x as u32) == k.0 { /* we can just check any axis, x or y */
            stdout.queue(cursor::MoveTo(k.0 as u16, k.1 as u16));
            stdout.queue(style::SetForegroundColor(return_color(v)));
            println!("\u{2588}");
        }
    }
}

/// Placing new cursor
fn place_new_cursor(stdout: &mut Stdout, runtime: &mut settings::Runtime, height: u16) -> f64 {
    if (runtime.cursor_y as u16) != (height - 2) {
        stdout.queue(cursor::MoveTo(runtime.cursor_x as u16, runtime.cursor_y as u16));
        stdout.queue(style::SetForegroundColor(style::Color::Black));
        println!("\u{2588}");
    }

    else {
        remove_old_cursor(stdout, runtime);

        runtime.cursor_y -= 2.0;
        runtime.cursor_y = place_new_cursor(stdout, runtime, height);
    }

    return runtime.cursor_y;
}

/// Placing blok
fn place_blok(stdout: &mut Stdout, runtime: &mut settings::Runtime) {
    runtime.placed.insert((runtime.cursor_x as u32, runtime.cursor_y as u32), runtime.color);
    stdout.queue(cursor::MoveTo(runtime.cursor_x as u16, runtime.cursor_y as u16));
    stdout.queue(style::SetForegroundColor(return_color(runtime.color)));
    println!("\u{2588}");
}

/// Entry function for drawing
pub fn logic(width: u16, height: u16, runtime: &mut settings::Runtime) {
    let mut stdout: Stdout = stdout();

    stdout.queue(style::SetForegroundColor(style::Color::Red));
    stdout.queue(cursor::MoveTo(0, 0));
    println!("File\tHelp");

    stdout.queue(cursor::EnableBlinking);
    stdout.queue(cursor::MoveTo(runtime.cursor_x as u16, runtime.cursor_y as u16));
    stdout.queue(style::SetForegroundColor(style::Color::Black));
    println!("\u{2588}");

    loop {
        let key: u32 = cursor_input();

        match key {
            1 => {remove_old_cursor(&mut stdout, runtime);
                    runtime.cursor_y -= 1.0;
                    runtime.cursor_y = place_new_cursor(&mut stdout, runtime, height);},
            2 => {remove_old_cursor(&mut stdout, runtime);
                    runtime.cursor_y += 1.0;
                    runtime.cursor_y = place_new_cursor(&mut stdout, runtime, height);},
            3 => {remove_old_cursor(&mut stdout, runtime);
                    runtime.cursor_x -= 1.0;
                    runtime.cursor_y =  place_new_cursor(&mut stdout, runtime, height);},
            4 => {remove_old_cursor(&mut stdout, runtime);
                    runtime.cursor_x += 1.0;
                    runtime.cursor_y =  place_new_cursor(&mut stdout, runtime, height);},
            5 => { /* will be there soon, hopefully */ },
            6 => { /* will be there soon, hopefully */ },
            7 => {place_blok(&mut stdout, runtime); /* all colors will be implemented soon, hopefully */
                    runtime.cursor_x -= 1.0;
                    runtime.cursor_y =  place_new_cursor(&mut stdout, runtime, height);},
            8 => {place_blok(&mut stdout, runtime);
                    runtime.cursor_x -= 1.0;
                    runtime.cursor_y = place_new_cursor(&mut stdout, runtime, height);},
            9 => { break; },
            _ => { /* ignore */ }
        }
    }
}

use std::{{io, io::{Read, Write, Stdout, stdout}}};
use std::collections::BTreeMap;

use crossterm::{terminal, style::{self, Color, SetForegroundColor}, cursor, QueueableCommand};
use termios::{Termios, TCSANOW, ECHO, ICANON, tcsetattr};

use crate::settings;

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
    C0
}

enum COLOR {
    BLACK,
    GREY,
    RED,
    GREEN,
    BLUE,
    LGREEN,
    AQUA,
    YELLOW,
    ORANGE,
    BROWN,
    WHITE
}

/// Reads a character from user input
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
        49 => return KEY::C1 as u32,
        50 => return KEY::C2 as u32,
        51 => return KEY::C3 as u32,
        52 => return KEY::C4 as u32,
        53 => return KEY::C5 as u32,
        54 => return KEY::C6 as u32,
        55 => return KEY::C7 as u32,
        56 => return KEY::C8 as u32,
        57 => return KEY::C9 as u32,
        48 => return KEY::C0 as u32,
        _ => return KEY::ERROR as u32,
    }
}

/// Returns color in style::Color based on number (0 - 11)
fn return_color(color: u32) -> Color {
    let mut crossterm_color = style::Color::Black;
    match color {
        10 => crossterm_color =  style::Color::Black,
        11 => crossterm_color =  style::Color::Grey,
        12 => crossterm_color =  style::Color::Red,
        13 => crossterm_color =  style::Color::Green,
        14 => crossterm_color =  style::Color::Blue,
        15 => crossterm_color =  style::Color::Rgb{r: 14, g: 237, b: 22},
        16 => crossterm_color =  style::Color::Rgb{r: 24, g: 194, b: 137},
        17 => crossterm_color =  style::Color::Yellow,
        18 => crossterm_color =  style::Color::Rgb{r: 237, g: 116, b: 24},
        19 => crossterm_color =  style::Color::Rgb{r: 102, g: 0, b: 0},
        20 => crossterm_color = style::Color::White,
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
            stdout.queue(style::SetForegroundColor(v));
            println!("\u{2588}");
        }
    }
}

/// Placing new cursor
fn place_new_cursor(stdout: &mut Stdout, canvas: &mut settings::Canvas, runtime: &mut settings::Runtime) -> f64 {
    if (runtime.cursor_y as u16) != (canvas.height - 2) {
        stdout.queue(cursor::MoveTo(runtime.cursor_x as u16, runtime.cursor_y as u16));
        stdout.queue(style::SetForegroundColor(style::Color::Black));
        println!("\u{2588}");
    }

    else {
        remove_old_cursor(stdout, runtime);

        runtime.cursor_y -= 2.0;
        runtime.cursor_y = place_new_cursor(stdout, canvas, runtime);
    }

    return runtime.cursor_y;
}

/// Placing blok
fn place_blok(stdout: &mut Stdout, runtime: &mut settings::Runtime) {
    runtime.placed.insert((runtime.cursor_x as u32, runtime.cursor_y as u32), runtime.color);
    stdout.queue(cursor::MoveTo(runtime.cursor_x as u16, runtime.cursor_y as u16));
    stdout.queue(style::SetForegroundColor(runtime.color));
    println!("\u{2588}");
}

/// Small "window" with shows help information
fn help_window(stdout: &mut Stdout, canvas: &mut settings::Canvas, runtime: &mut settings::Runtime, state: &mut settings::State) {
    if state.window_open == true && state.window_open_name == "help" {
        /* close */
        state.window_open = false;

        clearscreen::clear().expect("Failed to clear screen!");

        stdout.queue(style::SetForegroundColor(style::Color::Red));
        stdout.queue(cursor::MoveTo(0, 0));
        println!("Press 'H' or 'h' for help!");

        let mut placed = runtime.placed.clone();
        for (k, v) in placed {
            if k.0 != 0 { /* we can just check any axis, x or y */
                stdout.queue(cursor::MoveTo(k.0 as u16, k.1 as u16));
                stdout.queue(style::SetForegroundColor(v));
                println!("\u{2588}");
            }
        }

        stdout.queue(cursor::EnableBlinking);
        stdout.queue(cursor::MoveTo(runtime.cursor_x as u16, runtime.cursor_y as u16));
        stdout.queue(style::SetForegroundColor(style::Color::Black));
        println!("\u{2588}");

        return;
    }

    state.window_open = true;
    state.window_open_name = "help".to_string();

    stdout.queue(style::SetForegroundColor(style::Color::DarkGreen));
    stdout.queue(cursor::MoveTo(canvas.width - canvas.width + 5, canvas.height - 18));
    println!("--- --- --- --- --- --- --- --- --- --- --- ---");

    let mut i = 17;
    while i != 2 {
        stdout.queue(cursor::MoveTo(canvas.width - canvas.width + 4, canvas.height - i));
        println!("|");
        i -= 1;
    }

    let mut j = 17;
    while j != 2 {
        stdout.queue(cursor::MoveTo(canvas.width - canvas.width + 52, canvas.height - j));
        println!("|");
        j -= 1;
    }

    stdout.queue(cursor::MoveTo(canvas.width - canvas.width + 5, canvas.height - 3));
    println!("--- --- --- --- --- --- --- --- --- --- --- ---");

    stdout.queue(cursor::MoveTo(canvas.width - canvas.width + 6, canvas.height - 17));
    stdout.queue(style::SetForegroundColor(style::Color::Red));
    println!("Keyboard shortcuts: ");
    println!("\t W - move cursor up");
    println!("\t S - move cursor down");
    println!("\t A - move cursor left");
    println!("\t D - move cursor right");
    println!("\t F - open 'file window'");
    println!("\t H - open 'help window', this one");
    println!("\t P - place block");
    println!("\t E - erase block");
    println!("\t Q - exit a program or close a window");
    println!("\t 0 - 9 - change color");
    println!(" ");
    println!("\t Made with Rust and thanks to StjepanBM1");
}

/// Small "window" where we should have following options:
/// Open picture
/// Save picture
fn file_window(stdout: &mut Stdout, canvas: &mut settings::Canvas, runtime: &mut settings::Runtime, state: &mut settings::State) {
    if state.window_open == true && state.window_open_name == "file" {
        /* close */
        state.window_open = false;

        clearscreen::clear().expect("Failed to clear screen!");

        stdout.queue(style::SetForegroundColor(style::Color::Red));
        stdout.queue(cursor::MoveTo(0, 0));
        println!("Press 'H' or 'h' for help!");

        let mut placed = runtime.placed.clone();
        for (k, v) in placed {
            if k.0 != 0 { /* we can just check any axis, x or y */
                stdout.queue(cursor::MoveTo(k.0 as u16, k.1 as u16));
                stdout.queue(style::SetForegroundColor(v));
                println!("\u{2588}");
            }
        }

        stdout.queue(cursor::EnableBlinking);
        stdout.queue(cursor::MoveTo(runtime.cursor_x as u16, runtime.cursor_y as u16));
        stdout.queue(style::SetForegroundColor(style::Color::Black));
        println!("\u{2588}");

        return;
    }

    state.window_open = true;
    state.window_open_name = "file".to_string();

    stdout.queue(style::SetForegroundColor(style::Color::DarkGreen));
    stdout.queue(cursor::MoveTo(canvas.width - canvas.width + 5, canvas.height - 16));
    println!("--- --- --- --- --- --- --- --- --- ---");

    let mut i = 15;
    while i != 3 {
        stdout.queue(cursor::MoveTo(canvas.width - canvas.width + 4, canvas.height - i));
        println!("|");
        i -= 1;
    }

    let mut j = 15;
    while j != 3 {
        stdout.queue(cursor::MoveTo(canvas.width - canvas.width + 44, canvas.height - j));
        println!("|");
        j -= 1;
    }

    stdout.queue(cursor::MoveTo(canvas.width - canvas.width + 5, canvas.height - 3));
    println!("--- --- --- --- --- --- --- --- --- ---");

    stdout.queue(cursor::MoveTo(canvas.width - canvas.width + 10, canvas.height - 10));
    stdout.queue(style::SetForegroundColor(style::Color::Red));
    println!("Real implementation will be here somewhere in future, hopefully");
}

/// Closes windows or exits the program
fn close(stdout: &mut Stdout, canvas: &mut settings::Canvas, runtime: &mut settings::Runtime, state: &mut settings::State) {
    if state.window_open == true && state.window_open_name == "help" {
        help_window(stdout, canvas, runtime, state);
    }

    else if state.window_open == true && state.window_open_name == "file" {
        file_window(stdout, canvas, runtime, state);
    }

    else {
        std::process::exit(0);
    }
}

/// Entry function for drawing
pub fn logic(canvas: &mut settings::Canvas, runtime: &mut settings::Runtime, state: &mut settings::State) {
    let mut stdout: Stdout = stdout();

    stdout.queue(style::SetForegroundColor(style::Color::Red));
    stdout.queue(cursor::MoveTo(0, 0));
    println!("Press 'H' or 'h' for help!");

    stdout.queue(cursor::EnableBlinking);
    stdout.queue(cursor::MoveTo(runtime.cursor_x as u16, runtime.cursor_y as u16));
    stdout.queue(style::SetForegroundColor(style::Color::Black));
    println!("\u{2588}");

    loop {
        let key: u32 = cursor_input();

        match key {
            1 => { remove_old_cursor(&mut stdout, runtime);
                    runtime.cursor_y -= 1.0;
                    runtime.cursor_y = place_new_cursor(&mut stdout, canvas, runtime);},
            2 => { remove_old_cursor(&mut stdout, runtime);
                    runtime.cursor_y += 1.0;
                    runtime.cursor_y = place_new_cursor(&mut stdout, canvas, runtime);},
            3 => { remove_old_cursor(&mut stdout, runtime);
                    runtime.cursor_x -= 1.0;
                    runtime.cursor_y =  place_new_cursor(&mut stdout, canvas, runtime);},
            4 => { remove_old_cursor(&mut stdout, runtime);
                    runtime.cursor_x += 1.0;
                    runtime.cursor_y =  place_new_cursor(&mut stdout, canvas, runtime);},
            5 => { file_window(&mut stdout, canvas, runtime, state) },
            6 => { help_window(&mut stdout, canvas, runtime, state) },
            7 => { place_blok(&mut stdout, runtime); /* all colors will be implemented soon, hopefully */
                    runtime.cursor_x -= 1.0;
                    runtime.cursor_y =  place_new_cursor(&mut stdout, canvas, runtime);},
            8 => { place_blok(&mut stdout, runtime);
                    runtime.cursor_x -= 1.0;
                    runtime.cursor_y = place_new_cursor(&mut stdout, canvas, runtime);},
            9 => { close(&mut stdout, canvas, runtime, state); },
            _ => { if (key >= 10 && key <= 20) { runtime.color = return_color(key); } }
        }
    }
}


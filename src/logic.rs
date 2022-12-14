use std::collections::BTreeMap;
use std::path::Path;
use std::vec;
use std::{
    fs, io,
    io::{prelude::*, stdout, Read, Stdout, Write},
};

use crossterm::style::SetStyle;
use crossterm::{
    cursor,
    style::{self, Color, SetForegroundColor},
    terminal, QueueableCommand,
};
use termios::{tcsetattr, Termios, ECHO, ICANON, TCSANOW};

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
    C0,
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
    WHITE,
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
    let mut buffer = [0; 1];

    stdout.lock().flush().unwrap();
    reader.read_exact(&mut buffer).unwrap();
    tcsetattr(stdin, TCSANOW, &termios).unwrap();

    return buffer[0] as char;
}

/// Returns enum value of keys which user enter
fn cursor_input() -> u32 {
    let ch = read_user_input_character();
    let ch_u32: u32 = ch as u32;

    match ch_u32 {
        119 | 87 => return KEY::W as u32,
        115 | 83 => return KEY::S as u32,
        97 | 65 => return KEY::A as u32,
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

/// Returns color in style::Color based on number (10 - 18)
fn return_color(color: u32) -> Color {
    let mut crossterm_color = style::Color::Black;
    match color {
        10 => crossterm_color = style::Color::Black,
        11 => crossterm_color = style::Color::Grey,
        12 => crossterm_color = style::Color::Red,
        13 => crossterm_color = style::Color::Green,
        14 => crossterm_color = style::Color::Blue,
        15 => {
            crossterm_color = style::Color::Rgb {
                r: 0,
                g: 255,
                b: 255,
            }
        }
        16 => crossterm_color = style::Color::Yellow,
        17 => {
            crossterm_color = style::Color::Rgb {
                r: 237,
                g: 116,
                b: 24,
            }
        }
        18 => crossterm_color = style::Color::White,
        _ => crossterm_color = style::Color::Black,
    }

    return crossterm_color;
}

/// Returns color in u32 type
fn return_color_int(color: style::Color) -> u32 {
    let mut color_to_return = 10;

    match color {
        style::Color::Black => color_to_return = 10,
        style::Color::Grey => color_to_return = 11,
        style::Color::Red => color_to_return = 12,
        style::Color::Green => color_to_return = 13,
        style::Color::Blue => color_to_return = 14,
        style::Color::Rgb {
            r: 0,
            g: 255,
            b: 255,
        } => color_to_return = 15,
        style::Color::Yellow => color_to_return = 16,
        style::Color::Rgb {
            r: 237,
            g: 116,
            b: 24,
        } => color_to_return = 17,
        style::Color::White => color_to_return = 18,
        _ => color_to_return = 10,
    }

    return color_to_return;
}

/// Moves cursor (X or Y axis) by one based on last pressed key (W, S, A, D)
fn move_cursor_blkey(runtime: &mut settings::Runtime) {
    match runtime.last_pressed_key {
        1 => runtime.cursor_y -= 1.0,
        2 => runtime.cursor_y += 1.0,
        3 => runtime.cursor_x -= 1.0,
        4 => runtime.cursor_x += 1.0,
        _ => runtime.cursor_x -= 1.0,
    }
}

/// Removing old cursor, if painting is there, redraw it
fn remove_old_cursor(stdout: &mut Stdout, runtime: &mut settings::Runtime) {
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

/// Placing new cursor
fn place_new_cursor(
    stdout: &mut Stdout,
    canvas: &mut settings::Canvas,
    runtime: &mut settings::Runtime,
) -> f64 {
    if (runtime.cursor_y as u16) != (canvas.height - 2) {
        stdout.queue(cursor::MoveTo(
            runtime.cursor_x as u16,
            runtime.cursor_y as u16,
        ));
        stdout.queue(style::SetForegroundColor(style::Color::Black));
        println!("\u{2588}");
    } else {
        remove_old_cursor(stdout, runtime);

        runtime.cursor_y -= 2.0;
        runtime.cursor_y = place_new_cursor(stdout, canvas, runtime);
    }

    return runtime.cursor_y;
}

/// Placing blok
fn place_blok(stdout: &mut Stdout, runtime: &mut settings::Runtime) {
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

/// Small "window" with shows help information
fn help_window(
    stdout: &mut Stdout,
    canvas: &mut settings::Canvas,
    runtime: &mut settings::Runtime,
    state: &mut settings::State,
) {
    if state.window_open == true && state.window_open_name == "help" {
        /* close */
        state.window_open = false;

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
        stdout.queue(style::SetForegroundColor(style::Color::Black));
        println!("\u{2588}");

        return;
    }

    state.window_open = true;
    state.window_open_name = "help".to_string();

    stdout.queue(style::SetForegroundColor(style::Color::DarkGreen));
    stdout.queue(cursor::MoveTo(
        canvas.width - canvas.width + 5,
        canvas.height - 18,
    ));
    println!("--- --- --- --- --- --- --- --- --- --- --- ---");

    let mut i = 17;
    while i != 2 {
        stdout.queue(cursor::MoveTo(
            canvas.width - canvas.width + 4,
            canvas.height - i,
        ));
        println!("|");
        i -= 1;
    }

    let mut j = 17;
    while j != 2 {
        stdout.queue(cursor::MoveTo(
            canvas.width - canvas.width + 52,
            canvas.height - j,
        ));
        println!("|");
        j -= 1;
    }

    stdout.queue(cursor::MoveTo(
        canvas.width - canvas.width + 5,
        canvas.height - 3,
    ));
    println!("--- --- --- --- --- --- --- --- --- --- --- ---");

    stdout.queue(cursor::MoveTo(
        canvas.width - canvas.width + 6,
        canvas.height - 17,
    ));
    stdout.queue(style::SetForegroundColor(style::Color::Red));
    println!("Keyboard shortcuts: ");
    println!("\t W - move cursor up");
    println!("\t S - move cursor down");
    println!("\t A - move cursor left");
    println!("\t D - move cursor right");
    println!("\t F - open 'file window'");
    println!("\t H - open 'help window', this one");
    println!("\t P - place block or select a row in menu");
    println!("\t E - erase block");
    println!("\t Q - exit a program or close a window");
    println!("\t 1 - 9 - change color");
    println!(" ");
    println!("\t Made with Rust and thanks to StjepanBM1");
}

/// Small "window" where we should have following options:
/// Open picture
/// Save picture
fn file_window(
    stdout: &mut Stdout,
    canvas: &mut settings::Canvas,
    runtime: &mut settings::Runtime,
    state: &mut settings::State,
) {
    if state.window_open == true && state.window_open_name == "file" {
        /* close */
        state.window_open = false;

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
        stdout.queue(style::SetForegroundColor(style::Color::Black));
        println!("\u{2588}");

        return;
    }

    state.window_open = true;
    state.window_open_name = "file".to_string();

    stdout.queue(style::SetForegroundColor(style::Color::DarkGreen));
    stdout.queue(cursor::MoveTo(
        canvas.width - canvas.width + 5,
        canvas.height - 16,
    ));
    println!("--- --- --- --- --- --- --- --- --- ---");

    let mut i = 15;
    while i != 3 {
        stdout.queue(cursor::MoveTo(
            canvas.width - canvas.width + 4,
            canvas.height - i,
        ));
        println!("|");
        i -= 1;
    }

    let mut j = 15;
    while j != 3 {
        stdout.queue(cursor::MoveTo(
            canvas.width - canvas.width + 44,
            canvas.height - j,
        ));
        println!("|");
        j -= 1;
    }

    stdout.queue(cursor::MoveTo(
        canvas.width - canvas.width + 5,
        canvas.height - 3,
    ));
    println!("--- --- --- --- --- --- --- --- --- ---");

    stdout.queue(cursor::MoveTo(
        canvas.width - canvas.width + 10,
        canvas.height - 10,
    ));
    stdout.queue(style::SetForegroundColor(style::Color::Red));
    println!(" ");
    println!("\t Open text file");
    println!("\t Save as text file");
    println!("\t Open .png file (todo)");
    println!("\t Save as .png (todo)");
}

/// Returns content of text file
fn content_in_file(file_menu: &mut settings::FileMenu, filename: &str) -> std::io::Result<()> {
    let cwd = std::env::current_dir().unwrap();
    /*let cwd_str = cwd.into_os_string().into_string();
    println!("{:?}", cwd_str);*/

    let mut file = fs::File::open(filename)?;
    file.read_to_string(&mut file_menu.file_content)?;

    Ok(())
}

/// Writes content to file
fn write_to_file(new_file: bool, filename: &str, string: String) -> std::io::Result<()> {
    if new_file {
        let mut file = fs::File::create(filename)?;
        file.write_all(string.as_bytes())?;
    } else {
        let mut file2 = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(filename)
            .unwrap();

        if let Err(e) = write!(file2, "{}", string) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }

    Ok(())
}

/// Actions from menu from file_window function
fn file_window_actions(
    stdout: &mut Stdout,
    canvas: &mut settings::Canvas,
    runtime: &mut settings::Runtime,
    state: &mut settings::State,
    file_menu: &mut settings::FileMenu,
) {
    let max_x = 40;

    /*
     * Open text file
     * Save text file
     * Open .png file
     * Save as png file
     */
    if (runtime.cursor_x as u16 >= 23
        && runtime.cursor_x as u16 <= max_x
        && runtime.cursor_y as u16 == canvas.height - 9)
    {
        content_in_file(file_menu, "painting.txt");

        if (file_menu.file_content == " ") {
            let strings: [&str; 6] = [
                "Make sure you have `painting.txt` in",
                "working folder!",
                "If you are running directly from GitHub",
                "repo, enter repo root folder",
                "and there create `painting.txt` file.",
                " ",
            ];
            let mut i = 15;
            let mut j = 0;

            while (i >= 10) {
                stdout.queue(cursor::MoveTo(5, canvas.height - i));
                stdout.queue(style::SetForegroundColor(style::Color::DarkYellow));
                println!("{}", strings[j]);

                i -= 1;
                j += 1;
            }
        } else {
            let vec: Vec<u32> = file_menu
                .file_content
                .split_whitespace()
                .map(|s| s.parse().expect("Parsing error!"))
                .collect();

            let current_x = runtime.cursor_x;
            let current_y = runtime.cursor_y;
            let current_color = runtime.color;

            let mut temp_x = current_x;
            let mut temp_y = current_y;
            let mut temp_color = current_color;

            let mut iterator = 1;

            for i in vec {
                match iterator {
                    1 => {
                        temp_x = i as f64;
                        iterator += 1;
                    }
                    2 => {
                        temp_y = i as f64;
                        iterator += 1;
                    }
                    3 => {
                        temp_color = return_color(i);

                        runtime.cursor_x = temp_x;
                        runtime.cursor_y = temp_y;
                        runtime.color = temp_color;

                        place_blok(stdout, runtime);

                        runtime.cursor_x = current_x;
                        runtime.cursor_y = current_y;
                        runtime.color = current_color;

                        temp_x = 0 as f64;
                        temp_y = 0 as f64;
                        temp_color = return_color(10);

                        iterator = 1;
                    }

                    _ => { /* ignore */ }
                }
            }
        }
    } else if (runtime.cursor_x as u16 >= 26
        && runtime.cursor_x as u16 <= max_x
        && runtime.cursor_y as u16 == canvas.height - 8)
    {
        if Path::new("painting.txt").exists() {
            fs::rename("painting.txt", "painting2.txt")
                .expect("Couldn't rename `painting.txt` to `painting2.txt`!");
        }

        let mut new_file = true;
        let mut placed = runtime.placed.clone();
        for (k, v) in placed {
            let mut options = fs::OpenOptions::new();
            let mut file = options.write(true).open("painting.txt");

            let mut string: String = k.0.to_string()
                + " "
                + &k.1.to_string()
                + " "
                + &return_color_int(v).to_string()
                + "\n";

            write_to_file(new_file, "painting.txt", string);
            new_file = false;
        }
    } else if (runtime.cursor_x as u16 >= 22
        && runtime.cursor_x as u16 <= max_x
        && runtime.cursor_y as u16 == canvas.height - 7)
    {
        stdout.queue(cursor::MoveTo(
            canvas.width - canvas.width + 10,
            canvas.height - 11,
        ));
        stdout.queue(style::SetForegroundColor(style::Color::DarkYellow));
        println!("On TODO list!");
    } else if (runtime.cursor_x as u16 >= 28
        && runtime.cursor_x as u16 <= max_x
        && runtime.cursor_y as u16 == canvas.height - 6)
    {
        stdout.queue(cursor::MoveTo(
            canvas.width - canvas.width + 10,
            canvas.height - 11,
        ));
        stdout.queue(style::SetForegroundColor(style::Color::DarkYellow));
        println!("On TODO list!");
    }
}

/// Closes windows or exits the program
fn close(
    stdout: &mut Stdout,
    canvas: &mut settings::Canvas,
    runtime: &mut settings::Runtime,
    state: &mut settings::State,
) {
    if state.window_open == true && state.window_open_name == "help" {
        help_window(stdout, canvas, runtime, state);
    } else if state.window_open == true && state.window_open_name == "file" {
        file_window(stdout, canvas, runtime, state);
    } else {
        std::process::exit(0);
    }
}

/// Entry function for drawing
pub fn logic(
    canvas: &mut settings::Canvas,
    runtime: &mut settings::Runtime,
    state: &mut settings::State,
    file_menu: &mut settings::FileMenu,
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
    stdout.queue(style::SetForegroundColor(style::Color::Black));
    println!("\u{2588}");

    loop {
        let key: u32 = cursor_input();

        match key {
            1 => {
                remove_old_cursor(&mut stdout, runtime);
                runtime.cursor_y -= 1.0;
                runtime.cursor_y = place_new_cursor(&mut stdout, canvas, runtime);
                runtime.last_pressed_key = 1;
            }
            2 => {
                remove_old_cursor(&mut stdout, runtime);
                runtime.cursor_y += 1.0;
                runtime.cursor_y = place_new_cursor(&mut stdout, canvas, runtime);
                runtime.last_pressed_key = 2;
            }
            3 => {
                remove_old_cursor(&mut stdout, runtime);
                runtime.cursor_x -= 1.0;
                runtime.cursor_y = place_new_cursor(&mut stdout, canvas, runtime);
                runtime.last_pressed_key = 3;
            }
            4 => {
                remove_old_cursor(&mut stdout, runtime);
                runtime.cursor_x += 1.0;
                runtime.cursor_y = place_new_cursor(&mut stdout, canvas, runtime);
                runtime.last_pressed_key = 4;
            }
            5 => file_window(&mut stdout, canvas, runtime, state),
            6 => help_window(&mut stdout, canvas, runtime, state),
            7 => {
                if (state.window_open == true && state.window_open_name == "file") {
                    file_window_actions(&mut stdout, canvas, runtime, state, file_menu);
                } else {
                    place_blok(&mut stdout, runtime);
                    move_cursor_blkey(runtime);
                    runtime.cursor_y = place_new_cursor(&mut stdout, canvas, runtime);
                }
            }
            8 => {
                let current_color = runtime.color;
                runtime.color = style::Color::White;

                place_blok(&mut stdout, runtime);
                runtime.color = current_color;

                move_cursor_blkey(runtime);
                runtime.cursor_y = place_new_cursor(&mut stdout, canvas, runtime);
            }
            9 => {
                close(&mut stdout, canvas, runtime, state);
            }
            _ => {
                if (key >= 10 && key <= 18) {
                    runtime.color = return_color(key);
                }
            }
        }
    }
}

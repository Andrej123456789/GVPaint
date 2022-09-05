use std::{{io, io::{Read, Write, Stdout, stdout}}, thread, time::Duration};
use crossterm::{terminal, style, cursor, QueueableCommand};
use termios::{Termios, TCSANOW, ECHO, ICANON, tcsetattr};

static mut ALREADY: bool = false;

enum KEY {
    ERROR,
    W,
    S,
    A,
    D,
    PLACE,
    ERASE
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
        113 | 81 => return KEY::PLACE as u32,
        101 | 69 => return KEY::ERASE as u32,
        _ => return KEY::ERROR as u32,
    }
}

fn get_color(stdout: &mut Stdout) -> u32 {
    return COLOR::RED as u32; /* all colors will be implemented soon, hopefully */
}

fn remove_old_cursor(stdout: &mut Stdout, cursor_x: f64, cursor_y: f64) {
    stdout.queue(cursor::MoveTo(cursor_x as u16, cursor_y as u16));
    stdout.queue(style::SetForegroundColor(style::Color::White));
    println!("\u{2588}");
}

fn place_new_cursor(stdout: &mut Stdout, cursor_x: f64, cursor_y: f64) {
    stdout.queue(cursor::MoveTo(cursor_x as u16, cursor_y as u16));
    stdout.queue(style::SetForegroundColor(style::Color::Black));
    println!("\u{2588}");
}

fn place_blok(stdout: &mut Stdout, cursor_x: f64, cursor_y: f64, color: u32) {
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

    stdout.queue(cursor::MoveTo(cursor_x as u16, cursor_y as u16));
    stdout.queue(style::SetForegroundColor(crossterm_color));
    println!("\u{2588}");

    cursor_x -= 1.0;
    place_new_cursor(&mut stdout, cursor_x, cursor_y);
}

fn logic(width: u16, height:u16) {
    let mut stdout: Stdout = stdout();

    let mut cursor_x: f64 = (width as f64) / (2.2 as f64);
    let mut cursor_y: f64 = (height as f64) / (2.2 as f64);

    stdout.queue(style::SetForegroundColor(style::Color::Red));
    stdout.queue(cursor::MoveTo(0, 0));
    println!("File\tExit");

    stdout.queue(cursor::EnableBlinking);
    stdout.queue(cursor::MoveTo(cursor_x as u16, cursor_y as u16));
    stdout.queue(style::SetForegroundColor(style::Color::Black));
    println!("\u{2588}");

    loop {
        let key: u32 = cursor_input();
        
        if key == 1 {
            remove_old_cursor(&mut stdout, cursor_x, cursor_y);
            cursor_y -= 1.0;
            place_new_cursor(&mut stdout, cursor_x, cursor_y);
        }
    
        else if key == 2 {
            remove_old_cursor(&mut stdout, cursor_x, cursor_y);
            cursor_y += 1.0;
            place_new_cursor(&mut stdout, cursor_x, cursor_y);
        }
    
        else if key == 3 {
            remove_old_cursor(&mut stdout, cursor_x, cursor_y);
            cursor_x -= 1.0;
            place_new_cursor(&mut stdout, cursor_x, cursor_y);
        }
    
        else if key == 4 {
            remove_old_cursor(&mut stdout, cursor_x, cursor_y);
            cursor_x += 1.0;
            place_new_cursor(&mut stdout, cursor_x, cursor_y);
        }

        else if key == 5 { /* PLACE */
            place_blok(&mut stdout, cursor_x, cursor_y, 2); /* all colors will be implemented soon, hopefully */
        }

        else if key == 6 { /* ERASE */
            place_blok(&mut stdout, cursor_x, cursor_y, 11);
        }
    
        else {
            /* ignore */
        }
    }
}

pub fn menu(width: u16, height:u16)
{
    logic(width, height);
}

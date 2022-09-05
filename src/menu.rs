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
    ENTER
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

fn cursor_input() ->  u32 {
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

    match (ch_u32) {
        119 => return KEY::W as u32,
        115 => return KEY::S as u32,
        97 => return KEY::A as u32,
        100 => return KEY::D as u32,
        0 => return KEY::ENTER as u32,
        _ => return KEY::ERROR as u32,
    }

    return KEY::ERROR as u32;
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
            stdout.queue(cursor::MoveTo(cursor_x as u16, cursor_y as u16));
            stdout.queue(style::SetForegroundColor(style::Color::White));
            println!("\u{2588}");

            cursor_y -= 1.0;
            stdout.queue(cursor::MoveTo(cursor_x as u16, cursor_y as u16));
            stdout.queue(style::SetForegroundColor(style::Color::Black));
            println!("\u{2588}");
        }
    
        else if key == 2 {
            stdout.queue(cursor::MoveTo(cursor_x as u16, cursor_y as u16));
            stdout.queue(style::SetForegroundColor(style::Color::White));
            println!("\u{2588}");

            cursor_y += 1.0;
            stdout.queue(cursor::MoveTo(cursor_x as u16, cursor_y as u16));
            stdout.queue(style::SetForegroundColor(style::Color::Black));
            println!("\u{2588}");
        }
    
        else if key == 3 {
            stdout.queue(cursor::MoveTo(cursor_x as u16, cursor_y as u16));
            stdout.queue(style::SetForegroundColor(style::Color::White));
            println!("\u{2588}");

            cursor_x -= 1.0;
            stdout.queue(cursor::MoveTo(cursor_x as u16, cursor_y as u16));
            stdout.queue(style::SetForegroundColor(style::Color::Black));
            println!("\u{2588}");
        }
    
        else if key == 4 {
            stdout.queue(cursor::MoveTo(cursor_x as u16, cursor_y as u16));
            stdout.queue(style::SetForegroundColor(style::Color::White));
            println!("\u{2588}");
            
            cursor_x += 1.0;
            stdout.queue(cursor::MoveTo(cursor_x as u16, cursor_y as u16)); 
            stdout.queue(style::SetForegroundColor(style::Color::Black));
            println!("\u{2588}");
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

use std::{{io::{Write, stdout}}, thread, time::Duration, io::Stdout};
use crossterm::{terminal, style, cursor, QueueableCommand};

static mut ALREADY: bool = false;
enum KEY {
    ERROR,
    W,
    S,
    A,
    D,
    ENTER
}

///
/// Reads a character from user input. If multiple characters are given,
/// character at first index is returned. In any problematic cases, return
/// an asterisk (*).
/// Credits: https://github.com/dcode-youtube/hangman-rust/blob/master/src/main.rs
fn read_user_input_character() -> char {
    let mut user_input = String::new();

    match std::io::stdin().read_line(&mut user_input) {
        Ok(_) => {
            match user_input.chars().next() {
                Some(c) => { return c; }
                None => { return '*'; }
            }
        }
        Err(_) => { return '*'; }
    }
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
            stdout.queue(style::SetForegroundColor(style::Color::White));
            println!("\u{2588}");

            cursor_y += 1.0;
            stdout.queue(cursor::MoveTo(cursor_x as u16, cursor_y as u16));
            stdout.queue(style::SetForegroundColor(style::Color::Black));
            println!("\u{2588}");
        }
    
        else if key == 2 {
            stdout.queue(style::SetForegroundColor(style::Color::White));
            println!("\u{2588}");

            cursor_y -= 1.0;
            stdout.queue(cursor::MoveTo(cursor_x as u16, cursor_y as u16));
            stdout.queue(style::SetForegroundColor(style::Color::Black));
            println!("\u{2588}");
        }
    
        else if key == 3 {
            stdout.queue(style::SetForegroundColor(style::Color::White));
            println!("\u{2588}");

            cursor_x -= 1.0;
            stdout.queue(cursor::MoveTo(cursor_x as u16, cursor_y as u16));
            stdout.queue(style::SetForegroundColor(style::Color::Black));
            println!("\u{2588}");
        }
    
        else if key == 4 {
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

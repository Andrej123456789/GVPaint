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
    println!("Enter a char: ");
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

fn logic() {
    let mut stdout: Stdout = stdout();

    stdout.queue(style::SetForegroundColor(style::Color::Red));
    stdout.queue(cursor::MoveTo(0, 0));
    println!("File");

    loop {
        let key: u32 = cursor_input();
        println!("Key pressed: {}", key);
    }
}

pub fn menu()
{
    logic();
}

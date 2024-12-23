use std::{
    fs,
    io::{Read, Stdout, Write},
    path::Path
};

use crossterm::{
    cursor,
    style,
    QueueableCommand,
};

use image::{
    open, GenericImageView,
    Pixel, Rgb, RgbImage,
};

use crate::paint;
use crate::variables;

/// Window showing how to use GVPaint
pub fn help_window(
    stdout: &mut Stdout,
    canvas: &mut variables::Canvas,
    runtime: &mut variables::Runtime,
    state: &mut variables::State,
) {
    if state.window_open == true && state.window_open_name == "help" {
        /* close */
        state.window_open = false;
        paint::redraw_canvas(stdout, runtime);

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
    println!("\t P - place a block");
    println!("\t E - erase a block");
    println!("\t Q - exit a program or close a window");
    println!("\t 1 - 9 - change color");
    println!(" ");
    println!("\t Made with Rust and thanks to StjepanBM1");
}

/// Window for opening and saving files
pub fn file_window(
    stdout: &mut Stdout,
    canvas: &mut variables::Canvas,
    runtime: &mut variables::Runtime,
    state: &mut variables::State,
) {
    if state.window_open == true && state.window_open_name == "file" {
        /* close */
        state.window_open = false;
        paint::redraw_canvas(stdout, runtime);

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

/// Return a content of a file
fn content_in_file(file_menu: &mut variables::FileMenu, filename: &str) -> std::io::Result<()> {
    let cwd = std::env::current_dir().unwrap();
    /*let cwd_str = cwd.into_os_string().into_string();
    println!("{:?}", cwd_str);*/

    let mut file = fs::File::open(filename)?;
    file.read_to_string(&mut file_menu.file_content)?;

    Ok(())
}

/// Write content to a file
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

/// Convert crossterm::style::Color to image::Rgb<u8>
fn crossterm_to_image_color(color: style::Color) -> Rgb<u8> {
    match color {
        style::Color::Black => Rgb([0, 0, 0]),
        style::Color::DarkGrey => Rgb([64, 64, 64]),
        style::Color::Grey => Rgb([128, 128, 128]),
        style::Color::White => Rgb([255, 255, 255]),
        style::Color::Red => Rgb([255, 0, 0]),
        style::Color::DarkRed => Rgb([128, 0, 0]),
        style::Color::Green => Rgb([0, 255, 0]),
        style::Color::DarkGreen => Rgb([0, 128, 0]),
        style::Color::Blue => Rgb([0, 0, 255]),
        style::Color::DarkBlue => Rgb([0, 0, 128]),
        style::Color::Yellow => Rgb([255, 255, 0]),
        style::Color::DarkYellow => Rgb([128, 128, 0]),
        style::Color::Magenta => Rgb([255, 0, 255]),
        style::Color::DarkMagenta => Rgb([128, 0, 128]),
        style::Color::Cyan => Rgb([0, 255, 255]),
        style::Color::DarkCyan => Rgb([0, 128, 128]),
        style::Color::Rgb { r, g, b } => Rgb([r, g, b]),
        _ => Rgb([0, 0, 0])
    }
}

/// Convert image::Rgb<u8> to crossterm::style::Color
fn rgb_to_crossterm_color(rgb: Rgb<u8>) -> style::Color {
    style::Color::Rgb {
        r: rgb[0],
        g: rgb[1],
        b: rgb[2],
    }
}
/// Actions for file window
pub fn file_window_actions(
    stdout: &mut Stdout,
    canvas: &mut variables::Canvas,
    runtime: &mut variables::Runtime,
    state: &mut variables::State,
    file_menu: &mut variables::FileMenu,
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
                        unsafe {
                            temp_color = paint::return_color(std::mem::transmute(i as u8));
                        }

                        runtime.cursor_x = temp_x;
                        runtime.cursor_y = temp_y;
                        runtime.color = temp_color;

                        paint::place_blok(stdout, runtime);

                        runtime.cursor_x = current_x;
                        runtime.cursor_y = current_y;
                        runtime.color = current_color;

                        temp_x = 0 as f64;
                        temp_y = 0 as f64;
                        temp_color = style::Color::Black;

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

            let kcolor = paint::return_color_int(v);
            let icolor = kcolor as u32;

            let mut string: String = k.0.to_string()
                + " "
                + &k.1.to_string()
                + " "
                + &icolor.to_string()
                + "\n";

            write_to_file(new_file, "painting.txt", string);
            new_file = false;
        }
    } else if (runtime.cursor_x as u16 >= 22
        && runtime.cursor_x as u16 <= max_x
        && runtime.cursor_y as u16 == canvas.height - 7)
    {
        let img = open("painting.png").expect("Failed to open image");
        let rgb_img = img.to_rgb8();

        for (x, y, pixel) in rgb_img.enumerate_pixels() {
            let color = pixel.to_rgb(); // Extract the color as an RGB value

            runtime.cursor_x = x as f64;
            runtime.cursor_y = y as f64;
            runtime.color = rgb_to_crossterm_color(color.to_rgb());

            paint::place_blok(stdout, runtime);
        }

        runtime.cursor_color = style::Color::DarkRed;
    } else if (runtime.cursor_x as u16 >= 28
        && runtime.cursor_x as u16 <= max_x
        && runtime.cursor_y as u16 == canvas.height - 6)
    {
        let mut img = RgbImage::new(canvas.width as u32, canvas.height as u32);

        for (k, v) in runtime.placed.clone() {
            img.put_pixel(k.0, k.1, crossterm_to_image_color(v));
        }

        img.save("painting.png").expect("Failed to save image");
    }
}

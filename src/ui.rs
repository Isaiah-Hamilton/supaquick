extern crate termion;

use termion::color;
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use std::io::{self, stdin, stdout, Write};

pub fn input(text: &str, placeholder: &str, default: &str) -> String {
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut input = String::new();
    let mut ph = true;

    print!("{}", text);
    io::stdout().flush().unwrap();

    if !placeholder.is_empty() {
        print!("\x1b[90m"); // light grey
        print!("{}{}", placeholder, color::Fg(color::Reset));
        io::stdout().flush().unwrap();
        Cursor::shift("left", placeholder.len() as u8);
    }

    for c in stdin().keys() {
        match c.unwrap() {
            Key::Char('\n') => {
                if input.is_empty() && !default.is_empty() {
                    input.push_str(default);
                    break;
                } else if input.is_empty() && default.is_empty() {
                    // TODO: make input have a required parm instead
                    break;
                } else {
                    break;
                }
            }
            Key::Char(c) => {
                print!("{}", c);
                input.push_str(c.to_string().as_str());
                if !input.is_empty() && !placeholder.is_empty() && ph {
                    // TODO: refactor this
                    print!("\x1B[{}C", placeholder.len()); // go forward the length of placeholder
                    Cursor::backspace(placeholder.len() as u8);
                    ph = false;
                }
            }
            Key::Backspace => {
                if !input.is_empty() {
                    Cursor::backspace(1);
                    input.pop();
                }
                if input.is_empty() && !placeholder.is_empty() {
                    // TODO: fix this
                    print!("\x1b[90m"); // light grey
                    print!("{}{}", placeholder, color::Fg(color::Reset));
                    io::stdout().flush().unwrap();
                    Cursor::shift("left", placeholder.len() as u8);
                    ph = true;
                }
            }
            _ => {}
        }
        stdout.flush().unwrap();
    }

    Cursor::beginning();
    Cursor::new_line();

    input.trim().to_string()
}

pub fn list(text: &str, frameworks: &[&str]) -> String {
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut selected_index = 0;

    Cursor::hide();

    loop {
        println!("{}", text);
        Cursor::beginning();

        for (index, f) in frameworks.iter().enumerate() {
            if index == selected_index {
                println!(
                    "{}>{} {}. {}",
                    color::Fg(color::Green),
                    color::Fg(color::Reset),
                    index + 1,
                    f
                );
                Cursor::beginning();
            } else {
                println!("  {}. {}", index + 1, f);
                Cursor::beginning();
            }
        }

        for c in io::stdin().events() {
            match c.unwrap() {
                Event::Key(Key::Char('j')) | Event::Key(Key::Down) => {
                    selected_index = (selected_index + 1) % frameworks.len();
                    break;
                }
                Event::Key(Key::Char('k')) | Event::Key(Key::Up) => {
                    if selected_index > 0 {
                        selected_index -= 1;
                    } else {
                        selected_index = frameworks.len() - 1;
                    }
                    break;
                }
                Event::Key(Key::Char('q')) => {
                    println!("you quit");
                    Cursor::show();
                    return String::new();
                }
                Event::Key(Key::Char('\n')) => {
                    Cursor::show();
                    return frameworks[selected_index].to_string();
                }
                _ => {}
            }
            stdout.flush().unwrap();
        }

        print!("\x1B[{}A", frameworks.len() + 1); // Move the cursor up by the list text
        io::stdout().flush().unwrap();
    }
}

pub fn logo() {
    println!(
        r#"
 ________  ___  ___  ________  ________  ________  ___  ___  ___  ________  ___  __
|\   ____\|\  \|\  \|\   __  \|\   __  \|\   __  \|\  \|\  \|\  \|\   ____\|\  \|\  \
\ \  \___|\ \  \\\  \ \  \|\  \ \  \|\  \ \  \|\  \ \  \\\  \ \  \ \  \___|\ \  \/  /|_
 \ \_____  \ \  \\\  \ \   ____\ \   __  \ \  \\\  \ \  \\\  \ \  \ \  \    \ \   ___  \
  \|____|\  \ \  \\\  \ \  \___|\ \  \ \  \ \  \\\  \ \  \\\  \ \  \ \  \____\ \  \\ \  \
    ____\_\  \ \_______\ \__\    \ \__\ \__\ \_____  \ \_______\ \__\ \_______\ \__\\ \__\
   |\_________\|_______|\|__|     \|__|\|__|\|___| \__\|_______|\|__|\|_______|\|__| \|__|
   \|_________|                                   \|__|
"#
    );
    io::stdout().flush().unwrap();
    println!("Press ESC to exit\n");
}

struct Cursor;

impl Cursor {
    fn show() {
        print!("\x1b[?25h");
        io::stdout().flush().unwrap();
    }

    fn hide() {
        print!("\x1b[?25l");
        io::stdout().flush().unwrap();
    }

    fn shift(direction: &str, count: u8) {
        if direction == "left" {
            let mut i = 0;
            while i != count {
                print!("\x08");
                i += 1;
            }
            io::stdout().flush().unwrap();
        }
    }

    fn backspace(count: u8) {
        let mut i = 0;
        while i != count {
            print!("\x08 \x08");
            i += 1;
        }
        io::stdout().flush().unwrap();
    }

    fn beginning() {
        print!("\r");
        io::stdout().flush().unwrap();
    }

    fn new_line() {
        println!();
    }
}

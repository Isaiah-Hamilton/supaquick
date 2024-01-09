extern crate termion;

use termion::clear::CurrentLine;
use termion::color;
use termion::cursor::{BlinkingBlock, Down, Hide, Left, Restore, Right, Show, Up};
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
        Cursor::left(placeholder.len() as u16);
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
                    Cursor::right(placeholder.len() as u16);
                    Cursor::backspace(placeholder.len() as u16);
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
                    Cursor::left(placeholder.len() as u16);
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

pub fn list(text: &str, options: &[&str]) -> String {
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut selected_index = 0;

    Cursor::hide();

    loop {
        println!("{}", text);
        Cursor::beginning();

        for (index, f) in options.iter().enumerate() {
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
                    selected_index = (selected_index + 1) % options.len();
                    break;
                }
                Event::Key(Key::Char('k')) | Event::Key(Key::Up) => {
                    if selected_index > 0 {
                        selected_index -= 1;
                    } else {
                        selected_index = options.len() - 1;
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
                    return options[selected_index].to_string();
                }
                _ => {}
            }
            stdout.flush().unwrap();
        }

        Cursor::up(options.len() as u16 + 1);
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

pub fn option(text: &str, options: &[&str]) -> String {
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut selected_index = 0;

    Cursor::hide();

    loop {
        print!("{}", text);
        io::stdout().flush().unwrap();

        for (index, o) in options.iter().enumerate() {
            if index == selected_index {
                print!(
                    "{}{}{}",
                    color::Fg(color::Green),
                    o,
                    color::Fg(color::Reset),
                )
            } else {
                print!("{}", o);
            }
            if index + 1 != options.len() {
                print!(" / ");
            }
            io::stdout().flush().unwrap();
        }

        for c in io::stdin().events() {
            match c.unwrap() {
                Event::Key(Key::Char('h')) | Event::Key(Key::Left) => {
                    selected_index = (selected_index + 1) % options.len();
                    break;
                }
                Event::Key(Key::Char('l')) | Event::Key(Key::Right) => {
                    if selected_index > 0 {
                        selected_index -= 1;
                    } else {
                        selected_index = options.len() - 1;
                    }
                    break;
                }
                Event::Key(Key::Char('\n')) => {
                    Cursor::show();
                    Cursor::new_line();
                    Cursor::beginning();
                    return options[selected_index].to_string();
                }
                _ => {}
            }
            stdout.flush().unwrap();
        }
        Cursor::clear_line();
        Cursor::beginning();
    }
}

pub fn footer(name: &str) {
    Cursor::new_line();
    println!(
        "{}Success!{} {} has been created",
        color::Fg(color::Green),
        color::Fg(color::Reset),
        name
    );
}

pub struct Cursor;

impl Cursor {
    pub fn show() {
        print!("{}", Show);
        io::stdout().flush().unwrap();
    }

    pub fn hide() {
        print!("{}", Hide);
        io::stdout().flush().unwrap();
    }

    pub fn blink() {
        print!("{}", BlinkingBlock);
        io::stdout().flush().unwrap();
    }

    pub fn restore() {
        print!("{}", Restore);
        io::stdout().flush().unwrap();
    }

    pub fn clear_line() {
        print!("{}", CurrentLine);
        io::stdout().flush().unwrap();
    }

    pub fn left(count: u16) {
        print!("{}", Left(count));
        io::stdout().flush().unwrap();
    }

    pub fn down(count: u16) {
        print!("{}", Down(count));
        io::stdout().flush().unwrap();
    }

    pub fn up(count: u16) {
        print!("{}", Up(count));
        io::stdout().flush().unwrap();
    }

    pub fn right(count: u16) {
        print!("{}", Right(count));
        io::stdout().flush().unwrap();
    }

    pub fn backspace(count: u16) {
        let mut i = 0;
        while i != count {
            print!("\x08 \x08");
            i += 1;
        }
        io::stdout().flush().unwrap();
    }

    pub fn beginning() {
        print!("\r");
        io::stdout().flush().unwrap();
    }

    pub fn new_line() {
        println!();
    }
}

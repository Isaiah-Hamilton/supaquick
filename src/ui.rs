extern crate termion;

use termion::clear::CurrentLine;
use termion::color;
use termion::cursor::{BlinkingBlock, Down, Hide, Left, Restore, Right, Show, SteadyBlock, Up};
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use std::io::{self, stdin, stdout, Write};

/// Obtains user input from the command line, with support for a default value and placeholder text.
///
/// # Arguments
///
/// * `text` - The prompt text displayed to the user.
/// * `placeholder` - Optional placeholder text displayed before user input.
/// * `default` - Optional default value for the input field.
///
/// # Returns
///
/// A String containing the user's input.
pub fn input(text: &str, placeholder: &str, default: &str) -> String {
    // Enable raw mode for stdout
    let mut stdout = stdout().into_raw_mode().unwrap();

    // Initialize variables
    let mut input = String::new();
    let mut placeholder_visible = true;

    // Display prompt text
    print!("{}", text);
    io::stdout().flush().unwrap();

    // Display placeholder if provided
    if !placeholder.is_empty() {
        print!(
            "\x1b[90m{}{}", // light grey
            placeholder,
            termion::color::Fg(termion::color::Reset)
        );
        io::stdout().flush().unwrap();
        Cursor::left(placeholder.len() as u16);
    }

    Cursor::blink();

    for c in stdin().keys() {
        match c.unwrap() {
            Key::Char('\n') => {
                if input.is_empty() && !default.is_empty() {
                    // TODO: remove the light grey on the placeholder
                    input.push_str(default);
                    Cursor::steady();
                    break;
                } else if input.is_empty() && default.is_empty() {
                    // TODO: make input required if input is empty and default is empty
                }
                Cursor::steady();
                break;
            }
            Key::Char(c) => {
                print!("{}", c);
                input.push_str(c.to_string().as_str());
                if !input.is_empty() && !placeholder.is_empty() && placeholder_visible {
                    Cursor::right(placeholder.len() as u16);
                    Cursor::backspace(placeholder.len() as u16);
                    placeholder_visible = false;
                }
            }
            Key::Backspace => {
                if !input.is_empty() {
                    Cursor::backspace(1);
                    input.pop();
                }
                // Restore placeholder if needed
                if input.is_empty() && !placeholder.is_empty() {
                    // TODO: fix this
                    print!(
                        "\x1b[90m{}{}", // light grey
                        placeholder,
                        termion::color::Fg(termion::color::Reset)
                    );
                    io::stdout().flush().unwrap();
                    Cursor::left(placeholder.len() as u16);
                    placeholder_visible = true;
                }
            }
            Key::Esc => {
                Cursor::steady();
                std::process::exit(0);
            }
            _ => {}
        }
        stdout.flush().unwrap();
    }

    // Reset cursor position and print a new line
    Cursor::beginning();
    Cursor::new_line();

    input.trim().to_string()
}

/// Displays a list of options to the user and allows them to select one.
///
/// # Arguments
///
/// * `text` - The prompt text displayed to the user.
/// * `options` - A slice of strings representing the available options.
///
/// # Returns
///
/// A String containing the selected option.
pub fn list(text: &str, options: &[&str]) -> String {
    // Enable raw mode for stdout
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut selected_index = 0;

    Cursor::hide();

    loop {
        // Display prompt text
        println!("{}", text);
        Cursor::beginning();

        // Display options with highlighting for the selected index
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
                Event::Key(Key::Esc) => {
                    Cursor::show();
                    std::process::exit(0);
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

/// Displays the Supaquick logo in the terminal.
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

/// Displays a menu with options and allows the user to select one.
///
/// # Arguments
///
/// * `text` - The prompt text displayed to the user.
/// * `options` - A slice of strings representing the available options.
///
/// # Returns
///
/// A String containing the selected option.
pub fn option(text: &str, options: &[&str]) -> String {
    // Enable raw mode for stdout
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut selected_index = 0;

    Cursor::hide();

    loop {
        // Display prompt text
        print!("{}", text);
        io::stdout().flush().unwrap();

        // Display options with highlighting for the selected index
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
                Event::Key(Key::Esc) => {
                    Cursor::show();
                    std::process::exit(0);
                }
                _ => {}
            }
            stdout.flush().unwrap();
        }
        Cursor::clear_line();
        Cursor::beginning();
    }
}

/// Displays a success message in the terminal when the cli has ran successfully.
///
/// # Arguments
///
/// * `name` - The name of the project for which was successfully created.
pub fn footer(name: &str) {
    Cursor::new_line();
    println!(
        "{}Success!{} {} has been created",
        color::Fg(color::Green),
        color::Fg(color::Reset),
        name
    );
}

/// Represents a text cursor for controlling the display of text in the terminal.
pub struct Cursor;

impl Cursor {
    /// Displays the cursor.
    pub fn show() {
        print!("{}", Show);
        io::stdout().flush().unwrap();
    }

    /// Hides the cursor.
    pub fn hide() {
        print!("{}", Hide);
        io::stdout().flush().unwrap();
    }

    // TODO: Use the cursor style that the user prefers (e.g., block, underline, bar).
    /// Makes the cursor blink (intended for input components).
    pub fn blink() {
        print!("{}", BlinkingBlock);
        io::stdout().flush().unwrap();
    }

    /// Makes the cursor steady, turning off its blink (intended for input components).
    pub fn steady() {
        print!("{}", SteadyBlock);
        io::stdout().flush().unwrap();
    }

    /// Restores the cursor to its default state.
    pub fn restore() {
        print!("{}", Restore);
        io::stdout().flush().unwrap();
    }

    /// Clears the current line where the cursor is located.
    pub fn clear_line() {
        print!("{}", CurrentLine);
        io::stdout().flush().unwrap();
    }

    /// Moves the cursor to the left by the given count.
    pub fn left(count: u16) {
        print!("{}", Left(count));
        io::stdout().flush().unwrap();
    }

    /// Moves the cursor to the left by the given count.
    pub fn down(count: u16) {
        print!("{}", Down(count));
        io::stdout().flush().unwrap();
    }

    /// Moves the cursor to the left by the given count.
    pub fn up(count: u16) {
        print!("{}", Up(count));
        io::stdout().flush().unwrap();
    }

    /// Moves the cursor to the left by the given count.
    pub fn right(count: u16) {
        print!("{}", Right(count));
        io::stdout().flush().unwrap();
    }

    /// Simulates a backspace by moving the cursor to the left by the given count.
    pub fn backspace(count: u16) {
        let mut i = 0;
        while i != count {
            print!("\x08 \x08");
            i += 1;
        }
        io::stdout().flush().unwrap();
    }

    /// Moves the cursor to the beginning of the current line.
    pub fn beginning() {
        print!("\r");
        io::stdout().flush().unwrap();
    }

    /// Creates a new line.
    pub fn new_line() {
        println!();
    }
}

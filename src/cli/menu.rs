extern crate crossterm;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::{self, Write};
use std::path::Path;
use termion::{cursor, event::Key, input::TermRead};

pub fn entry_symbol<T: AsRef<Path>>(entry: T) -> String {
    let entry = entry.as_ref();
    match entry.metadata() {
        Ok(metadata) => {
            if metadata.is_dir() {
                "📁".to_string()
            } else {
                "📄".to_string()
            }
        }
        Err(_) => "".to_string(),
    }
}

pub fn get_yn_input() -> bool {
    enable_raw_mode().expect("Failed to enable raw mode");
    print_yn_prompt().expect("Failed to print prompt");

    for key in io::stdin().keys() {
        match key.unwrap() {
            Key::Char('y') | Key::Char('Y') | Key::Char('ㅛ') => {
                disable_raw_mode().expect("Failed to disable raw mode");
                return true;
            }
            Key::Char('n') | Key::Char('N') | Key::Char('ㅜ') | Key::Esc => {
                disable_raw_mode().expect("Failed to disable raw mode");
                return false;
            }
            _ => {}
        }
    }
    unreachable!();
}

pub fn get_mq_input() -> bool {
    enable_raw_mode().expect("Failed to enable raw mode");
    print_mq_prompt().expect("Failed to print prompt");

    for key in io::stdin().keys() {
        match key.unwrap() {
            Key::Char('m') | Key::Char('M') | Key::Char('ㅡ') => {
                disable_raw_mode().expect("Failed to disable raw mode");
                return true;
            }
            Key::Char('q') | Key::Char('Q') | Key::Char('ㅂ') | Key::Esc => {
                disable_raw_mode().expect("Failed to disable raw mode");
                return false;
            }
            _ => {}
        }
    }
    unreachable!();
}

pub fn print_yn_prompt() -> io::Result<()> {
    print!("{}", cursor::Save);
    print!("{}", cursor::Goto(1, termion::terminal_size().unwrap().1));
    print!("{}", cursor::Restore);
    io::stdout().flush()?;
    Ok(())
}

pub fn print_mq_prompt() -> io::Result<()> {
    print!("{}", cursor::Save);
    print!("{}", cursor::Goto(1, termion::terminal_size().unwrap().1));
    println!("PRESS 'm' to move entries or 'q' to quit...");
    print!("{}", cursor::Restore);
    io::stdout().flush()?;
    Ok(())
}

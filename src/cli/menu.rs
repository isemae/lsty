use crossterm::{
    cursor,
    event::{read, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};

use std::io::{self, Write};
use std::path::Path;

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

pub fn get_yn_input(msg: String) -> bool {
    println!("{}", msg);
    enable_raw_mode().expect("Failed to enable raw mode");
    print_yn_prompt().expect("Failed to print prompt");

    loop {
        if let Ok(Event::Key(event)) = read() {
            match event.code {
                KeyCode::Char('y') | KeyCode::Char('Y') | KeyCode::Char('ㅛ') => {
                    disable_raw_mode().expect("Failed to disable raw mode");
                    return true;
                }
                KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Char('ㅜ') | KeyCode::Esc => {
                    disable_raw_mode().expect("Failed to disable raw mode");
                    return false;
                }
                _ => {}
            }
        }
    }
}

pub fn get_mq_input() -> bool {
    enable_raw_mode().expect("Failed to enable raw mode");
    print_mq_prompt().expect("Failed to print prompt");

    loop {
        if let Ok(Event::Key(event)) = read() {
            match event.code {
                KeyCode::Char('m') | KeyCode::Char('M') | KeyCode::Char('ㅡ') => {
                    disable_raw_mode().expect("Failed to disable raw mode");
                    return true;
                }
                KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Char('ㅂ') | KeyCode::Esc => {
                    disable_raw_mode().expect("Failed to disable raw mode");
                    return false;
                }
                _ => {}
            }
        }
    }
}

pub fn print_yn_prompt() -> io::Result<()> {
    print!("{}", cursor::SavePosition);
    print!("{}", cursor::MoveTo(1, crossterm::terminal::size()?.1));
    print!("{}", cursor::RestorePosition);
    io::stdout().flush()?;
    Ok(())
}

pub fn print_mq_prompt() -> io::Result<()> {
    print!("{}", cursor::SavePosition);
    print!(
        "{}",
        cursor::MoveTo(1, crossterm::terminal::size().unwrap().1)
    );
    println!("PRESS 'm' to move entries or 'q' to quit...");
    print!("{}", cursor::RestorePosition);
    io::stdout().flush()?;
    Ok(())
}

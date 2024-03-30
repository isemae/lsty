use crossterm::style::{StyledContent, Stylize};

pub enum Status {
    YN,
    Error,
    Caution,
    Safe,
    NotFound,
}

pub fn status_symbol(status: &Status) -> StyledContent<&str> {
    match status {
        Status::YN => "[y/N]".yellow(),
        Status::Error => "✘".red(),
        Status::Caution => "!".yellow(),
        Status::NotFound => "?".cyan(),
        Status::Safe => "✓".green(),
    }
}

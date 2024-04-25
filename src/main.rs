mod cli;
mod data;

use clap::Parser;
use cli::commands::{command_manager, commands};

fn main() {
    let args = commands::Config::parse();

    match command_manager::process_command(&args) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("{}", e)
        }
    }
}

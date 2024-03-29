mod cli;
mod commands;
mod data;

use clap::Parser;
use commands::{arguments, command_manager};

fn main() {
    let args = arguments::Config::parse();

    match command_manager::process_command(&args) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("{}", e)
        }
    }
}

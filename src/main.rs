mod commands;
mod data;
mod interfaces;

use clap::{Args, Parser};
use commands::{arguments, command_manager};

fn main() {
    let args = arguments::Cli::parse();

    println!("{:?}", args);
    command_manager::process_command(&args);
}

mod commands;
mod data;
mod interfaces;

use clap::{Args, Parser};
use commands::{arguments::Cli, command_manager};

fn main() {
    let args = Cli::parse();

    println!("{:?}", args);
    command_manager::process_command(&args);
}

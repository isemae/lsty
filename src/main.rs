mod cli;
mod commands;
mod data;

use clap::{Args, Parser};
use commands::{arguments, command_manager};

fn main() {
    let args = arguments::Config::parse();

    println!("{:?}", args);
    command_manager::process_command(&args);
}

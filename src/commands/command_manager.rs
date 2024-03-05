use super::arguments::{Action, Actions, Cli, Commands};
use crate::{
    data::{
        data_manager::{DataAction, DataManager},
        model::DataModel,
    },
    interfaces::menu::{navigate_menu, MenuAction},
};
use regex::Regex;
use std::{
    env::current_dir,
    fs,
    io::{self, Result},
    path::{Path, PathBuf},
};
use unicode_normalization::UnicodeNormalization;

pub fn process_command(cli: &Cli) {
    let mut data_manager = DataManager::new("./data.json");

    match &cli.command {
        Commands::Source(subargs) => match subargs.action {
            Actions::Add => data_manager.match_action(DataAction::Add, cli),
            Actions::Delete => data_manager.match_action(DataAction::Delete, cli),
            Actions::List => {}
        },
        Commands::Target(subargs) => match subargs.action {
            Actions::Add => data_manager.match_action(DataAction::Add, cli),
            Actions::Delete => data_manager.match_action(DataAction::Delete, cli),
            Actions::List => {}
        },

        Commands::Pair(subargs) => match subargs.action {
            Actions::Add => data_manager.match_action(DataAction::Add, cli),
            _ => {}
        },
        Commands::Scan => {}
        Commands::Move(keyword) => {
            let data = DataModel::parse_json();
            move_dirs(data, &keyword.keyword);
        }
        Commands::List => {
            navigate_menu(MenuAction::Default);
        }
        _ => {
            std::process::exit(1);
        }
    }
}

pub fn move_dirs(data: DataModel, keyword: &str) -> io::Result<()> {
    println!("{:?}", data.pairs);
    let mut source_path = "";
    let mut target_path = "";

    for pair in data.pairs.iter() {
        source_path = &pair.source_path;
        for target in &pair.source_targets {
            if target.keyword == keyword {
                target_path = &target.target;
                println!(
                    "Files containing keyword:\"{}\" will be moved to {}",
                    keyword, target_path
                )
            } else {
                println!("no such keyword in list")
            }
        }
    }

    // checks if source/target paths exist
    if !fs::metadata(source_path).is_ok() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Source directory '{}' does not exist.", source_path),
        ));
    }
    if !fs::metadata(target_path).is_ok() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Target directory '{}' does not exist.", target_path),
        ));
    }

    // generates regex pattern
    let pattern = format!(r"{}", keyword);
    let re = Regex::new(&pattern).unwrap();

    let entries = fs::read_dir(source_path)?;
    for entry in entries {
        let entry = entry?;
        let file_path = entry.path();
        let file_name = match file_path.file_name() {
            Some(name) => name.to_string_lossy(),
            None => continue,
        };
        let normalized = file_name.nfc().collect::<String>();

        // checks if the normalized keyword exists in the filename and moves the file if it has the keyword
        if has_keyword(&normalized, &keyword) {
            println!("Found file '{}'.", file_name);
            let new_path = format!("{}/{}", target_path, normalized);
            fs::rename(&file_path, &new_path)?;
            println!("File '{}' moved to '{}'.", file_name, new_path);
        } else {
            println!("File '{}' does not match the pattern.", normalized);
        }
    }
    Ok(())
}

fn has_keyword(text: &str, keyword: &str) -> bool {
    let re = Regex::new(&format!(r"{}", keyword)).unwrap();
    re.is_match(text)
}

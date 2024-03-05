use crate::{
    commands::arguments::{Action, Cli, Commands},
    data::{json_manager, model::DataModel},
    interfaces::menu,
    Args,
};
use camino::Utf8PathBuf;
use std::{
    collections::HashSet,
    fs::{File, OpenOptions},
    io::{self, prelude::*, BufReader, BufWriter},
    path::{Path, PathBuf},
};

pub struct DataManager {
    paths: HashSet<String>,
    data_file: String,
}

pub enum DataAction {
    Add,
    Save,
    Delete,
    Move,
    Read,
    Keyword,
}

impl DataManager {
    pub fn new(data_file: &str) -> Self {
        let paths = DataManager::read_data(data_file);
        DataManager {
            paths,
            data_file: data_file.to_string(),
        }
    }

    pub fn match_action(&mut self, action: DataAction, cli: &Cli) {
        println!("Keyword: {}", cli.get_keyword().unwrap_or("empty_keyword"));

        if let Some(action) = cli.get_action() {
            println!("Action: {:?}", action);
        } else {
            println!("No action provided");
        }

        match action {
            DataAction::Add => {
                self.add_to(cli);
            }
            DataAction::Delete => {
                self.delete_from(cli);
            }
            DataAction::Keyword => {
                self.add_keyword(cli);
            }

            _ => {}
        }
    }

    fn add_to(&mut self, cli: &Cli) -> io::Result<()> {
        let mut add_path = Utf8PathBuf::new();
        let mut add_opt_path = Utf8PathBuf::new();

        if let Some(dir) = cli.get_dir() {
            add_path.push(dir.as_str());
        } else {
            println!("Error: Retrieved no path from cli input")
        }

        if let Some(opt_path) = cli.get_opt_path().unwrap() {
            add_opt_path.push(opt_path.as_str());
        }

        println!("{:?}", add_path);
        println!("{:?}", add_opt_path);

        if add_path.exists() {
            match cli.command {
                Commands::Source(_) => println!("Add this directory to Sources? (y/n):"),
                Commands::Target(_) => println!("Add this directory to Targets? (y/n):"),
                Commands::Pair(_) => println!("Add this directory to Pairs? (y/n):"),
                _ => println!(""),
            }
            let yn_status = menu::get_yn_input();
            if yn_status {
                if let Err(err) = DataModel::create_json(
                    cli,
                    add_path.to_string(),
                    add_opt_path.to_string(),
                    &cli.get_keyword().unwrap_or("empty_keyword"),
                ) {
                    eprintln!("Failed to create data file: {}", err);
                    return Err(err.into());
                }
            }
            Ok(())
        } else {
            println!("No such path exists.");
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                "No such path exists.",
            ))
        }
    }

    fn delete_from(&mut self, cli: &Cli) -> io::Result<()> {
        let keyword = cli.get_keyword().unwrap();
        let path = cli.get_dir().unwrap();

        match cli.command {
            Commands::Source(_) => {
                println!("Are you sure you want to delete current directory from sources? (y/n):")
            }
            Commands::Target(_) => {
                println!("Are you sure you want to delete current directory from targets? (y/n):")
            }
            _ => println!(""),
        }

        let yn_status = menu::get_yn_input();

        if yn_status {
            match DataModel::remove_json(&keyword, &cli.command) {
                Ok(()) => {
                    println!("path deleted {:?}:", path)
                }
                Err(e) => eprintln!("{}", e),
            }
        }
        Ok(())
    }

    fn scan_dir(&mut self, path: PathBuf) {}
    fn add_keyword(&mut self, args: &Cli) {
        let data = DataModel::parse_json();
        println!("{:?}", data);
    }

    pub fn read_data(data_file: &str) -> HashSet<String> {
        let mut paths = HashSet::new();
        if let Ok(file) = File::open(data_file) {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                if let Ok(path_str) = line {
                    let path = String::from(path_str.trim());
                    paths.insert(path);
                }
            }
        }
        paths
    }

    pub fn manage_source(args: Cli) {}
    pub fn manage_target(args: Cli) {}
}

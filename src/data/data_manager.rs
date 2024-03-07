use crate::{
    commands::arguments::{Cli, Commands, DelArgs, SubArgs},
    data::{json_manager, model::DataModel},
    interfaces::menu,
    Args,
};

use camino::Utf8PathBuf;
use colored::*;
use regex::Regex;
use serde_json::from_str;
use std::{
    collections::HashSet,
    fs::{self, File, OpenOptions},
    io::{self, prelude::*, BufReader, BufWriter},
    path::{Path, PathBuf},
};
use unicode_normalization::UnicodeNormalization;

pub struct DataManager {
    paths: HashSet<String>,
    data_file: String,
}

pub enum DataAction {
    Add,
    Delete,
    Move,
    Read,
}

impl DataManager {
    pub fn new(data_file: &str) -> Self {
        let paths = DataManager::load_data_file(data_file);
        DataManager {
            paths,
            data_file: data_file.to_string(),
        }
    }

    pub fn match_action(&mut self, action: DataAction, cli: &Cli) {
        let keyword = cli.get_keyword().unwrap_or("");
        let target_path = cli.get_target_path();

        match action {
            DataAction::Add => {
                if let Commands::Add(SubArgs {
                    keyword,
                    source_path,
                    target_path,
                }) = &cli.command
                {
                    self.add_to(keyword, source_path, target_path);
                }
            }
            DataAction::Delete => {
                if let Commands::Del(DelArgs {
                    keyword,
                    target_path,
                }) = &cli.command
                {
                    self.delete_from(
                        target_path.as_ref().unwrap().to_owned(),
                        keyword.as_ref().unwrap().as_str(),
                    );
                }
            }
            DataAction::Read => self.scan_path(cli.get_source_path().unwrap()),
            DataAction::Move => {
                let data = DataModel::parse_json();
                // self.move_dirs(data, keyword)
            }
            _ => {}
        }
    }

    fn add_to(
        &mut self,
        keyword: &str,
        source_path: &Utf8PathBuf,
        target_path: &Utf8PathBuf,
    ) -> io::Result<()> {
        // self.print_rule_info("add", &cli)?;

        if Utf8PathBuf::from(source_path).exists() {
            if menu::get_yn_input() {
                if let Err(err) = DataModel::create_json(
                    source_path.to_string(),
                    target_path.to_string(),
                    keyword,
                ) {
                    eprintln!("Failed to create data file: {}", err);
                    return Err(err.into());
                }
            }

            println!("new rule added:");
            println!(
                "items that has keyword \x1b[4m{:?}\x1b[0m will be moved",
                keyword
            );
            Ok(())
        } else {
            println!("No such path exists on the volume.");
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                "No such path exists.",
            ))
        }
    }

    fn delete_from(&mut self, target_path: Utf8PathBuf, keyword: &str) -> io::Result<()> {
        // self.print_rule_info("delete", cli.keyword);
        println!("k");
        if menu::get_yn_input() {
            match DataModel::remove_lst_from_json(keyword) {
                Ok(()) => {
                    println!("rule deleted.")
                }
                Err(e) => eprintln!("{}", e),
            }
        }
        Ok(())
    }

    fn load_data_file(data_file: &str) -> HashSet<String> {
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

    fn print_rule_info(&self, action: &str, cli: &Cli) -> io::Result<()> {
        let mut source_path = Utf8PathBuf::new();
        let mut target_path = Utf8PathBuf::new();

        source_path.push(cli.get_source_path().unwrap());
        target_path.push(cli.get_source_path().unwrap());

        let keyword = cli.get_keyword().unwrap_or("empty_keyword");

        println!("add this rule? (y/n):");
        println!("┊ Keyword: {}", keyword);
        println!("┊ - SOURCE: \x1b[4m{:?}\x1b[0m", source_path);
        println!("┊ - TARGET: └─> \x1b[4m{:?}\x1b[0m", target_path);
        println!("");
        Ok(())
    }

    fn scan_path(&mut self, path: &Utf8PathBuf) {}

    fn move_dirs(&self, data: DataModel, keyword: &str) -> io::Result<()> {
        // println!("{:?}", data.pairs);
        let mut source_path = "";
        let mut target_path = "";
        let mut source_pathbuf = Utf8PathBuf::new();
        let mut target_pathbuf = Utf8PathBuf::new();

        for pair in data.pairs.iter() {
            let source_path = &pair.source_path;
            let mut target_path = pair
                .source_targets
                .iter()
                .find(|target| target.keyword == keyword)
                .map(|target| &target.target)
                .unwrap();
            // .unwrap_or_else(|| "");

            // checks if source/target paths exist
            if !Path::new(source_path).exists() {
                eprintln!(
                    "\x1b[0;31m ✘ Source path {} does not exist\x1b[0m",
                    source_path.yellow()
                );
            } else {
                println!("\x1b[0;32m SOURCE: {}\x1b[0m", source_path.yellow());
            }

            // generates regex pattern
            let pattern = format!(r"{}", keyword);
            let re = Regex::new(&pattern).unwrap();

            let entries = fs::read_dir(source_path)?;
            let mut moved_count = 0;

            for entry in entries {
                let entry = entry?;
                let item_path = entry.path();
                let item_name = match item_path.file_name() {
                    Some(name) => name.to_string_lossy(),
                    None => continue,
                };
                let normalized = item_name.nfc().collect::<String>();

                // checks if the normalized keyword exists in the filename and moves the file if it has the keyword
                let re = Regex::new(&format!(r"{}", &keyword)).unwrap();
                if re.is_match(&normalized) {
                    let new_path = format!("{}/{}", target_path, normalized);
                    if Path::new(&new_path).exists() {
                        println!(" ! '{}' already exists in the target directory.", item_name);
                        continue;
                    }

                    println!("  ⤹ \x1b[4m{}\x1b[0m\x1b[0m", item_name);
                    moved_count += 1;
                }
            }

            if !Path::new(target_path).exists() {
                eprintln!(
                    "\x1b[0;31m ✘ Target path {} does not exist\x1b[0m\n",
                    target_path.yellow()
                );
            } else {
                println!("\x1b[0;32m TARGET: {}\x1b[0m\n", target_path.yellow());
            }
            if moved_count == 0 {
                println!("No items to move\n");
            }
        }
        Ok(())
    }
}

use super::model::*;
use crate::{
    cli::menu,
    commands::arguments::{Commands, Config, SubArgs},
    data::data_manager::json_manager::JsonManager,
    data::{json_manager, model::DataModel},
    Args,
};

use camino::{Utf8Path, Utf8PathBuf};
use colored::*;
use fs_extra;

use regex::Regex;
use serde::{ser::Error, Deserialize, Serialize};
use serde_json::{from_str, Value};
use std::{
    collections::HashMap,
    env,
    fs::{self, File, OpenOptions},
    io::{self, prelude::*, BufReader, BufWriter},
    ops::Index,
    path::{Path, PathBuf},
    process,
};
use unicode_normalization::UnicodeNormalization;

pub struct DataManager;
// pub struct DataManager {
// model: Box<dyn DataModelTrait>,
// model: DataModel,
// data: HashSet<String>,
// }

pub enum DataAction {
    Add,
    Delete,
    Move,
    Copy,
    Read,
}

// trait DataModelTrait {
//     fn parse_json(&self) -> DataModel;
// }

impl DataManager {
    pub fn new() -> Self {
        DataManager
    }
    pub fn match_action(&mut self, action: DataAction, args: &SubArgs) -> std::io::Result<()> {
        // println!("{:?}", args);
        let mut data = self.parse_json_data().unwrap_or_else(|_| DataModel {
            pairs: HashMap::new(),
        });
        match action {
            DataAction::Add => {
                self.print_rule_info(args);
                match self.add_rule_to_json(
                    data,
                    args.source_path.to_string(),
                    args.target_path.to_string(),
                    args.keyword.clone(),
                ) {
                    Ok(()) => {}
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        process::exit(1);
                    }
                };
            }
            DataAction::Delete => {
                if args.keyword.is_empty() {
                    println!("delmenu")
                } else {
                    println!("{}", args.source_path.as_str());

                    match self.remove_rule_from_json(
                        data.clone(),
                        args.source_path.as_str(),
                        args.keyword.as_str(),
                    ) {
                        Ok(()) => println!("rule deleted successfully"),
                        Err(e) => {
                            eprintln!("Error: {}", e);
                            println!(
                                "keywords available for current path: \n '{}'",
                                if let Some(pair) = data.pairs.get_mut(args.source_path.as_str()) {
                                    pair.iter()
                                        .map(|(k, _)| k.clone())
                                        .collect::<Vec<_>>()
                                        .join("', '")
                                } else {
                                    "".to_string()
                                }
                                .yellow()
                            );
                            process::exit(1);
                        }
                    }
                }
            }
            DataAction::Read => {
                self.scan_path(args.source_path);
            }
            DataAction::Move => {
                self.move_dirs(&args.keyword)?;
            }
            DataAction::Copy => {}
            _ => return Err(io::Error::new(io::ErrorKind::Other, "Unknown action")),
        }
        Ok(())
    }

    pub fn parse_json_data(&self) -> Result<DataModel, serde_json::Error> {
        let exe_path = env::current_exe().unwrap();
        let exe_dir = exe_path.parent().unwrap();
        match File::open(exe_dir.join("lsty.json")) {
            Ok(mut file) => {
                let mut data = String::new();
                match file.read_to_string(&mut data) {
                    Ok(..) => {}
                    Err(e) => eprintln!("Error: {}", e),
                }
                serde_json::from_str(&data)
            }
            Err(e) => Err(serde_json::Error::custom(format!(
                "failed to load data file: {}",
                e
            ))),
        }
    }

    pub fn save_json_data(&self, data: &DataModel) -> Result<(), io::Error> {
        let exe_path = env::current_exe().unwrap();
        let exe_dir = exe_path.parent().unwrap();

        let mut file = File::create(exe_dir.join("lsty.json"))?;
        serde_json::to_writer_pretty(&mut file, &data)?;
        Ok(())
    }

    pub fn add_rule_to_json(
        &self,
        mut data: DataModel,
        source_path: String,
        target_path: String,
        keyword: String,
    ) -> io::Result<()> {
        let target_path_on_volume = Utf8Path::new(target_path.as_str());
        if !target_path_on_volume.exists() || !target_path_on_volume.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "no such directory exists.",
            ));
        }

        if let Some(pair) = data.pairs.get_mut(&source_path) {
            if !pair.contains_key(&target_path) && !pair.contains_key(&keyword) {
                pair.insert(keyword, target_path);
            } else if pair.contains_key(&keyword) {
                eprintln!(
                    "rule for the target '{}' already exists. do you want to change the keyword? (y/N):",
                    target_path
                );
                if menu::get_yn_input() {
                    pair.insert(keyword, target_path);
                    println!("rule added.")
                }
            } else {
                eprintln!(
                    "rule for the keyword '{}' already exists. do you want to change the target? (y/N):",
                    keyword
                );
                if menu::get_yn_input() {
                    pair.insert(keyword, target_path);
                    println!("rule added.")
                }
            }
        } else {
            let mut new_pair = HashMap::new();
            new_pair.insert(keyword, target_path);
            data.pairs.insert(source_path, new_pair);
        }
        self.save_json_data(&data)?;
        Ok(())
    }

    fn remove_rule_from_json(
        &self,
        mut data: DataModel,
        source_path: &str,
        keyword: &str,
    ) -> Result<(), io::Error> {
        // source(current path) validation
        if let Some(targets) = data.pairs.get_mut(source_path) {
            if targets.contains_key(keyword) {
                targets.remove(keyword);
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "no such keyword rule for the current path",
                ));
            }

            if menu::get_yn_input() {
                self.save_json_data(&data);
            }
        } else {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "no rule for the current path in the data",
            ));
        }
        Ok(())
    }

    // pub fn load_data_file(&self) -> Result<Value, io::Error> {
    //     let file = File::open("lsty.json")?;
    //     let mut buffer = String::new();
    //     let mut reader = io::BufReader::new(file);
    //     reader.read_to_string(&mut buffer)?;

    //     let data: Value = serde_json::from_str(&buffer)?;

    //     Ok(data)
    // }

    fn move_dirs(&self, keyword: &str) -> io::Result<()> {
        let data: DataModel = self.parse_json_data()?;
        if keyword != "" {
            for pair in data.pairs.iter() {
                let source_path = &pair.0;
                let target_path = pair
                    .1
                    .iter()
                    .find(|target| target.0 == keyword)
                    .map(|target| target.1.as_str())
                    .unwrap_or(&"");

                // checks if source path exists
                if !target_path.is_empty() {
                    println!("");
                    if !Path::new(source_path).exists() {
                        eprintln!(
                            "\x1b[0;31m ✘ Source path {} does not exist\x1b[0m",
                            source_path.yellow()
                        );
                        continue;
                    } else {
                        println!("SOURCE: {}", source_path.yellow());
                    }

                    if !Path::new(target_path).exists() {
                        eprintln!(
                        "\x1b[0;33m⚠ target path '{}' does not exist. Creating the directory...\x1b[0m",
                        target_path.yellow()
                    );
                        fs::create_dir_all(&target_path).expect("fff");
                    }

                    // generates regex pattern
                    let re = Regex::new(&format!(r"{}", &keyword)).unwrap();
                    let mut moved_count = 0;

                    let entries = fs::read_dir(source_path)?;
                    for entry in entries {
                        let entry = entry?;
                        let item_path = entry.path();
                        let item_name = match item_path.file_name() {
                            Some(name) => name.to_string_lossy(),
                            None => continue,
                        };
                        let normalized = item_name.nfc().collect::<String>();
                        if re.is_match(&item_name) {
                            let new_path = format!("{}/{}", target_path, normalized);
                            if Path::new(&new_path).exists() {
                                println!(
                                "│ \x1b[0;31mEXIST:\x1b[0m {} already exists in the target directory.",
                                item_name
                            );
                                continue;
                            } else {
                                println!(
                                    "│\x1b[0;32m MOVE:\x1b[0m  \x1b[4m{}\x1b[0m\x1b[0m",
                                    item_name
                                );
                                if item_path.is_dir() {
                                    fs::create_dir_all(&new_path).expect("");
                                    self.copy_dir(&item_path, &PathBuf::from(&new_path))
                                        .expect("");
                                    fs::remove_dir_all(&item_path).expect("");
                                } else {
                                    fs::copy(&item_path, new_path).expect("");
                                    fs::remove_file(&item_path).expect("");
                                }
                            }
                            moved_count += 1;
                        }
                    }

                    println!("TARGET: {}", target_path.yellow());
                    if moved_count == 0 {
                        println!("No items to move");
                    }
                }
            }
        } else {
            println!("mmmm")
        }
        Ok(())
    }

    fn copy_dir(&self, src: &PathBuf, trg: &PathBuf) -> std::io::Result<()> {
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let file_path = entry.path();
            let file_name = file_path.file_name().unwrap().to_str().unwrap();
            let new_path = trg.join(file_name);
            if file_path.is_dir() {
                fs::create_dir_all(&new_path)?;
                self.copy_dir(&file_path, &new_path)?;
            } else {
                fs::copy(&file_path, &new_path).expect("");
            }
        }
        Ok(())
    }
    fn print_rule_info(&self, args: &SubArgs) -> io::Result<()> {
        let mut source_path = Utf8PathBuf::new();
        let mut target_path = Utf8PathBuf::new();

        source_path.push(args.source_path);
        target_path.push(args.target_path);

        let keyword = &args.keyword;

        println!("┊ - KEYWORD: {}", keyword);
        println!("┊ - SOURCE : \x1b[4m{:?}\x1b[0m", source_path);
        println!("┊ - TARGET : └─> \x1b[4m{:?}\x1b[0m", target_path);
        println!("");
        Ok(())
    }

    fn scan_path(&mut self, path: &Utf8PathBuf) {}
}

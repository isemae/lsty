use super::model::*;

use crate::{
    cli::menu,
    commands::arguments::{Commands, Config, SubArgs},
    data::data_manager::json_manager::JsonManager,
    data::{json_manager, model::DataModel},
    Args,
};
use serde::{ser::Error, Deserialize, Serialize};

use camino::{Utf8Path, Utf8PathBuf};
use colored::*;
use regex::Regex;
use serde_json::{from_str, Value};
use std::{
    collections::HashSet,
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
        // let data = load_data_file();
        // DataManager { data }
        DataManager
        // let paths = HashSet::new();
    }
    pub fn match_action(&mut self, action: DataAction, args: &SubArgs) -> std::io::Result<()> {
        // println!("{:?}", args);
        let mut data = self.parse_json_data().unwrap_or_else(|_| DataModel {
            pairs: vec![Pair {
                source_path: args.source_path.to_string(),
                source_targets: vec![SourceTarget {
                    target: args.target_path.to_string(),
                    keyword: args.keyword.to_string(),
                }],
            }],
        });
        match action {
            DataAction::Add => {
                // let source_data = std::fs::read_to_string(&args.source_path)?;
                match self.add_rule_to_json(
                    data,
                    args.source_path.as_str(),
                    args.target_path.as_str(),
                    &args.keyword.to_string(),
                ) {
                    Ok(()) => {
                        self.print_rule_info(args);
                        println!("rule added.")
                    }
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
                                "keywords available: {}",
                                data.pairs
                                    .iter()
                                    .flat_map(|p| p.source_targets.iter())
                                    .filter(|k| !k.keyword.is_empty())
                                    .map(|k| k.keyword.clone())
                                    .collect::<Vec<_>>()
                                    .join(", ")
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
            DataAction::Copy => {
                // self.add_new_rule(&args.keyword, args.source_path, args.target_path);
            }
            _ => return Err(io::Error::new(io::ErrorKind::Other, "Unknown action")),
        }
        Ok(())
    }

    // pub fn parse_json_data<DataModel: for<'b> Deserialize<'b>>(&self,) -> Result<Vec<DataModel>, serde_json::Error> {
    pub fn parse_json_data(&self) -> Result<DataModel, serde_json::Error> {
        match File::open("./data.json") {
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

    pub fn save_json_data(&self, data: DataModel) -> Result<(), io::Error> {
        let json = serde_json::to_vec_pretty(&data)?;
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open("./data.json")?;
        file.write_all(&json)?;
        Ok(())
    }

    // fn add_new_rule(
    //     &mut self,
    //     keyword: &str,
    //     source_path: &Utf8PathBuf,
    //     target_path: &Utf8PathBuf,
    // ) -> io::Result<()> {
    //     // self.print_rule_info("add", &cli)?;

    //     let mut data = DataModel::new(self);
    //     let new_rule: DataModel;

    //     if Utf8PathBuf::from(source_path).exists() {
    //         if menu::get_yn_input() {
    //             if let Err(err) = DataModel::add_source_target(
    //                 &mut data,
    //                 source_path.as_str(),
    //                 target_path.as_str(),
    //                 keyword,
    //             ) {
    //                 eprintln!("Failed to create data file: {}", err);
    //                 return Err(err.into());
    //             }
    //         }

    //         println!("new rule added:");
    //         println!(
    //             "items that has keyword \x1b[4m{:?}\x1b[0m will be moved",
    //             keyword
    //         );
    //         println!("Data added from {:?} to {:?}", source_path, target_path);
    //         Ok(())
    //     } else {
    //         println!("No such path exists on the volume.");
    //         Err(io::Error::new(
    //             io::ErrorKind::NotFound,
    //             "No such path exists.",
    //         ))
    //     }
    // }

    pub fn add_rule_to_json(
        &self,
        mut data: DataModel,
        source_path: &str,
        target_path: &str,
        keyword: &str,
    ) -> io::Result<()> {
        let target_path_on_volume = Utf8Path::new(target_path);
        if !target_path_on_volume.exists() || !target_path_on_volume.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "no such directory exists.",
            ));
        }

        if let Some(pair) = data
            .pairs
            .iter_mut()
            .find(|pair| pair.source_path == source_path)
        {
            let target_exist = pair
                .source_targets
                .iter()
                .any(|target| target.target == target_path && target.keyword == keyword);
            if !target_exist {
                pair.source_targets.push(SourceTarget {
                    target: target_path.to_string(),
                    keyword: keyword.to_string(),
                })
            }
        } else {
            let new_pair = Pair {
                source_path: source_path.to_string(),
                source_targets: vec![SourceTarget {
                    target: target_path.to_string(),
                    keyword: keyword.to_string(),
                }],
            };
            data.pairs.push(new_pair);
        }
        self.save_json_data(data);
        Ok(())
    }

    fn remove_rule_from_json(
        &self,
        mut data: DataModel,
        source_path: &str,
        keyword: &str,
    ) -> Result<(), io::Error> {
        // source(current path) validation
        if let Some(pair) = data
            .pairs
            .iter_mut()
            .find(|pair| pair.source_path == source_path)
        {
            if let Some(index) = pair
                .source_targets
                .iter()
                .position(|st| st.keyword == keyword)
            {
                pair.source_targets.remove(index);
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "no such keyword rule for the current path",
                ));
            }

            if menu::get_yn_input() {
                let mut file = File::create("data.json")?;
                serde_json::to_writer_pretty(&mut file, &data)?;
            }
        } else {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "no such source path in the data model",
            ));
        }
        Ok(())
    }

    pub fn load_data_file(&self) -> Result<Value, io::Error> {
        let file = File::open("data.json")?;
        let mut buffer = String::new();
        let mut reader = io::BufReader::new(file);
        reader.read_to_string(&mut buffer)?;

        let data: Value = serde_json::from_str(&buffer)?;

        Ok(data)
    }

    fn move_dirs(&self, keyword: &str) -> io::Result<()> {
        let data: DataModel = self.parse_json_data()?;

        for pair in data.pairs.iter() {
            let source_path = &pair.source_path;
            let target_path = pair
                .source_targets
                .iter()
                .find(|target| target.keyword == keyword)
                .map(|target| target.target.as_str())
                .unwrap_or("");

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
                    fs::create_dir_all(&target_path);
                }
                // generates regex pattern
                let re = Regex::new(&format!(r"{}", &keyword)).unwrap();
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
                            fs::rename(&item_path, &new_path)?;
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

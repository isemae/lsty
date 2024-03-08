use super::model::*;

use crate::{
    cli::menu,
    commands::arguments::{Commands, Config, SubArgs},
    data::data_manager::json_manager::JsonManager,
    data::{json_manager, model::DataModel},
    Args,
};
use serde::{Deserialize, Serialize};

use camino::{Utf8Path, Utf8PathBuf};
use colored::*;
use regex::Regex;
use serde_json::{from_str, Value};
use std::{
    collections::HashSet,
    error::Error,
    fs::{self, File, OpenOptions},
    io::{self, prelude::*, BufReader, BufWriter},
    path::{Path, PathBuf},
};
use unicode_normalization::UnicodeNormalization;

pub struct DataManager {
    // model: Box<dyn DataModelTrait>,
    // model: DataModel,
    paths: HashSet<String>,
    data_file: String,
}

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
        // let data_file = "data.json";
        // let data_manager = DataManager::new();
        // let model = DataModel::new(&data_manager);
        // let paths = DataManager::load_data_file(data_file);
        let paths = HashSet::new();
        DataManager {
            paths,
            data_file: "data.json".to_string(),
        }
    }

    pub fn parse_json_data<T: for<'b> Deserialize<'b>>(
        &self,
        file_path: &str,
    ) -> Result<T, io::Error> {
        let file = File::open(file_path)?;
        let data = serde_json::from_reader(file)?;
        Ok(data)
    }

    pub fn save_json_data<T: Serialize>(&self, file_path: &str, data: &T) -> Result<(), io::Error> {
        let json = serde_json::to_vec_pretty(data)?;
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_path)?;
        file.write_all(&json)?;
        Ok(())
    }

    pub fn match_action(&mut self, action: DataAction, args: &SubArgs) -> std::io::Result<()> {
        // let keyword = config.get_keyword().unwrap_or("");
        // let source_path = config.get_source_path();
        // let target_path = config.get_target_path();
        let model = DataModel::new(&self);
        // let data_manager = DataManager::new("data.json");
        println!("{:?}", args);
        // match action {
        //     DataAction::Add => {
        //         let source_data = std::fs::read_to_string(&args.source_path)?;
        //         self.add_new_rule(&args.keyword, &args.source_path, &args.target_path)?;
        //         println!(
        //             "Data added from {:?} to {:?}",
        //             args.source_path, args.target_path
        //         );
        //     }
        //     DataAction::Delete => self.delete_from(
        //         args.target_path
        //             .as_ref()
        //             .map(|p| p.to_owned())
        //             .unwrap_or_else(Utf8PathBuf::new)
        //             .to_owned(),
        //         args.keyword.as_deref().unwrap(),
        //     )?,
        //     DataAction::Read => {
        //         self.scan_path(args.get_source_path().unwrap());
        //     }
        //     DataAction::Move => {
        //         let data_manager = DataManager::new();

        //         // let data_manager = DataManager::new(fwq);
        //         self.move_dirs(model, &args.keyword)?;
        //     }
        //     DataAction::Copy => {
        //         let data = self.load_data_file();
        //         self.add_new_rule(
        //             &args.keyword,
        //             args.source_path.unwrap(),
        //             args.target_path.unwrap(),
        //         );
        //     }
        //     _ => return Err(io::Error::new(io::ErrorKind::Other, "Unknown action")),
        // }
        Ok(())
    }

    fn add_new_rule(
        &mut self,
        keyword: &str,
        source_path: &Utf8PathBuf,
        target_path: &Utf8PathBuf,
    ) -> io::Result<()> {
        // self.print_rule_info("add", &cli)?;

        if Utf8PathBuf::from(source_path).exists() {
            if menu::get_yn_input() {
                // if let Err(err) =
                //     create_json(source_path.to_string(), target_path.to_string(), keyword)
                // {
                // eprintln!("Failed to create data file: {}", err);
                // return Err(err.into());
                // }
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
    pub fn add_lst_to_json(
        data: &mut DataModel,
        source_path: &str,
        target_path: &str,
        keyword: &str,
    ) {
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
    }

    fn remove_lst_from_json(&self, keyword: &str) -> std::io::Result<()> {
        let file = File::open("data.json")?;
        let mut data: serde_json::Value = serde_json::from_reader(file)?;

        if let Some(pairs) = data.get_mut("pairs").and_then(|p| p.as_array_mut()) {
            pairs.retain(|pair| {
                pair["source_targets"]
                    .as_array()
                    .map(|targets| {
                        !targets
                            .iter()
                            .any(|t| t["keyword"] == Value::String(keyword.to_string()))
                    })
                    .unwrap_or(false)
            });
        }

        let mut file = File::create("data.json")?;
        serde_json::to_writer_pretty(&mut file, &data)?;

        Ok(())
    }

    fn delete_from(&mut self, target_path: Utf8PathBuf, keyword: &str) -> io::Result<()> {
        // self.print_rule_info("delete", config.keyword);
        println!("k");
        if menu::get_yn_input() {
            match self.remove_lst_from_json(keyword) {
                Ok(()) => {
                    println!("rule deleted.")
                }
                Err(e) => eprintln!("{}", e),
            }
        }
        Ok(())
    }

    fn load_data_file(&self) -> HashSet<String> {
        let mut paths = HashSet::new();
        if let Ok(file) = File::open("data.json") {
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
    // fn print_rule_info(&self, action: &str, config:&Config) -> io::Result<()> {
    //     let mut source_path = Utf8PathBuf::new();
    //     let mut target_path = Utf8PathBuf::new();

    //     source_path.push(config.get_source_path().unwrap());
    //     target_path.push(config.get_source_path().unwrap());

    //     let keyword = config.get_keyword().unwrap_or("empty_keyword");

    //     println!("add this rule? (y/n):");
    //     println!("┊ Keyword: {}", keyword);
    //     println!("┊ - SOURCE: \x1b[4m{:?}\x1b[0m", source_path);
    //     println!("┊ - TARGET: └─> \x1b[4m{:?}\x1b[0m", target_path);
    //     println!("");
    //     Ok(())
    // }

    fn scan_path(&mut self, path: &Utf8PathBuf) {}
}

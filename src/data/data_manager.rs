use crate::{commands::arguments::SubArgs, data::model::DataModel};

use camino::{Utf8Path, Utf8PathBuf};
use colored::*;

use serde::ser::Error;
use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{self, prelude::*},
    path::{Path, PathBuf},
    process,
};

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
    Import,
    Scan,
}

impl DataManager {
    pub fn new() -> Self {
        DataManager
    }
    pub fn match_action(&mut self, action: DataAction, args: &SubArgs) -> Result<(), io::Error> {
        let mut data = self.parse_json_data().unwrap_or_else(|_| DataModel {
            pairs: HashMap::new(),
        });
        let current_dir = env::current_dir().unwrap_or_else(|_| PathBuf::from(""));

        match action {
            DataAction::Add => {
                match self.print_rule_info(args) {
                    Ok(()) => {}
                    Err(e) => {
                        eprintln!("Error: {}", e)
                    }
                }
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
            DataAction::Scan => {
                // self.scan_and_validate_path(data);
            }
            DataAction::Move => {
                if let Some(pair) = data.pairs.get(&current_dir.to_string_lossy().to_string()) {
                    self.move_dirs(pair.to_owned(), args.keyword.clone())?;
                }
            }
            DataAction::Import => {}
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

    fn print_rule_info(&self, args: &SubArgs) -> io::Result<()> {
        let mut source_path = Utf8PathBuf::new();
        let mut target_path = Utf8PathBuf::new();

        source_path.push(args.source_path);
        target_path.push(args.target_path);

        let keyword = &args.keyword;

        println!(" KEYWORD: {}", keyword);
        println!(" SOURCE : \x1b[4m{:?}\x1b[0m", source_path);
        println!(" TARGET : └─> \x1b[4m{:?}\x1b[0m", target_path);
        println!("");
        Ok(())
    }
}

use crate::{
    commands::arguments::{Commands, SubArgs},
    data::model::DataModel,
};

use camino::{Utf8Path, Utf8PathBuf};
use colored::*;

use serde::ser::Error;
use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{self, prelude::*},
    ops::Deref,
    path::{Path, PathBuf},
    process,
    str::FromStr,
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
    Alias,
    Scan,
    Default,
}

impl From<&Commands> for DataAction {
    fn from(c: &Commands) -> Self {
        match c {
            Commands::Add { .. } => DataAction::Add,
            Commands::Del { .. } => DataAction::Delete,
            Commands::Move { .. } => DataAction::Move,
            Commands::Alias { .. } => DataAction::Alias,
            Commands::Import { .. } => DataAction::Import,
            _ => DataAction::Default,
        }
    }
}

impl DataManager {
    pub fn new() -> Self {
        DataManager
    }
    pub fn match_action(&mut self, action: DataAction, args: &SubArgs) -> Result<(), io::Error> {
        let mut data = self
            .parse_json_data()
            .unwrap_or_else(|_| DataModel { data: Vec::new() });
        let current_dir = env::current_dir().unwrap_or_else(|_| PathBuf::from(""));

        match action {
            DataAction::Add => {
                match self.print_rule_info(&args) {
                    Ok(()) => {}
                    Err(e) => {
                        eprintln!("Error: {}", e)
                    }
                }
                match self.add_rule_to_json(
                    data.clone(),
                    args.secondary_path.to_string(),
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
                    println!("{}", args.primary_path.as_str());

                    match self.remove_rule_from_json(
                        data.clone(),
                        args.primary_path.as_str(),
                        args.keyword.as_str(),
                    ) {
                        Ok(()) => println!("rule deleted successfully"),
                        Err(e) => {
                            eprintln!("Error: {}", e);
                            println!(
                                "keywords available for current path: \n '{}'",
                                if let Some(obj) = data
                                    .data
                                    .iter()
                                    .find(|o| o.source.contains(&args.primary_path.to_string()))
                                {
                                    obj.targets.keys().cloned().collect::<Vec<_>>().join("', '")
                                } else {
                                    "".to_string()
                                }
                            );
                            process::exit(1);
                        }
                    }
                }
            }
            DataAction::Scan => {
                // self.scan_and_validate_path(data.data);
            }
            DataAction::Move => {
                if let Some(target_map) = data.data.iter_mut().find(|obj| {
                    obj.source
                        .contains(&current_dir.to_string_lossy().to_string())
                }) {
                    self.move_dirs(&target_map.targets, args.keyword.as_str())?;
                }
            }
            DataAction::Import => {
                println!("{:?}", args);
                match self.import_rule(&mut data, args.keyword.clone(), args.secondary_path.clone())
                {
                    Ok(()) => {
                        println!("rules imported.")
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e)
                    }
                }
            }
            DataAction::Alias => {
                if let Some(target_map) = data.data.iter_mut().find(|obj| {
                    obj.source
                        .contains(&current_dir.to_string_lossy().to_string())
                }) {
                    self.set_alias(target_map, args.keyword.clone());
                    match self.save_json_data(&data) {
                        Ok(()) => println!("set new alias {}", args.keyword),
                        Err(e) => eprintln!("{}", e),
                    };
                }
            }
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
                serde_json::from_str(data.as_str())
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
        serde_json::to_writer_pretty(&mut file, data)?;
        Ok(())
    }

    fn print_rule_info(&self, args: &SubArgs) -> io::Result<()> {
        let mut source_path = Utf8PathBuf::new();
        let mut target_path = Utf8PathBuf::new();

        source_path.push(args.primary_path.clone());
        target_path.push(args.secondary_path.clone());

        let keyword = &args.keyword;

        println!(" KEYWORD: {}", keyword);
        println!(" SOURCE : \x1b[4m{}\x1b[0m", source_path);
        println!(" TARGET : └─> \x1b[4m{}\x1b[0m \n", target_path);
        Ok(())
    }
}

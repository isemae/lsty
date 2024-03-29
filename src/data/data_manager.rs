use crate::{
    cli::menu,
    commands::arguments::{Commands, SubArgs},
    data::model::DataModel,
};

use camino::Utf8PathBuf;

use serde::ser::Error;
use std::{
    env,
    fs::File,
    io::{self, prelude::*},
    process,
};

pub struct DataManager;

pub enum DataAction {
    Add,
    Delete,
    Move,
    Import,
    Alias,
    Scan,
    Edit,
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
            Commands::Scan { .. } => DataAction::Scan,
            Commands::Edit { .. } => DataAction::Edit,
            _ => DataAction::Default,
        }
    }
}

impl DataManager {
    pub fn match_action(&mut self, action: DataAction, args: &SubArgs) -> Result<(), io::Error> {
        let mut data = self.parse_json_data().unwrap_or_else(|_| DataModel::new());
        let current_dir = Utf8PathBuf::from_path_buf(env::current_dir().unwrap_or_default())
            .expect("valid Unicode path succeeded");
        match action {
            DataAction::Add => match data.object_by_source_mut(current_dir.clone()) {
                Ok(obj) => {
                    self.print_rule_info(args);
                    match self.add_rule_to_json(
                        obj,
                        args.secondary_path.to_string(),
                        args.keyword.clone(),
                    ) {
                        Ok(()) => {}
                        Err(e) => {
                            eprintln!("Error: {}", e);
                            process::exit(1);
                        }
                    };
                    self.save_json_data(&data)?;
                }
                Err(_) => {
                    self.set_new_rules(
                        &mut data,
                        args.keyword.clone(),
                        current_dir,
                        args.secondary_path.clone(),
                    );
                    self.save_json_data(&data)?;
                }
            },
            DataAction::Delete => match data.object_by_source_mut(current_dir) {
                Ok(obj) => match self.remove_rule_from_json(obj, args.keyword.as_str()) {
                    Ok(_) => {
                        if menu::get_yn_input() {
                            match self.save_json_data(&data) {
                                Ok(()) => {
                                    println!("deleted rule successfully.")
                                }
                                Err(e) => {
                                    eprintln!("{}", e);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("{}", e)
                    }
                },
                Err(_) => {
                    return Err(io::Error::new(
                        io::ErrorKind::NotFound,
                        "[?] no rule for the current path in the data",
                    ));
                }
            },
            DataAction::Scan => match data.object_by_source_mut(current_dir) {
                Ok(obj) => {
                    let maps = self.scan_current_path(obj, args.keyword.as_str())?;
                    if maps.is_empty() {
                        println!("\x1b[0;32m[✓]\x1b[0m no entries to move. ")
                    } else {
                        println!("ENTRIES IN SOURCE: ");
                        for entries in maps {
                            println!(" TARGET: {}", entries.0);
                            for entry in entries.1 {
                                let entry_symbol = menu::entry_symbol(&entry);
                                println!("  {} {}", entry_symbol, entry)
                            }
                            println!()
                        }
                        match menu::get_mq_input() {
                            true => {
                                if let Err(err) = self.rename_entries(obj, &args.keyword) {
                                    println!("Error moving entries: {}", err);
                                }
                            }
                            false => {}
                        }
                    }
                }
                Err(_) => {
                    return Err(io::Error::new(
                        io::ErrorKind::NotFound,
                        "[?] no rule for the current path in the data",
                    ));
                }
            },
            DataAction::Move => {
                if let Ok(obj) = data.object_by_source(current_dir) {
                    self.rename_entries(obj, args.keyword.as_str())?;
                }
            }
            DataAction::Import => {
                println!("{:?}", args);
                match self.import_rule(&mut data, args.keyword.clone(), args.secondary_path.clone())
                {
                    Ok(()) => {}
                    Err(e) => {
                        eprintln!("Error: {}", e)
                    }
                }
            }
            DataAction::Edit => {
                self.edit_rule(&mut data, args.keyword.clone(), args.secondary_path.clone())
            }
            DataAction::Alias => {
                if let Ok(obj) = data.object_by_source_mut(current_dir) {
                    self.set_alias(obj, args.keyword.clone());
                    match self.save_json_data(&data) {
                        Ok(()) => {}
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

    fn print_rule_info(&self, args: &SubArgs) {
        let mut source_path = Utf8PathBuf::new();
        let mut target_path = Utf8PathBuf::new();

        source_path.push(args.primary_path.clone());
        target_path.push(args.secondary_path.clone());

        let keyword = &args.keyword;

        println!(" KEYWORD: {}", keyword);
        println!(" SOURCE : \x1b[4m{}\x1b[0m", source_path);
        println!(" TARGET : └─> \x1b[4m{}\x1b[0m \n", target_path);
    }
}

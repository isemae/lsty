use crate::{
    cli::{
        cli_format::{msg_format, MsgArgs, MsgKind},
        menu,
        status_symbols::{status_symbol, Status::*},
    },
    commands::{Commands, SubArgs},
    data::model::DataModel,
};
use camino::Utf8PathBuf;
use serde::ser::Error;
use std::{
    env,
    fs::File,
    io::{self, prelude::*},
};

use super::model::DataObject;

pub struct DataManager;

pub enum DataAction {
    Add,
    Delete,
    Move,
    Import,
    Alias,
    Scan,
    List,
    Edit,
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
            Commands::List { .. } => DataAction::List,
            Commands::Edit { .. } => DataAction::Edit,
        }
    }
}

impl DataManager {
    pub fn match_action(&mut self, action: DataAction, args: &SubArgs) -> Result<(), io::Error> {
        let mut data = self.parse_json_data().unwrap_or_else(|_| DataModel::new());
        let current_dir = Utf8PathBuf::from_path_buf(env::current_dir().unwrap_or_default())
            .expect("valid Unicode path succeeded");
        match action {
            DataAction::Add => {
                match data.object_by_source_mut(current_dir.clone()) {
                    Err(_) => {
                        self.set_empty_rule(
                            &mut data,
                            args.keyword.clone(),
                            current_dir,
                            args.secondary_path.clone(),
                        );
                    }
                    Ok(obj) => {
                        println!(
                            "{}",
                            msg_format(MsgKind::RuleInfo(MsgArgs {
                                primary_keyword: args.keyword.clone(),
                                primary_path: args.primary_path.clone(),
                                secondary_path: args.secondary_path.clone(),
                                ..Default::default()
                            },),)
                        );
                        self.add_rule_to_json(
                            obj,
                            args.secondary_path.to_string(),
                            args.keyword.clone(),
                        )?;
                    }
                }
                self.save_json_data(&data)?;
            }
            DataAction::Delete => match data.object_by_source_mut(current_dir) {
                Err(e) => {
                    eprintln!("{}", e)
                }
                Ok(obj) => {
                    if obj.targets.get(&args.keyword).is_some() {
                        self.remove_rule_from_json(obj, args.keyword.as_str())?;
                        println!("deleted rule successfully.");
                        self.save_json_data(&data)?;
                    } else {
                        let mut keys: Vec<_> = obj.targets.keys().cloned().collect();
                        keys.sort();
                        return Err(io::Error::new(
                            io::ErrorKind::NotFound,
                            msg_format(MsgKind::NoRuleShowAvailable(MsgArgs {
                                primary_keyword: keys.join("\n"),
                                ..Default::default()
                            })),
                        ));
                    }
                }
            },
            DataAction::Scan => match data.object_by_source_mut(current_dir) {
                Err(_) => {
                    return Err(io::Error::new(
                        io::ErrorKind::NotFound,
                        format!(
                            "{} no rule for the current path in the data",
                            status_symbol(&Error)
                        ),
                    ));
                }
                Ok(obj) => {
                    let maps = self.scan_current_path(obj, args.keyword.as_str())?;
                    if maps.is_empty() {
                        println!("{} no entries to move.", status_symbol(&Safe))
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
                        if menu::get_mq_input() {
                            match self.rename_entries(obj, &args.keyword) {
                                Ok(_) => {}
                                Err(err) => println!("Error moving entries: {}", err),
                            }
                        }
                    }
                }
            },
            DataAction::List => match data.object_by_source_mut(current_dir.clone()) {
                Err(_) => {}
                Ok(obj) => {
                    let mut keys: Vec<_> = obj.targets.keys().cloned().collect();
                    keys.sort();
                    println!(
                        "{}",
                        msg_format(MsgKind::ListRule(MsgArgs {
                            primary_keyword: keys.join("\n"),
                            ..Default::default()
                        })),
                    )
                }
            },
            DataAction::Move => {
                if let Ok(obj) = data.object_by_source(current_dir) {
                    self.rename_entries(obj, args.keyword.as_str())?;
                }
            }
            DataAction::Import => {
                if let Err(e) =
                    self.import_rule(&mut data, args.keyword.clone(), args.secondary_path.clone())
                {
                    eprintln!("Error: {}", e)
                }
            }
            DataAction::Edit => match data.object_by_source_mut(current_dir) {
                Err(_) => {
                    eprintln!("no such rule for the keyword");
                }
                Ok(obj) => {
                    if obj.targets.get(&args.keyword).is_some() {
                        self.edit_rule(obj, args.keyword.clone(), args.secondary_path.clone());
                        self.save_json_data(&data).expect("");
                    } else {
                        return Err(io::Error::new(
                            io::ErrorKind::NotFound,
                            msg_format(MsgKind::NoRuleShowAvailable(MsgArgs {
                                primary_keyword: obj
                                    .targets
                                    .keys()
                                    .cloned()
                                    .collect::<Vec<_>>()
                                    .join("\n"),
                                ..Default::default()
                            })),
                        ));
                    }
                }
            },
            DataAction::Alias => {
                if let Some(o) = data.data.iter().find(|o| o.alias == args.keyword) {
                    return Err(io::Error::new(
                        io::ErrorKind::AlreadyExists,
                        msg_format(MsgKind::ExistingAlias(MsgArgs {
                            primary_keyword: args.keyword.clone(),
                            primary_path: o.source.clone(),
                            ..Default::default()
                        })),
                    ));
                } else if let Ok(obj) = data.object_by_source_mut(current_dir) {
                    match self.set_alias(obj, args.keyword.clone()) {
                        Err(e) => {
                            eprintln!("{}", e)
                        }
                        Ok(_) => {
                            println!(
                                "{}",
                                msg_format(MsgKind::UpdatedAlias(MsgArgs {
                                    primary_keyword: obj.alias.clone(),
                                    secondary_keyword: args.keyword.clone(),
                                    ..Default::default()
                                }),)
                            );
                            self.save_json_data(&data)?
                        }
                    }
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
            Err(e) => Err(serde_json::Error::custom(format!(
                "failed to load data file: {}",
                e
            ))),
            Ok(mut file) => {
                let mut data = String::new();
                match file.read_to_string(&mut data) {
                    Ok(..) => {}
                    Err(e) => eprintln!("Error: {}", e),
                }
                serde_json::from_str(data.as_str())
            }
        }
    }

    pub fn save_json_data(&self, data: &DataModel) -> Result<(), io::Error> {
        let exe_path = env::current_exe().unwrap();
        let exe_dir = exe_path.parent().unwrap();

        let mut file = File::create(exe_dir.join("lsty.json"))?;
        serde_json::to_writer_pretty(&mut file, data)?;
        Ok(())
    }
}

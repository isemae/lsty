use crate::{
    cli::{
        cli_format::{msg_format, MsgArgs, MsgKind},
        menu,
        status_symbols::{status_symbol, Status::*},
    },
    commands::{Commands, SubArgs},
    data::{json_manager, model::DataModel},
};
use camino::Utf8PathBuf;
use std::{collections::HashMap, env, io};

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
        let json = json_manager::JsonManager;
        let mut data = json.parse_json_data().unwrap_or_else(|_| DataModel::new());
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
                            }))
                        );
                        self.add_rule_to_json(
                            obj,
                            args.secondary_path.to_string(),
                            args.keyword.clone(),
                        )?;
                    }
                }
                json.save_json_data(&data)?;
            }
            DataAction::Delete => match data.object_by_source_mut(current_dir) {
                Err(e) => {
                    eprintln!("{}", e)
                }
                Ok(obj) => {
                    if obj.targets.get(&args.keyword).is_some() {
                        self.remove_rule_from_json(obj, args.keyword.as_str())?;
                        println!("deleted rule successfully.");
                        json.save_json_data(&data)?;
                    } else {
                        let targets = &obj.targets;
                        let pairs = self.handle_pairs_list(targets);
                        return Err(io::Error::new(
                            io::ErrorKind::NotFound,
                            msg_format(MsgKind::NoRuleShowAvailable(MsgArgs {
                                primary_keyword: pairs,
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
                    let targets = &obj.targets;
                    let pairs = self.handle_pairs_list(targets);
                    println!(
                        "{}",
                        msg_format(MsgKind::ListRule(MsgArgs {
                            primary_keyword: pairs,
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
                match self.import_rule(&mut data, args.keyword.clone(), args.secondary_path.clone())
                {
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                    Ok(()) => json.save_json_data(&data)?,
                }
            }
            DataAction::Edit => match data.object_by_source_mut(current_dir) {
                Err(_) => {
                    eprintln!("no such rule for the keyword");
                }
                Ok(obj) => {
                    let targets = &obj.targets;
                    if targets.get(&args.keyword).is_some() {
                        self.edit_rule(obj, args.keyword.clone(), args.secondary_path.clone());
                        json.save_json_data(&data).expect("");
                    } else {
                        let pairs = self.handle_pairs_list(targets);
                        return Err(io::Error::new(
                            io::ErrorKind::NotFound,
                            msg_format(MsgKind::NoRuleShowAvailable(MsgArgs {
                                primary_keyword: pairs,
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
                            json.save_json_data(&data)?
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn handle_pairs_list(&self, pairs: &HashMap<String, String>) -> String {
        let mut result = String::new();
        let mut sorted: Vec<(&String, &String)> = pairs.iter().collect();
        sorted.sort_by_key(|&(k, _)| k);

        for &(k, v) in &sorted {
            result.push_str(&format!(" '{}' ~> '{}'\n", k, v));
        }
        result
    }
}

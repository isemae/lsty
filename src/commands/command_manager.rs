use camino::Utf8PathBuf;

use super::arguments::{Commands, Config, CopyArg, DelArgs, SubArgs};
use crate::{
    cli::menu::{navigate_menu, MenuAction},
    data::{
        data_manager::{self, DataAction, DataManager},
        model::DataModel,
    },
};
use std::{
    io::{self, Result},
    path::{Path, PathBuf},
};

pub fn process_command(config: &Config) -> Result<()> {
    let mut data_manager = DataManager::new();
    let mut action;

    match &config.command {
        Commands::Add {
            keyword,
            source_path,
            target_path,
        } => {
            let sub_args = &SubArgs {
                keyword: keyword.to_string(),
                source_path,
                target_path,
            };
            action = DataAction::Add;
            data_manager.match_action(action, sub_args)
        }

        Commands::Del {
            keyword,
            target_path,
        } => {
            let sub_args = &SubArgs {
                keyword: keyword.as_deref().unwrap_or("").to_string(),
                source_path: &Utf8PathBuf::default(),
                target_path: target_path.as_ref().unwrap(),
            };
            data_manager.match_action(DataAction::Delete, &sub_args)
        }
        Commands::Move { keyword } => {
            let sub_args = &SubArgs {
                keyword: keyword.to_string(),
                source_path: &Utf8PathBuf::default(),
                target_path: &Utf8PathBuf::default(),
            };
            data_manager.match_action(DataAction::Move, &sub_args)
        }
        //
        Commands::Scan {} => Ok(()),

        Commands::List => {
            navigate_menu(MenuAction::Default);
            Ok(())
        }
        Commands::Copy { source_path } => Ok(()),
        _ => Ok(()),
    }
}
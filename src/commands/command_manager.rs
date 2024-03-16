use camino::Utf8PathBuf;

use super::arguments::{Commands, Config, SubArgs};
use crate::{
    cli::menu::{navigate_menu, MenuAction},
    data::data_manager::{DataAction, DataManager},
};
use std::{io::Result, path::PathBuf};

pub fn process_command(config: &Config) -> Result<()> {
    let mut data_manager = DataManager;
    let current_path = std::env::current_dir().unwrap_or(PathBuf::from(""));
    let default_path = &Utf8PathBuf::from_path_buf(current_path).unwrap_or(Utf8PathBuf::from(""));

    match &config.command {
        Commands::Add {
            keyword,
            target_path,
        } => {
            let sub_args = &SubArgs {
                keyword: keyword.to_string(),
                source_path: &default_path,
                target_path: &target_path.clone().unwrap_or(Utf8PathBuf::from("")),
            };
            DataManager::match_action(&mut data_manager, DataAction::Add, sub_args)
        }

        Commands::Del { keyword } => {
            let sub_args = &SubArgs {
                keyword: keyword.clone().unwrap_or_default(),
                source_path: &default_path,
                target_path: &default_path,
            };
            DataManager::match_action(&mut data_manager, DataAction::Delete, &sub_args)
        }
        Commands::Move {
            keyword,
            target_path,
        } => {
            let sub_args = &SubArgs {
                keyword: keyword.clone().unwrap_or_default(),
                source_path: &default_path,
                target_path: target_path.as_ref().unwrap_or(default_path),
            };
            DataManager::match_action(&mut data_manager, DataAction::Move, &sub_args);
            Ok(())
        }
        //
        Commands::Scan {
            keyword,
            source_path,
            target_path,
        } => {
            let sub_args = &SubArgs {
                keyword: keyword.clone().unwrap_or_default(),
                source_path: &default_path,
                target_path: target_path.as_ref().unwrap_or(default_path),
            };
            DataManager::match_action(&mut data_manager, DataAction::Scan, &sub_args);
            Ok(())
        }

        Commands::List => {
            navigate_menu(MenuAction::Default);
            Ok(())
        }
        Commands::Copy { source_path } => Ok(()),
        _ => Ok(()),
    }
}

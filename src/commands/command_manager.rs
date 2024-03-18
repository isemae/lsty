use camino::Utf8PathBuf;

use super::arguments::{Commands, Config, SubArgs};
use crate::{
    cli::menu::{navigate_menu, MenuAction},
    data::data_manager::{DataAction, DataManager},
};
use std::{io::Result, path::PathBuf};

pub fn process_command(config: &Config) -> Result<()> {
    let mut data_manager = DataManager;
    let current_path = std::env::current_dir().unwrap_or_default();
    let default_path = &Utf8PathBuf::from_path_buf(current_path).unwrap_or_default();
    println!("{:?}", config);
    config.command.process(&mut data_manager, default_path);

    //     //
    //     Commands::Scan {
    //         keyword,
    //         source_path,
    //         target_path,
    //     } => {
    //         let sub_args = &SubArgs {
    //             keyword: keyword.clone().unwrap_or_default(),
    //             source_path: &default_path,
    //             target_path: target_path.as_ref().unwrap_or(default_path),
    //         };
    //         DataManager::match_action(&mut data_manager, DataAction::Scan, &sub_args);
    //         Ok(())
    //     }

    //     Commands::List => {
    //         navigate_menu(MenuAction::Default);
    //         Ok(())
    //     }
    //     Commands::Copy { alias, target_path } => {
    //         let sub_args = &SubArgs {
    //             keyword: "".to_string(),
    //             source_path: &default_path,
    //             target_path: target_path.as_ref().unwrap_or(default_path),
    //         };
    //         DataManager::match_action(&mut data_manager, DataAction::Copy, &sub_args);
    //         Ok(())
    //     }
    //     _ => Ok(()),
    // }
    Ok(())
}

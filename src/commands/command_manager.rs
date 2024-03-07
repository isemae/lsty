use super::arguments::{Cli, Commands, DelArgs, SubArgs};
use crate::{
    data::{
        data_manager::{self, DataAction, DataManager},
        model::DataModel,
    },
    interfaces::menu::{navigate_menu, MenuAction},
};
use camino::{Utf8Path, Utf8PathBuf};
use std::{
    io::{self, Result},
    path::{Path, PathBuf},
};

pub fn process_command(cli: &Cli) {
    let mut data_manager = DataManager::new("./data.json");

    match &cli.command {
        Commands::Add(subargs) => data_manager.match_action(DataAction::Add, cli),
        Commands::Del(delargs) => data_manager.match_action(DataAction::Delete, cli),
        Commands::Scan => {}
        Commands::Move(keyword) => {
            data_manager.match_action(DataAction::Delete, cli);
        }
        Commands::List => {
            navigate_menu(MenuAction::Default);
        }
        _ => {
            eprintln!("unknown command");
            std::process::exit(1);
        }
    }
}

use crate::data::data_manager::{DataAction, DataManager};
use camino::Utf8PathBuf;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::io;
use strum_macros::{EnumString, VariantNames};

#[derive(Parser, Debug)]
pub struct Config {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug, EnumString, VariantNames)]
#[strum(serialize_all = "lowercase")]
pub enum Commands {
    #[command(alias = "-a")]
    #[strum(serialize = "add")]
    Add {
        keyword: String,
        target_path: Option<Utf8PathBuf>,
    },
    #[command(alias = "-d")]
    #[strum(serialize = "delete")]
    Del {
        #[arg(value_name = "KEYWORD")]
        keyword: Option<String>,
    },
    #[command(alias = "-m")]
    #[strum(serialize = "move")]
    Move {
        keyword: Option<String>,
        target_path: Option<Utf8PathBuf>,
    },
    Scan {
        keyword: Option<String>,
        source_path: Option<Utf8PathBuf>,
        target_path: Option<Utf8PathBuf>,
    },
    #[command(alias = "-c")]
    #[strum(serialize = "copy")]
    Copy {
        target_path: Option<Utf8PathBuf>,
        alias: Option<String>,
    },
    #[command(alias = "-l")]
    #[strum(serialize = "list")]
    List,
}

#[derive(Debug)]
pub struct SubArgs<'a> {
    pub keyword: String,
    pub source_path: &'a Utf8PathBuf,
    pub target_path: Utf8PathBuf,
}

impl<'a> SubArgs<'a> {
    pub fn new(keyword: String, source_path: &'a Utf8PathBuf, target_path: Utf8PathBuf) -> Self {
        SubArgs {
            keyword,
            source_path,
            target_path,
        }
    }
}

impl Commands {
    pub fn process(
        &self,
        data_manager: &mut DataManager,
        default_path: &Utf8PathBuf,
    ) -> Result<(), io::Error> {
        let sub_args = match self {
            Commands::Add {
                keyword,
                target_path,
            } => SubArgs::new(
                keyword.to_string(),
                default_path,
                target_path.clone().unwrap_or(Utf8PathBuf::default()),
            ),

            Commands::Del { keyword } => SubArgs::new(
                keyword.clone().unwrap_or_default(),
                default_path,
                default_path.clone(),
            ),

            Commands::Move {
                keyword,
                target_path,
            } => SubArgs::new(
                keyword.clone().unwrap_or_default(),
                default_path,
                target_path.clone().unwrap_or(Utf8PathBuf::default()),
            ),
            Commands::Copy { target_path, alias } => SubArgs::new(
                alias.clone().unwrap_or_default(),
                default_path,
                target_path.clone().unwrap_or(Utf8PathBuf::default()),
            ),
            _ => SubArgs::new("".to_string(), default_path, default_path.clone()),
        };
        data_manager.match_action(DataAction::from(self), &sub_args);
        Ok(())
    }
}

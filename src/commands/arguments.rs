use crate::{
    cli::menu,
    data::data_manager::{DataAction, DataManager},
};
use camino::Utf8PathBuf;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::{io, process};
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

    #[command(alias = "-s")]
    #[strum(serialize = "scan")]
    Scan {
        keyword: Option<String>,
        source_path: Option<Utf8PathBuf>,
        target_path: Option<Utf8PathBuf>,
    },

    #[command(alias = "-i")]
    #[strum(serialize = "import")]
    Import { alias_or_source: String },

    #[command(alias = "-al")]
    #[strum(serialize = "alias")]
    Alias { alias: Option<String> },

    #[command(alias = "-l")]
    #[strum(serialize = "list")]
    List,
}

#[derive(Debug)]
pub struct SubArgs {
    pub keyword: String,
    pub primary_path: Utf8PathBuf,
    pub secondary_path: Utf8PathBuf,
}

impl SubArgs {
    pub fn new(keyword: String, primary_path: Utf8PathBuf, secondary_path: Utf8PathBuf) -> Self {
        SubArgs {
            keyword,
            primary_path,
            secondary_path,
        }
    }
}

impl Commands {
    pub fn process(
        &self,
        data_manager: &mut DataManager,
        default_path: Utf8PathBuf,
    ) -> Result<(), io::Error> {
        let sub_args = &match self {
            Commands::Add {
                keyword,
                target_path,
            } => SubArgs::new(
                keyword.to_lowercase(),
                default_path,
                if target_path.is_some() {
                    target_path.clone().unwrap()
                } else {
                    println!("target path is not provided. set the target path to the keyword in the current directory? (y/N):");
                    match menu::get_yn_input() {
                        true => {
                            println!(
                                "Note: the actual directory doesn't exist yet. it will be created later when the files are being moved."
                            );
                            target_path
                                .clone()
                                .unwrap_or(Utf8PathBuf::from(format!("./{}", &keyword)))
                        }
                        false => {
                            eprintln!("target path input should be given.");
                            process::exit(1)
                        }
                    }
                },
            ),

            Commands::Del { keyword } => SubArgs::new(
                keyword.clone().unwrap_or_default(),
                default_path.clone(),
                default_path,
            ),
            Commands::Move {
                keyword,
                target_path,
            } => SubArgs::new(
                keyword.clone().unwrap_or_default(),
                default_path,
                target_path.clone().unwrap_or(Utf8PathBuf::default()),
            ),

            Commands::Import { alias_or_source } => {
                if Utf8PathBuf::from(alias_or_source).is_dir() {
                    SubArgs::new(
                        String::from(""),
                        default_path,
                        Utf8PathBuf::from(alias_or_source.clone()),
                    )
                } else {
                    SubArgs::new(alias_or_source.clone(), default_path, Utf8PathBuf::new())
                }
            }

            Commands::Alias { alias } => SubArgs::new(
                alias.clone().unwrap_or_default(),
                default_path.clone(),
                default_path,
            ),
            _ => SubArgs::new("".to_string(), default_path.clone(), default_path),
        };

        data_manager
            .match_action(DataAction::from(self), sub_args)
            .expect("failed to match data action.");
        Ok(())
    }
}

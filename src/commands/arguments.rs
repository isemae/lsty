use camino::Utf8PathBuf;
use clap::{Parser, Subcommand};

use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
pub struct Config {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(alias = "-a")]
    Add {
        keyword: String,
        target_path: Utf8PathBuf,
    },
    #[command(alias = "-d")]
    Del {
        #[arg(value_name = "KEYWORD")]
        keyword: Option<String>,
    },
    #[command(alias = "-m")]
    Move {
        keyword: Option<String>,
        target_path: Option<Utf8PathBuf>,
    },
    Scan,
    Copy {
        source_path: Utf8PathBuf,
    },
    List,
}

#[derive(Debug)]
pub struct SubArgs<'a> {
    pub keyword: String,
    pub source_path: &'a Utf8PathBuf,
    pub target_path: &'a Utf8PathBuf,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SymlinkInfo {
    pub original_path: String,
    pub link_path: String,
}

impl Config {
    // pub fn get_arg(&self) -> Option<&dyn Args> {
    //     match &self.command {
    //         Commands::Add(subargs) => Some(subargs as &dyn Args),
    //         Commands::Del(delargs) => Some(delargs as &dyn Args),
    //         Commands::Keyword(keyword) => Some(keyword as &dyn Args),
    //         Commands::Copy(copy_arg) => Some(copy_arg as &dyn Args),
    //         _ => None,
    //     }
    // }

    // pub fn get_keyword(&self) -> Option<&str> {
    //     self.get_subarg().map(|subargs| subargs.keyword.as_str())
    // }

    // pub fn get_source_path(&self) -> Option<&Utf8PathBuf> {
    //     self.get_subarg().map(|subargs| &subargs.source_path)
    // }

    // pub fn get_target_path(&self) -> Option<&Utf8PathBuf> {
    //     self.get_subarg().map(|subargs| &subargs.target_path)
    // }

    // // pub fn get_action(&self) -> Option<&Actions> {
    // //     self.get_subarg().map(|subargs| &subargs.action)
    // // }
}

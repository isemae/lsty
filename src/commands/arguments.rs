use camino::Utf8PathBuf;
use clap::{builder, ArgGroup, Args, Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};
use std::env;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Add(SubArgs),
    Del(DelArgs),
    Move(Keyword),
    Scan,
    List,
}

#[derive(Debug, Args)]
pub struct SubArgs {
    pub keyword: String,
    pub source_path: Utf8PathBuf,
    pub target_path: Utf8PathBuf,
}

#[derive(Debug, Args)]
pub struct Keyword {
    pub keyword: String,
}

#[derive(Debug, Args)]
pub struct DelArgs {
    pub keyword: Option<String>,
    pub target_path: Option<Utf8PathBuf>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SymlinkInfo {
    pub original_path: String,
    pub link_path: String,
}

impl Cli {
    pub fn get_subarg(&self) -> Option<&SubArgs> {
        if let Commands::Add(subargs) = &self.command {
            Some(subargs)
        } else {
            None
        }
    }
    pub fn get_keyword(&self) -> Option<&str> {
        self.get_subarg().map(|subargs| subargs.keyword.as_str())
    }

    pub fn get_source_path(&self) -> Option<&Utf8PathBuf> {
        self.get_subarg().map(|subargs| &subargs.source_path)
    }

    pub fn get_target_path(&self) -> Option<&Utf8PathBuf> {
        self.get_subarg().map(|subargs| &subargs.target_path)
    }

    // pub fn get_action(&self) -> Option<&Actions> {
    //     self.get_subarg().map(|subargs| &subargs.action)
    // }
}

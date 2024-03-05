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
    Source(Action),
    Target(Action),
    Pair(Action),
    Scan,
    Move(Keyword),
    List,
}

#[derive(Debug, Args)]
pub struct Action {
    #[command(subcommand)]
    pub action: Actions,
    pub path: Utf8PathBuf,
    pub keyword: String,
    pub opt_path: Option<Utf8PathBuf>,
}

#[derive(Debug, Args)]
pub struct Keyword {
    pub keyword: String,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Actions {
    Add,
    Delete,
    List,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SymlinkInfo {
    pub original_path: String,
    pub link_path: String,
}

impl Cli {
    pub fn get_subarg(&self) -> Option<&Action> {
        if let Commands::Source(subargs) | Commands::Target(subargs) | Commands::Pair(subargs) =
            &self.command
        {
            Some(subargs)
        } else {
            None
        }
    }
    pub fn get_keyword(&self) -> Option<&str> {
        self.get_subarg().map(|subargs| subargs.keyword.as_str())
    }

    pub fn get_dir(&self) -> Option<&Utf8PathBuf> {
        self.get_subarg().map(|subargs| &subargs.path)
    }

    pub fn get_opt_path(&self) -> Option<&Option<Utf8PathBuf>> {
        self.get_subarg().map(|subargs| &subargs.opt_path)
    }

    pub fn get_action(&self) -> Option<&Actions> {
        self.get_subarg().map(|subargs| &subargs.action)
    }
}

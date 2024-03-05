use super::model::*;
use crate::{
    commands::arguments::{Cli, Commands},
    interfaces::menu,
};
use serde::de::Error;
use serde_json::{self, Result, Value};
use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{Read, Write},
};

impl DataModel {
    pub fn create_json(cli: &Cli, dir: String, opt_path: String, keyword: &str) -> Result<()> {
        let mut data = Self::parse_json();

        match &cli.command {
            Commands::Pair(subargs) => Self::add_pair_to_json(&mut data, &dir, &opt_path, keyword),
            Commands::Source(subargs) => Self::add_source_to_json(&mut data, dir, keyword),
            Commands::Target(subargs) => Self::add_target_to_json(&mut data, dir, keyword),
            _ => {}
        }

        let j = serde_json::to_string_pretty(&data)?;
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open("data.json")
            .map_err(|e| serde_json::Error::custom(e.to_string()))?;
        file.write_all(j.as_bytes());

        Ok(())
    }

    pub fn parse_json() -> DataModel {
        match File::open("data.json") {
            Ok(mut file) => {
                let mut data = String::new();
                file.read_to_string(&mut data).unwrap();
                serde_json::from_str(&data).unwrap()
            }
            Err(_) => DataModel {
                pairs: Vec::new(),
                sources: Vec::new(),
                targets: Vec::new(),
                // symlinks: Vec::new(),
            },
        }
    }

    pub fn add_source_to_json(data: &mut DataModel, dir: String, keyword: &str) {
        if let Some(source) = data
            .sources
            .iter_mut()
            .find(|source| source.source_path == dir)
        {
            if !source.keywords.contains(&keyword.to_string()) {
                source.keywords.push(keyword.to_string());
            }
        } else {
            let new_source = Source {
                source_path: dir,
                keywords: vec![keyword.to_string()],
            };
            data.sources.push(new_source);
        }
    }

    pub fn add_target_to_json(data: &mut DataModel, dir: String, keyword: &str) {
        if let Some(target) = data
            .targets
            .iter_mut()
            .find(|target| target.target_path == dir)
        {
            if !target.keyword.contains(&keyword.to_string()) {
                target.keyword = keyword.to_string();
            }
        } else {
            let new_target = Target {
                target_path: dir,
                keyword: keyword.to_string(),
            };
            data.targets.push(new_target);
        }
    }

    pub fn add_pair_to_json(
        data: &mut DataModel,
        source_path: &str,
        target_path: &str,
        keyword: &str,
    ) {
        println!("{}", keyword);
        println!("source: {}", source_path);
        println!("target: {}", target_path);
        if let Some(pair) = data
            .pairs
            .iter_mut()
            .find(|pair| pair.source_path == source_path)
        {
            let target_exist = pair
                .source_targets
                .iter()
                .any(|target| target.target == target_path && target.keyword == keyword);

            if !target_exist {
                pair.source_targets.push(SourceTarget {
                    target: target_path.to_string(),
                    keyword: keyword.to_string(),
                })
            }
        } else {
            let new_pair = Pair {
                source_path: source_path.to_string(),
                source_targets: vec![SourceTarget {
                    target: target_path.to_string(),
                    keyword: keyword.to_string(),
                }],
            };
            data.pairs.push(new_pair);
        }
    }

    pub fn remove_json(keyword: &str, command: &Commands) -> std::io::Result<()> {
        let file = File::open("data.json")?;
        let mut data: serde_json::Value = serde_json::from_reader(file)?;

        match command {
            Commands::Source(_) => {
                println!("Are you sure you want to delete current directory from sources? (y/n):")
            }
            Commands::Target(_) => {
                println!("Are you sure you want to delete current directory from targets? (y/n):")
            }
            _ => println!(""),
        }

        match command {
            Commands::Target(_) => {
                if let Some(targets) = data.get_mut("targets").and_then(|t| t.as_array_mut()) {
                    targets
                        .retain(|target| target["keyword"] != Value::String(keyword.to_string()));
                }
            }
            Commands::Source(_) => {
                if let Some(sources) = data.get_mut("sources").and_then(|s| s.as_array_mut()) {
                    sources.retain(|source| {
                        !source["keywords"]
                            .as_array()
                            .map(|keywords| {
                                keywords
                                    .iter()
                                    .any(|k| k == &Value::String(keyword.to_string()))
                            })
                            .unwrap_or(false)
                    });
                }
            }
            Commands::Pair(_) => {
                if let Some(pairs) = data.get_mut("pairs").and_then(|p| p.as_array_mut()) {
                    pairs.retain(|pair| {
                        pair["source_targets"]
                            .as_array()
                            .map(|targets| {
                                !targets
                                    .iter()
                                    .any(|t| t["keyword"] == Value::String(keyword.to_string()))
                            })
                            .unwrap_or(false)
                    });
                }
            }
            _ => {
                eprintln!("Invalid Command");
            }
        }

        let mut file = File::create("data.json")?;
        serde_json::to_writer_pretty(&mut file, &data)?;

        Ok(())
    }

    //// for printing cli menu
    // pub fn pairs(&self) -> &Vec<Pair> {
    //     &self.pairs
    // }
    // pub fn sources(&self) -> &Vec<Source> {
    //     &self.sources
    // }
    // pub fn targets(&self) -> &Vec<Target> {
    //     &self.targets
    // }
}

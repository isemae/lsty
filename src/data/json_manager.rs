use super::model::*;
use crate::{
    cli::menu,
    commands::arguments::{Commands, Config},
};
use camino::{Utf8Path, Utf8PathBuf};
use serde::{Deserialize, Serialize};
use serde_json::{self, Value};
use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{self, Error, Read, Write},
};

pub struct JsonManager;

impl JsonManager {}

// impl DataModel {
//     pub fn create_json(
//         source_path_string: String,
//         target_path_string: String,
//         keyword: &str,
//     ) -> Result<()> {
//         let mut data = Self::parse_json();

//         Self::add_lst_to_json(&mut data, &source_path_string, &target_path_string, keyword);
//         // Commands::Source(subargs) => Self::add_source_to_json(&mut data, dir, keyword),
//         // Commands::Target(subargs) => Self::add_target_to_json(&mut data, dir, keyword),

//         let j = serde_json::to_string_pretty(&data)?;
//         let mut file = OpenOptions::new()
//             .write(true)
//             .create(true)
//             .open("data.json")
//             .map_err(|e| serde_json::Error::custom(e.to_string()))?;
//         file.write_all(j.as_bytes());

//         Ok(())
//     }

//     pub fn parse_json() -> DataModel {
//         match File::open("data.json") {
//             Ok(mut file) => {
//                 let mut data = String::new();
//                 file.read_to_string(&mut data).unwrap();
//                 serde_json::from_str(&data).unwrap()
//             }
//             Err(_) => DataModel {
//                 pairs: Vec::new(),
//                 sources: Vec::new(),
//                 targets: Vec::new(),
//                 // symlinks: Vec::new(),
//             },
//         }
//     }

// pub fn load_data() -> std::io::Result<()> {
//     let file = File::open("data.json")?;
//     let mut data: serde_json::Value = serde_json::from_reader(file)?;
//     Ok(())
// }

// // for printing cli menu
// pub fn pairs(&self) -> &Vec<Pair> {
//     &self.pairs
// }
// pub fn sources(&self) -> &Vec<Source> {
//     &self.sources
// }
// pub fn targets(&self) -> &Vec<Target> {
//     &self.targets
// }

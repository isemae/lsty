use crate::data::{data_manager::DataManager, model::DataModel};
use std::{collections::HashMap, env::current_dir, path::PathBuf};

impl DataManager {
    pub fn scan_and_validate_path(&self, data: DataModel) -> Option<HashMap<String, String>> {
        let current_dir = current_dir().unwrap_or_else(|_| PathBuf::from(""));

        if let Some(pair) = data.pairs.get(&current_dir.to_string_lossy().to_string()) {
            let mut valid_pair = HashMap::new();

            for map in pair {
                if PathBuf::from(map.1).exists() {
                    valid_pair.insert(map.0.clone(), map.1.clone());
                }
            }
            println!("{:?} valid", valid_pair);
            return Some(valid_pair);
        }
        None
    }
}

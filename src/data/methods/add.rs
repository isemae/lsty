use crate::{
    cli::menu,
    data::{data_manager::DataManager, model::DataModel},
};
use camino::{Utf8Path, Utf8PathBuf};
use std::{collections::HashMap, io};

impl DataManager {
    pub fn add_rule_to_json(
        &self,
        mut data: DataModel,
        source_path: String,
        mut target_path: String,
        keyword: String,
    ) -> io::Result<()> {
        if !target_path.is_empty() {
            target_path = target_path
        } else {
            target_path = format!("./{}", &keyword);
        }

        let target_path_on_volume = Utf8Path::new(target_path.as_str());
        if !target_path_on_volume.exists() || !target_path_on_volume.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "no such directory exists.",
            ));
        }

        if let Some(pair) = data.pairs.get_mut(&source_path) {
            if !pair.contains_key(&target_path) && !pair.contains_key(&keyword) {
                pair.insert(keyword, target_path);
            } else if pair.contains_key(&keyword) {
                eprintln!(
                  "rule for the target '{}' already exists. do you want to change the keyword? (y/N):",
                  target_path
              );
                if menu::get_yn_input() {
                    pair.insert(keyword, target_path);
                    println!("rule added.")
                }
            } else {
                eprintln!(
                  "rule for the keyword '{}' already exists. do you want to change the target? (y/N):",
                  keyword
              );
                if menu::get_yn_input() {
                    pair.insert(keyword, target_path);
                    println!("rule added.")
                }
            }
        } else {
            let mut new_pair = HashMap::new();
            new_pair.insert(keyword, target_path);
            data.pairs.insert(source_path, new_pair);
        }
        self.save_json_data(&data)?;
        Ok(())
    }
}

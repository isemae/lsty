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

        if let Some(obj) = data
            .data
            .iter_mut()
            .find(|o| o.sources.contains(&source_path))
        {
            if !obj.targets.contains_key(&target_path) && !obj.targets.contains_key(&keyword) {
                obj.targets.insert(keyword, target_path);
            } else if obj.targets.contains_key(&keyword) {
                eprintln!(
                  "rule for the target '{}' already exists. do you want to change the keyword? (y/N):",
                  target_path
              );
                if menu::get_yn_input() {
                    obj.targets.insert(keyword, target_path);
                    println!("rule added.")
                }
            } else {
                eprintln!(
                  "rule for the keyword '{}' already exists. do you want to change the target? (y/N):",
                  keyword
              );
                if menu::get_yn_input() {
                    obj.targets.insert(keyword, target_path);
                    println!("rule added.")
                }
            }
        } else {
            // let mut new_sources = Vec::new();
            let mut new_target_map = HashMap::new();
            new_target_map.insert(keyword, target_path);

            // data.data.append(other).push(new_sources, new_target_map);
        }
        self.save_json_data(&data)?;
        Ok(())
    }
}

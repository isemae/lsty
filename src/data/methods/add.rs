use crate::{
    cli::menu,
    data::{
        data_manager::DataManager,
        model::{DataModel, DataObject},
    },
};
use camino::{Utf8Path, Utf8PathBuf};
use std::{collections::HashMap, env::current_dir, io};

impl DataManager {
    pub fn add_rule_to_json(
        &self,
        mut data: DataModel,
        mut target_path: String,
        keyword: String,
    ) -> io::Result<()> {
        let source_path = Utf8PathBuf::from_path_buf(current_dir().unwrap_or_default())
            .expect("valid Unicode path succeeded");

        if target_path.is_empty() {
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
            .find(|o| o.source.contains(&source_path.to_string()))
        {
            if let Some(existing_target) = obj.targets.get(&keyword) {
                if existing_target == &target_path {
                    eprintln!(
                      "rule for the target '{}' already exists. do you want to change the keyword? (y/N):",
                      target_path
                  );
                } else {
                    eprintln!(
                        "Rule for the keyword '{}' already exists with a different target: '{}'. do you want to change the target path?",
                        keyword, existing_target
                    );
                }
                if menu::get_yn_input() {
                    obj.targets.insert(keyword.clone(), target_path);
                    println!("rule updated.");
                }
            } else {
                obj.targets.insert(keyword, target_path);
                println!("rule added.");
            }
        } else {
            self.set_new_rules(&mut data, keyword, source_path.to_string(), target_path);
        }
        self.save_json_data(&data)?;
        Ok(())
    }

    pub fn set_new_rules(
        &self,
        data: &mut DataModel,
        keyword: String,
        source_path: String,
        target_path: String,
    ) {
        let new_obj = DataObject {
            alias: String::new(),
            source: source_path.to_string(),
            targets: [(keyword, target_path)].iter().cloned().collect(),
        };
        data.data.push(new_obj)
    }
}

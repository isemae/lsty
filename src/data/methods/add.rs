use crate::data::{
    data_manager::DataManager,
    model::{DataModel, DataObject},
};
use camino::Utf8PathBuf;
use std::{env::current_dir, io, process};

impl DataManager {
    pub fn add_rule_to_json(
        &self,
        mut data: DataModel,
        target_path: String,
        keyword: String,
    ) -> io::Result<()> {
        let lowercase_keyword = keyword.to_lowercase();
        let source_path = Utf8PathBuf::from_path_buf(current_dir().unwrap_or_default())
            .expect("valid Unicode path succeeded");

        // case rule for the source exists
        if let Some(obj) = data.data.iter_mut().find(|o| o.source == source_path) {
            // key exists in the targets map
            if let Some(existing_target) = obj.targets.get(&lowercase_keyword) {
                if existing_target == &target_path {
                    println!("rule already exists.");
                    println!(
                        "Note: try \"lsty edit {}\" or \"lsty edit {}\" to edit the keyword or path.",
                        keyword, target_path
                    );
                    process::exit(1)
                }
            } else {
                obj.targets.insert(lowercase_keyword, target_path.clone());
                println!("rule added.");
            }

        // case rule for the source doesn't exist
        } else {
            self.set_new_rules(
                &mut data,
                lowercase_keyword,
                source_path.to_string(),
                target_path,
            );
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

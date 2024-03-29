use camino::Utf8PathBuf;

use crate::data::{
    data_manager::DataManager,
    model::{DataModel, DataObject},
};
use std::{io, process};

impl DataManager {
    pub fn add_rule_to_json(
        &self,
        data: &mut DataObject,
        target_path: String,
        keyword: String,
    ) -> io::Result<()> {
        if let Some(existing_target) = data.targets.get(&keyword) {
            if existing_target == &target_path {
                println!("rule already exists.");
                println!(
                    "Note: try \"lsty edit {}\" or \"lsty edit {}\" to edit the keyword or path.",
                    keyword, target_path
                );
                process::exit(1)
            }
        } else {
            data.targets.insert(keyword, target_path.clone());
            println!("rule added.");
        }
        // } else {

        Ok(())
    }

    pub fn set_new_rules(
        &self,
        data: &mut DataModel,
        keyword: String,
        source_path: Utf8PathBuf,
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

use camino::Utf8PathBuf;

use crate::{
    cli::cli_format::{message_format, MessageArgs, MessageKind},
    data::{
        data_manager::DataManager,
        model::{DataModel, DataObject},
    },
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
            if existing_target == &target_path || data.targets.contains_key(&keyword) {
                println!(
                    "{}",
                    message_format(
                        MessageKind::AlreadyExistsTryEdit,
                        MessageArgs {
                            primary_keyword: keyword,
                            primary_path: target_path,
                            ..Default::default()
                        }
                    )
                );
                process::exit(1)
            }
        } else {
            data.targets.insert(keyword, target_path.clone());
            println!("rule added.");
        }
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

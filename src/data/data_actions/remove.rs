use crate::{
    cli::{
        cli_format::{message_format, MessageArgs, MessageKind},
        menu,
    },
    data::{data_manager::DataManager, model::DataObject},
};
use std::{io, process};

impl DataManager {
    pub fn remove_rule_from_json(
        &self,
        data: &mut DataObject,
        keyword: &str,
    ) -> Result<(), io::Error> {
        if let Some(target_path) = data.targets.get(keyword) {
            if menu::get_yn_input(message_format(
                MessageKind::DeleteRule,
                MessageArgs {
                    primary_keyword: keyword.to_string(),
                    primary_path: target_path.to_string(),
                    ..Default::default()
                },
            )) {
                data.targets.remove(keyword);
            } else {
                process::exit(1)
            }
        }
        Ok(())
    }
}

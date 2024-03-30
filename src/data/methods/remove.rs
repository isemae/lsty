use crossterm::style::Stylize;

use crate::{
    cli::menu,
    data::{data_manager::DataManager, model::DataObject},
};
use std::{io, process};

impl DataManager {
    pub fn remove_rule_from_json(
        &self,
        data: &mut DataObject,
        keyword: &str,
    ) -> Result<(), io::Error> {
        if data.targets.get(keyword).is_some() {
            let target_path = data.targets.get(keyword);
            if menu::get_yn_input(format!(
                "[y/N] delete rule for keyword '{}', target path '\x1b[4m{}\x1b[0m\x1b[0m'?",
                keyword,
                target_path.unwrap_or(&"".to_string())
            )) {
                data.targets.remove(keyword);
            } else {
                process::exit(1)
            }
            Ok(())
        } else {
            Err(io::Error::new(
                        io::ErrorKind::NotFound,
                        format!(
                            "{} no such rule for the keyword rule in the current path. \nkeywords available for current path:\n{}",
                            "[?]".yellow(),
                                data.targets.keys().cloned().collect::<Vec<_>>().join("\n").cyan()
                        ),
                    ))
        }
    }
}

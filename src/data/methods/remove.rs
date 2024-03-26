use crossterm::style::{Color, Stylize};

use crate::{
    cli::menu,
    data::{
        data_manager::DataManager,
        model::{DataModel, DataObject},
    },
};
use std::io;

impl DataManager {
    pub fn remove_rule_from_json(
        &self,
        mut data: DataModel,
        source_path: &str,
        keyword: &str,
    ) -> Result<(), io::Error> {
        // source validation
        if let Some(obj) = data
            .data
            .iter_mut()
            .find(|o| o.source == source_path.to_string())
        {
            if obj.targets.get(keyword).is_some() {
                let target_path = obj.targets.get(keyword);
                println!(
                    "would you like to delete data for keyword '{}', target path '{}'? (y/N)",
                    keyword,
                    target_path.unwrap_or(&"".to_string())
                );
                obj.targets.remove(keyword);
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!(
                        "{} no such rule for the keyword rule in the current path. \nkeywords available for current path:\n {}",
                        "[?]".yellow(),
                        if let Some(obj) = data.data.iter().find(|o| o.source == source_path) {
                            obj.targets.keys().cloned().collect::<Vec<_>>().join(", ").cyan()
                        } else {
                            "".to_string().cyan()
                        }
                    ),
                ));
            }

            if menu::get_yn_input() {
                match self.save_json_data(&data) {
                    Ok(()) => {}
                    Err(e) => {
                        eprintln!("{}", e);
                    }
                }
            }
        } else {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "[!] no rule for the current path in the data",
            ));
        }
        Ok(())
    }
}

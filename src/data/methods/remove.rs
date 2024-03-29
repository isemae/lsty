use camino::Utf8PathBuf;
use crossterm::style::{Color, Stylize};

use crate::data::{
    data_manager::DataManager,
    model::{DataModel, DataObject},
};
use std::io;

impl DataManager {
    pub fn remove_rule_from_json(
        &self,
        data: &mut DataObject,
        keyword: &str,
    ) -> Result<(), io::Error> {
        if data.targets.get(keyword).is_some() {
            let target_path = data.targets.get(keyword);
            println!(
                "would you like to delete data for keyword '{}', target path '{}'? (y/N)",
                keyword,
                target_path.unwrap_or(&"".to_string())
            );
            data.targets.remove(keyword);
            Ok(())
        } else {
            return Err(io::Error::new(
                        io::ErrorKind::NotFound,
                        format!(
                            "{} no such rule for the keyword rule in the current path. \nkeywords available for current path:\n {}",
                            "[?]".yellow(),
                                data.targets.keys().cloned().collect::<Vec<_>>().join(", ").cyan()
                        ),
                    ));
        }
    }
}

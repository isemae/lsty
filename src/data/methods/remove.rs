use crate::{
    cli::{
        menu,
        status_symbols::{status_symbol, Status::*},
    },
    data::{data_manager::DataManager, model::DataObject},
};
use crossterm::style::Stylize;
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
                "{0} delete rule for keyword '{1}', target path '\x1b[4m{2}\x1b[0m\x1b[0m'?",
                status_symbol(&YN),
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
                            status_symbol(&Safe),
                                data.targets.keys().cloned().collect::<Vec<_>>().join("\n").cyan()
                        ),
                    ))
        }
    }
}

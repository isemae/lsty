use crate::{
    cli::{
        menu,
        status_symbols::{status_symbol, Status::*},
    },
    data::{data_manager::DataManager, model::DataObject},
};
use camino::Utf8PathBuf;
use std::process;

impl DataManager {
    pub fn edit_rule(&self, obj: &mut DataObject, keyword: String, replacement: String) {
        let targets = obj.targets.clone();
        if let Some(target_path) = targets.get(&keyword) {
            let is_replacement_dir = Utf8PathBuf::from(&replacement).is_dir();
            let confirmation = if is_replacement_dir {
                format!(
                "{0} change target path '\x1b[4m{1}\x1b[0m\x1b[0m' -> '\x1b[4m{2}\x1b[0m\x1b[0m' for keyword '{3}'?",
                status_symbol(&YN),target_path, replacement, keyword)
            } else if !replacement.contains('\\')
                && !replacement.contains('/')
                && !replacement.contains('~')
            {
                format!(
                    "{0} change keyword '{1}' -> '{2}'?",
                    status_symbol(&YN),
                    keyword,
                    replacement
                )
            } else {
                eprintln!("invalid path.");
                return;
            };

            if menu::get_yn_input(confirmation) {
                obj.targets.remove(&keyword);
                if is_replacement_dir {
                    obj.targets.insert(keyword, replacement);
                } else {
                    obj.targets.insert(replacement, target_path.to_string());
                }
                println!("rule updated.");
            } else {
                process::exit(1);
            }
        }
        println!("rule updated.")
    }
}

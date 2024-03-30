use crate::{
    cli::menu,
    data::{data_manager::DataManager, model::DataObject},
};
use camino::Utf8PathBuf;
use std::process;

impl DataManager {
    pub fn edit_rule(&self, obj: &mut DataObject, keyword: String, replacement: String) {
        let targets = obj.targets.clone();
        if let Some(target_path) = targets.get(&keyword) {
            let is_replacement_dir = Utf8PathBuf::from(&replacement).is_dir();
            if is_replacement_dir {
                println!(
                    "[y/N] change target path '\x1b[4m{}\x1b[0m\x1b[0m' -> '\x1b[4m{}\x1b[0m\x1b[0m' for the keyword '{}' ?",
                    target_path, replacement, keyword
                );
                if menu::get_yn_input() {
                    obj.targets.remove(&keyword);
                    obj.targets.insert(keyword, replacement);
                } else {
                    process::exit(1)
                }
            } else if !replacement.contains('\\')
                && !replacement.contains('/')
                && !replacement.contains('~')
            {
                println!("[y/N] change keyword '{}' -> '{}'?", keyword, replacement);
                if menu::get_yn_input() {
                    obj.targets.remove(&keyword);
                    obj.targets.insert(replacement, target_path.to_string());
                } else {
                    process::exit(1)
                }
            } else {
                eprintln!("invalid path.");
            }
        }
    }
}

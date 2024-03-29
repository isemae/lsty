use crate::{
    cli::menu,
    data::{data_manager::DataManager, model::DataModel},
};
use camino::Utf8PathBuf;
use std::{env, process};

impl DataManager {
    pub fn edit_rule(&self, data: &mut DataModel, keyword: String, replacement: String) {
        let source_path = Utf8PathBuf::from_path_buf(env::current_dir().unwrap_or_default())
            .expect("valid Unicode path succeeded");
        if let Some(obj) = data.data.iter_mut().find(|o| o.source == source_path) {
            let targets = obj.targets.clone();
            let target_path = targets
                .get(&keyword)
                .expect("found no target path for the key.");
            println!("[y/N] change keyword '{}' -> '{}'?", keyword, replacement);

            match menu::get_yn_input() {
                true => {
                    obj.targets.remove(&keyword);
                    obj.targets.insert(replacement, target_path.to_string());
                }
                false => process::exit(1),
            }
        } else {
            eprintln!("no such rule for the keyword");
        }
        self.save_json_data(data).expect("");
    }
}

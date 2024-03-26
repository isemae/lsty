use crate::{
    cli::menu,
    data::{data_manager::DataManager, model::DataModel},
};
use camino::Utf8PathBuf;
use std::{env, io};

impl DataManager {
    pub fn import_rule(
        &self,
        data: &mut DataModel,
        alias: String,
        mut source_path: String,
    ) -> Result<(), io::Error> {
        let current_dir = Utf8PathBuf::from_path_buf(env::current_dir().unwrap_or_default())
            .expect("valid Unicode path succeeded");

        let error_message = match (alias.is_empty(), alias.contains('/'), alias.contains('\\')) {
            (true, _, _) => "NOT FOUND: no rule for the path found.",
            (false, true, _) | (false, _, true) => {
                "invalid alias: alias should not contain '/' or '\\'."
            }
            (false, false, false) => "NOT FOUND: no rule for the alias found.",
        };

        if let Some(data_map) = data
            .data
            .iter()
            .find(|obj| obj.source == source_path || obj.alias == alias && !alias.is_empty())
        {
            if source_path.is_empty() {
                source_path = data_map.source.clone();
            }

            let targets = data_map.targets.clone();
            let current_obj = data.data.iter_mut().find(|o| o.source == current_dir);

            println!("do you want to import rules: ");
            for (k, v) in &targets {
                println!("  keyword: {}, target path: {}", k, v);
            }
            println!("from {}?", source_path);

            match menu::get_yn_input() {
                true => {
                    current_obj.unwrap().targets.extend(targets);
                    self.save_json_data(data)?;
                    println!("rules imported.")
                }
                false => {}
            }
        } else {
            return Err(io::Error::new(io::ErrorKind::NotFound, error_message));
        }
        Ok(())
    }
}

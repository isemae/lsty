use crate::data::{
    data_manager::DataManager,
    model::{DataModel, DataObject},
};
use camino::Utf8PathBuf;
use std::{collections::HashMap, env, error::Error, io};

impl DataManager {
    pub fn import_rule(
        &self,
        data: &mut DataModel,
        alias: String,
        source_path: Utf8PathBuf,
    ) -> Result<(), io::Error> {
        let current_dir = Utf8PathBuf::from_path_buf(env::current_dir().unwrap_or_default())
            .expect("valid Unicode path succeeded");

        let targets = {
            let mut targets_map = None;

            let error_message = if alias.is_empty() {
                "no rules for the path found."
            } else if alias.contains("/") || alias.contains("\\") {
                "invalid path."
            } else {
                "no rules for the alias found."
            };

            for obj in &mut data.data {
                if alias.is_empty() && obj.source == source_path {
                    targets_map = Some(&obj.targets);
                    break;
                } else if !alias.is_empty() && obj.alias == alias {
                    targets_map = Some(&obj.targets);
                    break;
                }
            }
            targets_map.ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, error_message))?
        };

        let mut import_targets: HashMap<String, String> = HashMap::new();
        for (k, v) in targets.iter() {
            import_targets.insert(k.clone(), v.clone());
        }

        let current_obj = data
            .data
            .iter_mut()
            .find(|o| o.source == current_dir.to_string());

        current_obj.unwrap().targets.extend(import_targets);

        self.save_json_data(&data)?;

        Ok(())
    }
}

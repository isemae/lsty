use crate::{
    cli::{
        menu,
        messages::{message_format, MessageArgs, MessageKind},
    },
    data::{
        data_manager::DataManager,
        model::{DataModel, DataObject},
    },
};
use camino::Utf8PathBuf;
use std::{collections::HashMap, env, io};

impl DataManager {
    pub fn import_rule(
        &self,
        data: &mut DataModel,
        alias: String,
        mut import_path: String,
    ) -> Result<(), io::Error> {
        let current_dir = Utf8PathBuf::from_path_buf(env::current_dir().unwrap_or_default())
            .expect("valid Unicode path succeeded");

        let error_message = match (alias.is_empty(), alias.contains('/'), alias.contains('\\')) {
            (true, _, _) => message_format(
                MessageKind::NotFoundRuleForPath,
                MessageArgs {
                    ..Default::default()
                },
            ),
            (false, true, _) | (false, _, true) => message_format(
                MessageKind::InvalidAlias,
                MessageArgs {
                    ..Default::default()
                },
            ),
            (false, false, false) => message_format(
                MessageKind::NotFoundAlias,
                MessageArgs {
                    ..Default::default()
                },
            ),
        };

        if let Some(data_map) = data
            .data
            .iter()
            .find(|obj| obj.source == import_path || obj.alias == alias && !alias.is_empty())
        {
            if import_path.is_empty() {
                import_path = data_map.source.clone();
            }

            let targets = data_map.targets.clone();
            let current_obj = match data.object_by_source_mut(current_dir.clone()) {
                Ok(obj) => obj,
                Err(_) => {
                    data.data.push(DataObject {
                        alias: "".to_string(),
                        source: current_dir.clone().to_string(),
                        targets: HashMap::new(),
                    });
                    data.data.last_mut().unwrap()
                }
            };

            println!("do you want to import rules: ");
            for (k, v) in &targets {
                println!("  keyword: {}, target path: \x1b[4m{}\x1b[0m\x1b[0m", k, v);
            }
            println!();
            match menu::get_yn_input(message_format(
                MessageKind::FromPath,
                MessageArgs {
                    primary_path: import_path,
                    ..Default::default()
                },
            )) {
                true => {
                    current_obj.targets.extend(targets);
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

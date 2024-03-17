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
            .find(|obj| obj.sources.contains(&source_path.to_string()))
        {
            if obj.targets.contains_key(keyword) {
                obj.targets.remove(keyword);
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "no such keyword rule for the current path",
                ));
            }

            if menu::get_yn_input() {
                match self.save_json_data(&data) {
                    Ok(()) => {}
                    Err(e) => {
                        eprintln!("Error: {}", e)
                    }
                }
            }
        } else {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "no rule for the current path in the data",
            ));
        }
        Ok(())
    }
}

use crate::data::{
    data_manager::DataManager,
    model::{DataModel, DataObject},
};
use std::env;
use std::path::{Path, PathBuf};

impl DataManager {
    pub fn copy_rule(&self, mut data: DataModel, target_path: String) {
        let current_dir = env::current_dir().unwrap_or_else(|_| PathBuf::from(""));
        if let Some(data_obj) = data.data.iter_mut().find(|obj| {
            obj.sources
                .contains(&current_dir.to_string_lossy().to_string())
        }) {
            // println!("export? (y/n)");
            println!("{}", target_path);
            if !target_path.is_empty() {
                println!("import");
                println!("{:?}", data_obj.sources);
                data_obj.sources.push(target_path);
                println!("{:?}", data_obj.sources);
                // self.import_rule(data_obj.clone(), target_path);
                self.save_json_data(&data).expect("")
            } else {
                println!("export");
                println!("setting alias for later import? (y/n)");
                self.export_rule(data_obj)
            }
        }
    }

    // pub fn import_rule(&self, mut data: DataObject, path: String) -> {

    // }

    pub fn export_rule(&self, mut data: &DataObject) {
        // data.alias = alias
    }
}

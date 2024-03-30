use crate::data::{data_manager::DataManager, model::DataObject};
use std::process;

impl DataManager {
    pub fn set_alias(&self, data: &mut DataObject, alias: String) {
        if alias.contains('/') || alias.contains('\\') {
            eprintln!("[!] invalid alias: alias should not contain '/' or '\\'.");
            process::exit(1);
        } else {
            data.alias = alias;
        }
    }
}

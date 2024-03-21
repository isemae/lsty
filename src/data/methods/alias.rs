use crate::data::{data_manager::DataManager, model::DataObject};

impl DataManager {
    pub fn set_alias(&self, data: &mut DataObject, alias: String) {
        data.alias = alias
    }
}

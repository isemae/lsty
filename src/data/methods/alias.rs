use crate::{
    cli::status_symbols::{status_symbol, Status::*},
    data::{data_manager::DataManager, model::DataObject},
};
use std::process;

impl DataManager {
    pub fn set_alias(&self, data: &mut DataObject, alias: String) {
        if alias.contains('/') || alias.contains('\\') {
            eprintln!(
                "{0} invalid alias: alias should not contain '/' or '\\'.",
                status_symbol(&Error)
            );
            process::exit(1);
        } else {
            data.alias = alias;
        }
    }
}

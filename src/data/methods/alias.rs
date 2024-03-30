use crate::{
    cli::{
        menu,
        status_symbols::{status_symbol, Status::*},
    },
    data::{data_manager::DataManager, model::DataObject},
};
use std::io;

impl DataManager {
    pub fn set_alias(&self, data: &mut DataObject, alias: String) -> Result<(), io::Error> {
        if alias.contains('/') || alias.contains('\\') {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "{0} invalid alias: alias should not contain '/' or '\\'.",
                    status_symbol(&Error)
                ),
            ))
        } else {
            if menu::get_yn_input(format!(
                "{} update alias '{}' -> '{}'?",
                status_symbol(&YN),
                data.alias,
                alias
            )) {
                data.alias = alias;
            }
            Ok(())
        }
    }
}

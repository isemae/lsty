use crate::{
    cli::{cli_format::*, menu},
    data::{data_manager::DataManager, model::DataObject},
};
use std::io;

impl DataManager {
    pub fn set_alias(&self, data: &mut DataObject, alias: String) -> Result<(), io::Error> {
        if alias.contains('/') || alias.contains('\\') {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                error_format(ErrorKind::InvalidAlias),
            ))
        } else {
            if menu::get_yn_input(msg_format(MsgKind::UpdatingAlias(MsgArgs {
                primary_keyword: data.alias.clone(),
                secondary_keyword: alias.clone(),
                ..Default::default()
            }))) {
                data.alias = alias;
            }
            Ok(())
        }
    }
}

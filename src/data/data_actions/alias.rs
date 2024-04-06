use crate::{
    cli::{
        menu,
        messages::{message_format, MessageArgs, MessageKind},
    },
    data::{data_manager::DataManager, model::DataObject},
};
use std::io;

impl DataManager {
    pub fn set_alias(&self, data: &mut DataObject, alias: String) -> Result<(), io::Error> {
        if alias.contains('/') || alias.contains('\\') {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                message_format(
                    MessageKind::InvalidAlias,
                    MessageArgs {
                        ..Default::default()
                    },
                ),
            ))
        } else {
            if menu::get_yn_input(message_format(
                crate::cli::messages::MessageKind::UpdatingAlias,
                MessageArgs {
                    primary_keyword: data.alias.clone(),
                    secondary_keyword: alias.clone(),
                    ..Default::default()
                },
            )) {
                data.alias = alias;
            }
            Ok(())
        }
    }
}

use crate::{
    cli::menu,
    data::{data_manager::DataManager, model::DataObject},
};
use std::process;

impl DataManager {
    pub fn set_alias(&self, data: &mut DataObject, alias: String) {
        if alias.contains('/') || alias.contains('\\') {
            eprintln!("[!] invalid alias: alias should not contain '/' or '\\'.");
            process::exit(1);
        } else {
            println!("[y/N] update alias '{}' -> '{}'?", data.alias, alias);
            match menu::get_yn_input() {
                true => {
                    println!("updated alias: {} -> {}", data.alias, alias);
                    data.alias = alias;
                }
                false => (),
            }
        }
    }
}

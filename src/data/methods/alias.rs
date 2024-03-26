use crate::{
    cli::menu,
    data::{data_manager::DataManager, model::DataObject},
};

impl DataManager {
    pub fn set_alias(&self, data: &mut DataObject, alias: String) {
        println!(
            "updating alias of the current directory rules from \n'{}' to '{}' (y/N)",
            data.alias, alias
        );
        match menu::get_yn_input() {
            true => {
                println!("updated alias: {} -> {}", data.alias, alias);
                data.alias = alias;
            }
            false => return,
        }
    }
}

use camino::Utf8PathBuf;

use super::arguments::Config;
use crate::data::data_manager::DataManager;
use std::io::Result;

pub fn process_command(config: &Config) -> Result<()> {
    let mut data_manager = DataManager;
    let current_path = std::env::current_dir().unwrap_or_default();
    let default_path = Utf8PathBuf::from_path_buf(current_path)
        .unwrap_or_default()
        .to_string();
    config
        .command
        .process(&mut data_manager, default_path)
        .expect("error processing commands");

    Ok(())
}

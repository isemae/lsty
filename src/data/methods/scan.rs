use crate::data::{data_manager::DataManager, model::DataModel};
use std::{collections::HashMap, env::current_dir, fs, path::PathBuf};

impl DataManager {
    pub fn scan_and_validate_path(&self, data: DataModel) -> Option<HashMap<String, String>> {
        let current_dir = current_dir().unwrap_or_else(|_| PathBuf::from(""));

        if let Some(pair) = data.pairs.get(&current_dir.to_string_lossy().to_string()) {
            let mut valid_pair = HashMap::new();

            for map in pair {
                if !PathBuf::from(map.1).exists() {
                    eprintln!(
                            "\x1b[0;33m⚠ target path '{}' doesn't exist. Creating the directory...\x1b[0m",
                            map.1
                        );
                    fs::create_dir_all(&map.1)
                        .expect("Error: failed to create target directory on disk.");
                    valid_pair.insert(map.0.clone(), map.1.clone());
                } else {
                    valid_pair.insert(map.0.clone(), map.1.clone());
                }
            }
            return Some(valid_pair);
        }
        None
    }
}
// if !Path::new(source_path).exists() {
//     eprintln!(
//         "\x1b[0;31m ✘ Source path {} is not a valid path.\x1b[0m",
//         source_path.yellow()
//     );
//     return Some(Err(io::Error::new(
//         io::ErrorKind::NotFound,
//         "no such directory exists.",
//     )));
// }
// if !Path::new(&target).is_dir() {
//     eprintln!(
//         "\x1b[0;33m⚠ target path '{}' doesn't exist. Creating the directory...\x1b[0m",
//         target
//     );
//     fs::create_dir_all(&target)
//         .expect("Error: failed to create target directory on disk.");
// } else {
//     println!("hehe")
// }
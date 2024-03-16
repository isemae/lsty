use crate::{
    cli::menu,
    data::{data_manager::DataManager, model::DataModel},
};
use regex::Regex;

use std::fs::DirEntry;
use std::{collections::HashMap, io};
use std::{
    env::current_dir,
    fs,
    path::{Path, PathBuf},
};
use unicode_normalization::UnicodeNormalization;

impl DataManager {
    pub fn normalize_entry(&self, entry: &DirEntry) -> String {
        let entry_path = entry.path();
        let entry_name = match entry_path.file_name() {
            Some(name) => name,
            None => return String::new(),
        };
        entry_name.to_string_lossy().nfc().collect::<String>()
    }

    fn generate_new_entries(
        &self,
        map: HashMap<String, String>,
    ) -> Result<HashMap<String, Vec<String>>, io::Error> {
        let current_dir = current_dir()?;
        let entries = fs::read_dir(&current_dir)?;
        let mut entry_map = HashMap::new();

        for entry in entries {
            let entry = entry?;

            for (kw, target) in &map {
                let re = Regex::new(kw).unwrap();
                let normalized = self.normalize_entry(&entry);
                if re.is_match(&normalized) {
                    entry_map
                        .entry(target.to_string())
                        .or_insert_with(Vec::new)
                        .push(normalized)
                }
            }
        }
        Ok(entry_map)
    }

    pub fn move_dirs(
        &self,
        map: HashMap<String, String>,
        keyword: String,
    ) -> Result<(), io::Error> {
        let mut moved_count = 0;
        let current_dir = current_dir()?;
        let entries_map = self.generate_new_entries(map.clone())?;
        println!("");
        println!("SOURCE: {}", current_dir.display());
        for (target, vec) in entries_map {
            println!("\r└→ \x1b[4m{}\x1b[0m\x1b[0m", target);
            for entry in vec.clone() {
                let new_entry = format!("{}/{}", target, entry);

                let mut entry_symbol = "";
                match fs::metadata(&entry) {
                    Ok(metadata) => {
                        if metadata.is_dir() {
                            entry_symbol = "📁"
                        } else {
                            entry_symbol = "📄"
                        }
                    }
                    Err(_) => {}
                }

                if !new_entry.is_empty() {
                    match Path::new(&new_entry).exists() {
                        true => {
                            println!(
                                "  \x1b[0;33m[{}]\x1b[0m {} {} already exists in the target directory.",
                                "!", entry_symbol, entry
                            );
                            continue;
                        }
                        false => {
                            self.scan_and_validate_path(map.clone()).unwrap();
                            println!("  \x1b[0;32m{}\x1b[0m {} {}", "[✓]", entry_symbol, entry,);
                            self.move_entry(entry, new_entry);
                            moved_count += 1;
                        }
                    }
                }
            }
            println!("");
        }
        if moved_count == 0 {
            println!("{} No items to move in the source path.", "[✓]");
        }
        Ok(())
    }

    fn move_entry(&self, entry_path: String, new_entry: String) {
        match PathBuf::from(entry_path.clone()).is_dir() {
            true => {
                fs::create_dir_all(&new_entry).expect("");
                self.copy_dir(
                    &PathBuf::from(entry_path.clone()),
                    &PathBuf::from(&new_entry),
                )
                .expect("");
                fs::remove_dir_all(&entry_path).expect("");
            }
            false => {
                fs::copy(&entry_path, new_entry).expect("");
                fs::remove_file(&entry_path).expect("");
            }
        }
    }

    fn copy_dir(&self, src: &PathBuf, trg: &PathBuf) -> std::io::Result<()> {
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let file_path = entry.path();
            let file_name = file_path.file_name().unwrap().to_str().unwrap();
            let new_path = trg.join(file_name);
            if file_path.is_dir() {
                fs::create_dir_all(&new_path)?;
                self.copy_dir(&file_path, &new_path)?;
            } else {
                fs::copy(&file_path, &new_path).expect("");
            }
        }
        Ok(())
    }
}

// fn validates_pair(source_path: &str, target_path: &str) -> Option<Result<(), io::Error>> {
//
// }

// fn validates_path(path: &str) -> Result<(), io::Error> {
//     if !Path::new(path).exists() {
//         eprintln!(
//             "\x1b[0;31m ✘ path {} is not a valid path.\x1b[0m",
//             path.yellow()
//         );
//         return Err(io::Error::new(
//             io::ErrorKind::NotFound,
//             "no such directory exists.",
//         ));
//     }
//     Ok(())
// }
// 데이터모델에 등록된 경로 유효성 확인

use crate::data::{data_manager::DataManager, model::DataModel};
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

    fn generate_new_entry(
        &self,
        entry_name: String,
        map: HashMap<String, String>,
    ) -> Option<String> {
        let mut new_entry = String::new();
        for (kw, target) in map {
            let re = Regex::new(&kw).unwrap();
            if re.is_match(&entry_name) {
                // new_map.insert(entry_name, target.clone());
                return Some(format!("{}/{}", target, entry_name));
            }
        }
        // println!("{}", new_entry);
        None
    }

    pub fn move_dirs(
        &self,
        map: HashMap<String, String>,
        keyword: String,
    ) -> Result<(), io::Error> {
        let mut moved_count = 0;
        let current_dir = current_dir()?;
        let entries = fs::read_dir(&current_dir)?;

        println!("");
        println!("SOURCE: {:?}", current_dir);

        for e in entries {
            let entry = e?;
            let entry_path = entry.path();
            let normalized = self.normalize_entry(&entry);

            let new_entry = self
                .generate_new_entry(normalized.clone(), map.clone())
                .unwrap_or_default();
            let mut entry_symbol = "";
            match fs::metadata(&entry_path) {
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
                            "[!] \x1b[0;31mEXIST:\x1b[0m {} already exists in the target directory.",
                            new_entry
                        );
                        continue;
                    }
                    false => {
                        println!(
                            "[\x1b[0;32mM\x1b[0m] {} {} \n\
                            \r └→ \x1b[4m{}\x1b[0m\x1b[0m",
                            entry_symbol, normalized, new_entry
                        );
                        self.move_entry(entry_path, new_entry);
                        moved_count += 1;
                    }
                }
            }
        }

        //// 특정키워드만 / 일괄
        // if keyword.is_empty() {
        //     keywords = map.keys().cloned().collect();
        // } else {
        //     keywords.push(keyword.to_owned());
        // }

        // if validates_path(&new_entry) {
        //     println!(
        //         "│ \x1b[0;31mEXIST:\x1b[0m {} already exists in the target directory.",
        //         entry_name_normalized
        //     );
        //     continue;
        // }

        if moved_count == 0 {
            println!("{} source path is clean. No items to move", "[✓]");
        }
        Ok(())
    }

    fn move_entry(&self, entry_path: PathBuf, new_entry: String) {
        match entry_path.is_dir() {
            true => {
                fs::create_dir_all(&new_entry).expect("");
                self.copy_dir(&entry_path, &PathBuf::from(&new_entry))
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

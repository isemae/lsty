use crate::data::{data_manager::DataManager, model::DataModel};
use regex::Regex;

use std::{collections::HashMap, io};
use std::{
    env::current_dir,
    fs,
    path::{Path, PathBuf},
};
use unicode_normalization::UnicodeNormalization;

impl DataManager {
    pub fn move_dirs(&self, map: HashMap<String, String>, keyword: &str) -> Result<(), io::Error> {
        let mut moved_count = 0;
        // println!("map: {:?}", map);
        let current_dir = current_dir()?;
        let target_path = map.get(keyword);
        let mut keywords = Vec::new();

        if keyword.is_empty() {
            keywords = map.keys().cloned().collect();
        } else {
            keywords.push(keyword.to_owned());
        }

        println!("keywords: {:?}", keywords);

        println!("");
        println!("SOURCE: {:?}", current_dir);

        // generates regex pattern
        let pattern = keywords.join("|");
        let re = Regex::new(&pattern).unwrap();

        // 소스경로 내 모든 엔트리
        let entries = fs::read_dir(current_dir)?;
        let filtered_entries: Vec<_> = entries
            .filter_map(|e| {
                let entry = e.ok()?;
                let path = entry.path();
                let entry_name_normalized = match path.file_name() {
                    Some(name) => name.to_string_lossy().nfc().collect::<String>(),
                    None => return None,
                };

                if re.is_match(&entry_name_normalized) {
                    Some(path)
                } else {
                    None
                }
            })
            .collect();

        for entry in filtered_entries {
            println!("found {}", entry.display())
        }
        //     // 정규화된 키워드를 파일명으로 포함하는 엔트리
        // let new_entry = format!("{:?}/{:?}", target_path, filtered_entry);

        //     // 엔트리가 이미 존재하는 파일인지
        //     if validates_path(&new_entry) {
        //         println!(
        //             "│ \x1b[0;31mEXIST:\x1b[0m {} already exists in the target directory.",
        //             entry_name_normalized
        //         );
        //         continue;
        //     }

        //     println!(
        //         "│\x1b[0;32m MOVE:\x1b[0m  \x1b[4m{}\x1b[0m\x1b[0m",
        //         entry_name_normalized
        //     );

        //     self.move_entry(entry.path(), new_entry);
        //     moved_count += 1;
        // }

        println!("└→ TARGET: {:?}", target_path);
        if moved_count == 0 {
            println!("No items to move");
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
//     if !Path::new(source_path).exists() {
//         eprintln!(
//             "\x1b[0;31m ✘ Source path {} is not a valid path.\x1b[0m",
//             source_path.yellow()
//         );
//         return Some(Err(io::Error::new(
//             io::ErrorKind::NotFound,
//             "no such directory exists.",
//         )));
//     }
//     if !Path::new(target_path).exists() {
//         eprintln!(
//             "\x1b[0;33m⚠ target path '{}' doesn't exist. Creating the directory...\x1b[0m",
//             target_path.yellow()
//         );
//         fs::create_dir_all(&target_path)
//             .expect("Error: failed to create target directory on disk.");
//     }
//     None
// }
// match Path::new(&new_entry).exists() {
//     true => {
//         println!(
//             "│ \x1b[0;31mEXIST:\x1b[0m {} already exists in the target directory.",
//             entry_name_normalized
//         );
//         continue;
//     }
//     false => {
//         println!(
//             "│\x1b[0;32m MOVE:\x1b[0m  \x1b[4m{}\x1b[0m\x1b[0m",
//             entry_name_normalized
//         );

//         self.move_entry(entry_path, new_entry);
//         moved_count += 1;
//     }
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

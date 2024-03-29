use crate::{cli::menu, data::data_manager::DataManager};
use regex::Regex;
use std::{
    collections::{hash_map::Entry, HashMap},
    env::current_dir,
    fs::{self, DirEntry},
    io,
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
        source: &str,
        target_map: &HashMap<String, String>,
        keyword: &str,
    ) -> Result<HashMap<String, Vec<String>>, io::Error> {
        let entries = fs::read_dir(source)?;
        let mut entry_map: HashMap<String, Vec<String>> = HashMap::new();
        let mut patterns = HashMap::new();

        for entry in entries {
            let entry = entry?;
            let entry_name = self.normalize_entry(&entry);

            let keywords = if !keyword.is_empty() {
                vec![keyword.to_string()]
            } else {
                target_map.keys().cloned().collect()
            };

            for kw in &keywords {
                let lowercase_kw = kw.to_lowercase();
                let pattern = match patterns.entry(kw.to_lowercase()) {
                    Entry::Occupied(entry) => entry.into_mut(),
                    Entry::Vacant(entry) => entry.insert(Regex::new(&lowercase_kw).unwrap()),
                };
                if let Some(target) = target_map.get(&lowercase_kw) {
                    let lowercase_entry_name = entry_name.to_lowercase();
                    if pattern.is_match(&lowercase_entry_name) {
                        entry_map
                            .entry(target.to_string())
                            .or_default()
                            .push(entry_name.clone())
                    }
                }
            }
        }
        Ok(entry_map)
    }

    pub fn move_dirs(
        &self,
        target_map: &HashMap<String, String>,
        keyword: &str,
    ) -> Result<(), io::Error> {
        let mut moved_count = 0;
        let current_dir = current_dir()?;
        let current_dir_str = current_dir.to_str().expect("");

        let entries_map = self.generate_new_entries(current_dir_str, target_map, keyword)?;
        println!("\nSOURCE: {}", current_dir_str);
        for (target, vec) in entries_map {
            println!("\r└→ \x1b[4m{}\x1b[0m\x1b[0m ", target);
            for entry in vec.clone() {
                let new_entry = format!("{}/{}", target, entry);
                let entry_symbol = menu::entry_symbol(&entry);

                if !new_entry.is_empty() {
                    match Path::new(&new_entry).exists() {
                        true => {
                            println!(
                                "  \x1b[0;33m[!]\x1b[0m {} {} already exists in the target directory.",
                                entry_symbol, entry
                            );
                            continue;
                        }
                        false => {
                            self.scan_and_validate_path(target_map).unwrap();
                            self.move_entry(entry.clone(), new_entry);
                            println!("  \x1b[0;32m[✓]\x1b[0m {} {}", entry_symbol, entry);
                            moved_count += 1;
                        }
                    }
                }
            }
        }
        if moved_count == 0 {
            println!("[✓] No items to move in the source path.");
        } else {
            println!("\nDone.")
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

use crate::{
    cli::{
        menu,
        status_symbols::{status_symbol, Status::*},
    },
    data::{data_manager::DataManager, model::DataObject},
};
use std::{
    collections::HashMap,
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
    fn validate_pair(&self, targets: &HashMap<String, String>) -> Option<HashMap<String, String>> {
        let mut valid_pair = HashMap::new();

        for map in targets.iter() {
            if !PathBuf::from(map.1).exists() {
                eprintln!(
                            " {} \x1b[0;33mtarget path '{}' doesn't exist. Creating the directory...\x1b[0m",
                            status_symbol(&Caution), map.1
                        );
                fs::create_dir_all(map.1)
                    .expect("Error: failed to create target directory on disk.");
                valid_pair.insert(map.0.clone(), map.1.clone());
            } else {
                valid_pair.insert(map.0.clone(), map.1.clone());
            }
        }
        Some(valid_pair)
    }

    pub fn rename_entries(&self, data: &DataObject, keyword: &str) -> Result<(), io::Error> {
        let mut moved_count = 0;
        let current_dir = current_dir()?;
        let current_dir_str = current_dir.to_str().expect("");

        let entries_map = self.scan_current_path(data, keyword)?;
        println!("\nSOURCE: \x1b[4m{}\x1b[0m\x1b[0m", current_dir_str);
        for (target, vec) in entries_map {
            println!("\r└→ \x1b[4m{}\x1b[0m\x1b[0m ", target);
            for entry in vec.clone() {
                let new_entry = format!("{}/{}", target, entry);
                let entry_symbol = menu::entry_symbol(&entry);

                if !new_entry.is_empty() {
                    match Path::new(&new_entry).exists() {
                        true => {
                            println!(
                                "  {0} {1} {2} already exists in the target directory.",
                                status_symbol(&Caution),
                                entry_symbol,
                                entry
                            );
                            continue;
                        }
                        false => {
                            self.validate_pair(&data.targets).unwrap();
                            self.move_entry(&entry, new_entry);
                            println!("  {0} {1} {2}", status_symbol(&Safe), entry_symbol, entry);
                            moved_count += 1;
                        }
                    }
                }
            }
        }
        if moved_count == 0 {
            println!(
                "{} No items to move in the source path.",
                status_symbol(&Safe)
            );
        } else {
            println!("\nDone.")
        }

        Ok(())
    }

    pub fn move_entry(&self, entry_path: &String, new_entry: String) {
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

    #[allow(clippy::only_used_in_recursion)]
    fn copy_dir(&self, src: &PathBuf, trg: &Path) -> std::io::Result<()> {
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let file_path = entry.path();
            let file_name = file_path
                .file_name()
                .and_then(|name_osstr| name_osstr.to_str())
                .ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::NotFound,
                        format!("{} no rule for the current path.", status_symbol(&NotFound)),
                    )
                })?;

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

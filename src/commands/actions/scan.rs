use crate::data::{data_manager::DataManager, model::DataObject};
use regex::Regex;
use std::{
    collections::{hash_map::Entry, HashMap},
    fs, io,
};

impl DataManager {
    pub fn scan_current_path(
        &self,
        data: &DataObject,
        keyword: &str,
    ) -> Result<HashMap<String, Vec<String>>, io::Error> {
        let entries = fs::read_dir(&data.source)?;
        let mut entry_map: HashMap<String, Vec<String>> = HashMap::new();
        let mut patterns = HashMap::new();

        for entry in entries {
            let entry = entry?;
            let entry_name = self.normalize_entry(&entry);

            let keywords = if !keyword.is_empty() {
                vec![keyword.to_string()]
            } else {
                data.targets.keys().cloned().collect()
            };

            for kw in &keywords {
                let lowercase_kw = kw.to_lowercase();
                let pattern = match patterns.entry(kw.to_lowercase()) {
                    Entry::Occupied(entry) => entry.into_mut(),
                    Entry::Vacant(entry) => entry.insert(Regex::new(&lowercase_kw).unwrap()),
                };
                if let Some(target) = data.targets.get(&lowercase_kw) {
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
}

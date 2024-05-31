use crate::{
    cli::{
        cli_format::{
            error_format, msg_format, ConfirmationResult, ErrorKind, MsgArgs, MsgKind, MsgKind::*,
        },
        menu,
        status_symbols::{status_symbol, Status::*},
    },
    data::{
        check_input::{check_input, InputCase},
        data_manager::DataManager,
        model::{DataModel, DataObject},
    },
};
use std::{
    collections::{hash_map::Entry, HashMap},
    env::{self, current_dir},
    fs::{self, DirEntry},
    io,
    path::{Path, PathBuf},
    process,
};

use camino::Utf8PathBuf;
use regex::Regex;
use unicode_normalization::UnicodeNormalization;

// Add new rule in lsty.json
impl DataManager {
    pub fn add_rule_to_json(
        &self,
        data: &mut DataObject,
        target_path: String,
        keyword: String,
    ) -> io::Result<()> {
        let current_dir = Utf8PathBuf::from_path_buf(current_dir().unwrap_or_default()).expect("");
        let current_dir_str = current_dir.as_str();
        let keyword_trimmed = keyword.trim_matches('\"').trim_matches('\'').to_string();
        let target_trimmed = target_path
            .trim_matches('\"')
            .trim_matches('\'')
            .to_string();

        match check_input(data, keyword_trimmed.clone(), target_trimmed.clone()) {
            InputCase::PathExists => process::exit(1),
            InputCase::CurrentDir => {
                let target_in_current_dir = format!(
                    "{}/{}",
                    current_dir_str,
                    target_trimmed.trim_start_matches("./")
                );
                data.targets.insert(keyword_trimmed, target_in_current_dir);
                println!("{}", msg_format(MsgKind::AddedRule));
                println!("{}", msg_format(MsgKind::ActualPathWillBeCreated));
            }
            // InputCase::InputInvalid => process::exit(1),
            InputCase::Normal => {
                data.targets.insert(keyword_trimmed, target_trimmed);
                println!("{}", msg_format(MsgKind::AddedRule));
            }
            InputCase::Invalid => process::exit(1),
        }
        Ok(())
    }

    // set an empty rule when lsty.json is empty or has invalid data
    pub fn set_empty_rule(
        &self,
        data: &mut DataModel,
        keyword: String,
        source_path: Utf8PathBuf,
        target_path: String,
    ) {
        let new_obj = DataObject {
            alias: String::new(),
            source: source_path.to_string(),
            targets: [(keyword, target_path)].iter().cloned().collect(),
        };
        data.data.push(new_obj)
    }
}

// Edit a rule
impl DataManager {
    pub fn edit_rule(&self, obj: &mut DataObject, arg: String, replacement: String) {
        let targets = obj.targets.clone();
        if let Some(target_path) = targets.get(&arg) {
            let confirmation =
                self.confirmation(target_path.clone(), arg.clone(), replacement.clone());
            if menu::get_yn_input(confirmation.message) {
                obj.targets.remove(&arg);
                if confirmation.bool {
                    obj.targets.insert(arg, replacement.clone());
                } else {
                    obj.targets.insert(replacement, target_path.to_string());
                }
                println!("rule updated.")
            } else {
                process::exit(1);
            }
        }
    }

    pub fn set_alias(&self, data: &mut DataObject, alias: String) -> Result<(), io::Error> {
        if alias.contains('/') || alias.contains('\\') {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                error_format(ErrorKind::InvalidAlias),
            ))
        } else {
            if menu::get_yn_input(msg_format(MsgKind::AliasUpdating(MsgArgs {
                primary_keyword: data.alias.clone(),
                secondary_keyword: alias.clone(),
                ..Default::default()
            }))) {
                data.alias = alias;
            }
            Ok(())
        }
    }

    pub fn confirmation(
        &self,
        target: String,
        arg: String,
        replacement: String,
    ) -> ConfirmationResult {
        let is_replacement_dir = Utf8PathBuf::from(&replacement).is_dir();
        let is_trimmed_replacement_dir = if let Some(i) = replacement.rfind('/') {
            let upper_dir = &replacement[..i];
            Utf8PathBuf::from(upper_dir).is_dir()
        } else {
            false
        };

        if replacement.is_empty() {
            println!(
                "{}",
                msg_format(MsgKind::NoKeywordOrPathForReplace(MsgArgs {
                    primary_keyword: arg,
                    primary_path: target.clone(),
                    ..Default::default()
                }))
            );
            process::exit(1);
        } else {
            match (is_replacement_dir, is_trimmed_replacement_dir) {
                // replacement is path
                (true, _) => ConfirmationResult {
                    bool: true,
                    message: msg_format(MsgKind::ChangeTargetPath(MsgArgs {
                        primary_keyword: arg.clone(),
                        primary_path: target.to_string(),
                        secondary_path: replacement.clone(),
                        ..Default::default()
                    })),
                },
                // replacement is not an existing path.
                // the path will be created when moving entries
                (false, true) => ConfirmationResult {
                    bool: true,
                    message: msg_format(MsgKind::ActualPathNonExists(MsgArgs {
                        primary_path: target.to_string(),
                        primary_keyword: arg.clone(),
                        secondary_path: replacement.clone(),
                        ..Default::default()
                    })),
                },
                // replacement is keyword
                _ => ConfirmationResult {
                    bool: false,
                    message: msg_format(MsgKind::ChangeTargetKeyword(MsgArgs {
                        primary_keyword: arg.clone(),
                        secondary_keyword: replacement.clone(),
                        ..Default::default()
                    })),
                },
            }
        }
    }
}

// Import rules from one of the other source paths
impl DataManager {
    pub fn import_rule(
        &self,
        data: &mut DataModel,
        alias: String,
        mut import_path: String,
    ) -> Result<(), io::Error> {
        let current_dir =
            Utf8PathBuf::from_path_buf(env::current_dir().unwrap_or_default()).expect("");

        let error_message = match (alias.is_empty(), alias.contains('/'), alias.contains('\\')) {
            (true, _, _) => error_format(ErrorKind::NotFoundRuleForPath),
            (false, true, _) | (false, _, true) => error_format(ErrorKind::InvalidAlias),
            (false, false, false) => error_format(ErrorKind::NotFoundAlias),
        };

        if let Some(data_map) = data
            .data
            .iter()
            .find(|obj| obj.source == import_path || obj.alias == alias && !alias.is_empty())
        {
            if import_path.is_empty() {
                import_path = data_map.source.clone();
            }

            let targets = data_map.targets.clone();
            let current_obj = match data.object_by_source_mut(current_dir.clone()) {
                Ok(obj) => obj,
                Err(_) => {
                    data.data.push(DataObject {
                        alias: "".to_string(),
                        source: current_dir.clone().to_string(),
                        targets: HashMap::new(),
                    });
                    data.data.last_mut().unwrap()
                }
            };

            println!("{}", msg_format(ImportYN));
            for (k, v) in &targets {
                println!(
                    "{}",
                    msg_format(KeywordAndTarget(MsgArgs {
                        primary_keyword: k.to_string(),
                        primary_path: v.to_string(),
                        ..Default::default()
                    }))
                );
            }
            if menu::get_yn_input(msg_format(ImportFromPath(MsgArgs {
                primary_path: import_path,
                ..Default::default()
            }))) {
                current_obj.targets.extend(targets);
                println!("{}", msg_format(MsgKind::ImportedRules));
            }
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, error_message))
        }
    }
}

// Remove rule
impl DataManager {
    pub fn remove_rule_from_json(
        &self,
        data: &mut DataObject,
        keyword: &str,
    ) -> Result<(), io::Error> {
        if let Some(target_path) = data.targets.get(keyword) {
            if menu::get_yn_input(msg_format(DeleteRule(MsgArgs {
                primary_keyword: keyword.to_string(),
                primary_path: target_path.to_string(),
                ..Default::default()
            }))) {
                data.targets.remove(keyword);
            } else {
                process::exit(1)
            }
        }
        Ok(())
    }
}

// Move(rename) entry
impl DataManager {
    pub fn rename_entries(&self, data: &DataObject, keyword: &str) -> Result<(), io::Error> {
        let mut moved_count = 0;
        let current_dir = current_dir()?;
        let current_dir_str = current_dir.to_str().expect("");

        let entries_map = self.scan_current_path(data, keyword)?;
        println!(
            "{}",
            msg_format(MsgKind::DisplaySource(MsgArgs {
                primary_path: current_dir_str.to_string(),
                ..Default::default()
            }))
        );
        for (target, vec) in entries_map {
            println!(
                "{}",
                msg_format(MsgKind::DisplayTarget(MsgArgs {
                    primary_path: target.to_string(),
                    ..Default::default()
                }))
            );
            for entry in vec.clone() {
                println!("{}", entry);
                let new_entry = format!("{}/{}", target, entry);
                let entry_symbol = menu::entry_symbol(&entry);

                println!("{}", new_entry);

                if !new_entry.is_empty() {
                    match !vec.is_empty()
                        && Path::new(&new_entry).exists()
                        // && vec.iter().any(|e| e == &new_entry)
                    {
                        true => {
                            println!("{}", msg_format(MsgKind::AlreadyExistsInTarget(MsgArgs {
                                secondary_keyword: entry_symbol, primary_path: entry, ..Default::default()
                            })));
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
            println!("{}", msg_format(NoItemsToMoveInSource));
        } else {
            println!("{}", msg_format(SimpleDone))
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
                fs::remove_dir_all(entry_path).expect("");
            }
            false => {
                fs::copy(entry_path, new_entry).expect("");
                fs::remove_file(entry_path).expect("");
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
                        error_format(ErrorKind::NoRuleForPath),
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

// Pre-rename(move) processings
impl DataManager {
    fn validate_pair(&self, targets: &HashMap<String, String>) -> Option<HashMap<String, String>> {
        let mut valid_pair = HashMap::new();

        for map in targets.iter() {
            if !PathBuf::from(map.1).exists() {
                eprintln!(
                    "{}",
                    msg_format(MsgKind::PathNonExistsCreating(MsgArgs {
                        primary_path: map.1.to_string(),
                        ..Default::default()
                    }))
                );
                fs::create_dir_all(map.1)
                    .unwrap_or_else(|_| panic!("{}", error_format(ErrorKind::CreateTargetDirFail)));
                valid_pair.insert(map.0.clone(), map.1.clone());
            } else {
                valid_pair.insert(map.0.clone(), map.1.clone());
            }
        }
        Some(valid_pair)
    }

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

    pub fn normalize_entry(&self, entry: &DirEntry) -> String {
        let entry_path = entry.path();
        let entry_name = match entry_path.file_name() {
            Some(name) => name,
            None => return String::new(),
        };
        entry_name.to_string_lossy().nfc().collect::<String>()
    }
}

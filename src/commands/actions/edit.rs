use crate::{
    cli::{
        cli_format::{msg_format, MsgArgs, MsgKind},
        menu,
    },
    data::{data_manager::DataManager, model::DataObject},
};
use camino::Utf8PathBuf;
use std::process;

impl DataManager {
    pub fn edit_rule(&self, obj: &mut DataObject, keyword: String, replacement: String) {
        let targets = obj.targets.clone();
        if let Some(target_path) = targets.get(&keyword) {
            if !replacement.is_empty() {
                let is_replacement_dir = Utf8PathBuf::from(&replacement).is_dir();
                let confirmation = if is_replacement_dir {
                    msg_format(MsgKind::TargetChangePath(MsgArgs {
                        primary_keyword: keyword.clone(),
                        primary_path: target_path.to_string(),
                        secondary_path: replacement.clone(),
                        ..Default::default()
                    }))
                } else if !replacement.contains('\\')
                    && !replacement.contains('/')
                    && !replacement.contains('~')
                {
                    msg_format(MsgKind::TargetChangeKeyword(MsgArgs {
                        primary_keyword: keyword.clone(),
                        secondary_keyword: replacement.clone(),
                        ..Default::default()
                    }))
                } else {
                    eprintln!("invalid path.");
                    return;
                };

                if menu::get_yn_input(confirmation) {
                    obj.targets.remove(&keyword);
                    if is_replacement_dir {
                        obj.targets.insert(keyword, replacement.clone());
                    } else {
                        obj.targets.insert(replacement, target_path.to_string());
                    }
                } else {
                    process::exit(1);
                }
            } else {
                println!(
                    "{}",
                    msg_format(MsgKind::NoKeywordOrPathForReplace(MsgArgs {
                        primary_keyword: keyword,
                        primary_path: target_path.clone(),
                        ..Default::default()
                    }))
                );
                process::exit(1);
            }
        }
        println!("rule updated.")
    }
}

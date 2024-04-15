use camino::Utf8PathBuf;

use crate::{
    cli::cli_format::{msg_format, MsgArgs, MsgKind},
    data::{
        data_manager::DataManager,
        model::{DataModel, DataObject},
    },
};
use std::{io, process};

#[derive(PartialEq, Debug)]
enum Case {
    DupQuotes,
    InputInvalid,
    PathExists,
    Normal,
}

impl DataManager {
    fn check_case(&self, d: &DataObject, p: String, k: String) -> Case {
        let case: Case;
        if let Some(existings) = d.targets.get(&k) {
            if existings == &p || d.targets.contains_key(&k) {
                case = Case::PathExists;
            } else {
                case = Case::Normal;
            }
        } else {
            if [k.clone(), p.clone()].iter().any(|s| {
                s.contains("\'") || s.contains("\"") || s.contains("\\") || s.contains("/")
            }) {
                if [k.clone(), p.clone()].iter().any(|s| {
                    s.starts_with("\"") && s.ends_with("\"")
                        || s.starts_with("\'") && s.ends_with("\'")
                }) {
                    case = Case::DupQuotes;
                } else {
                    case = Case::InputInvalid;
                }
            } else if p.contains("\"") || p.contains("\"") {
                case = Case::InputInvalid;
            } else {
                case = Case::Normal;
            }
        }
        println!("{:?}", case);
        case
    }

    pub fn add_rule_to_json(
        &self,
        data: &mut DataObject,
        target_path: String,
        keyword: String,
    ) -> io::Result<()> {
        match self.check_case(data, target_path.clone(), keyword.clone()) {
            Case::PathExists => {
                println!(
                    "{}",
                    msg_format(MsgKind::AlreadyExistsTryEdit(MsgArgs {
                        primary_keyword: keyword,
                        primary_path: target_path,
                        ..Default::default()
                    }),)
                );
                process::exit(1)
            }

            Case::InputInvalid => {
                println!("hmm");
                process::exit(1)
            }
            Case::DupQuotes => {
                let keyword_trimmed = keyword.trim_matches('\"').to_string();
                data.targets.insert(keyword_trimmed, target_path.clone());
                println!("rule added.");
            }
            Case::Normal => {
                data.targets.insert(keyword, target_path.clone());
                println!("rule added.");
            }
        }
        Ok(())
    }

    pub fn set_new_rules(
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

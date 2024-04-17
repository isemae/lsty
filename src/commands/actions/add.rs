use crate::{
    cli::cli_format::{msg_format, MsgArgs, MsgKind},
    data::{
        check_input::{check_input, InputCase},
        data_manager::DataManager,
        model::{DataModel, DataObject},
    },
};
use camino::Utf8PathBuf;
use std::{io, process};

impl DataManager {
    pub fn add_rule_to_json(
        &self,
        data: &mut DataObject,
        target_path: String,
        keyword: String,
    ) -> io::Result<()> {
        match check_input(data, target_path.clone(), keyword.clone()) {
            InputCase::PathExists => {
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

            InputCase::InputInvalid => {
                println!("hmm");
                process::exit(1)
            }
            InputCase::DupQuotes => {
                let keyword_trimmed = keyword.trim_matches('\"').trim_matches('\'').to_string();
                let target_trimmed = target_path
                    .trim_matches('\"')
                    .trim_matches('\'')
                    .to_string();
                data.targets.insert(keyword_trimmed, target_trimmed);
                println!("rule added.");
            }
            InputCase::Normal => {
                data.targets.insert(keyword, target_path);
                println!("rule added.");
            }
        }
        Ok(())
    }

    pub fn set_new_rule(
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

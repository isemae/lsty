use crate::cli::{
    cli_format::{error_format, msg_format, ErrorKind, MsgArgs, MsgKind},
    menu,
};
use crate::data::model::DataObject;
use std::env::current_dir;
#[derive(PartialEq, Debug)]
pub enum InputCase {
    PathExists,
    Normal,
    CurrentDir,
    Invalid,
}
use camino::Utf8PathBuf;

pub fn check_input(d: &DataObject, k: String, p: String) -> InputCase {
    let current_dir = Utf8PathBuf::from_path_buf(current_dir().unwrap_or_default()).expect("");
    let current_dir_str = current_dir.as_str();
    let target_in_current_dir = format!("{}/{}", current_dir_str, k);

    match d.targets.get(&k) {
        Some(existings) if existings == &p || d.targets.contains_key(&k) => {
            println!(
                "{}",
                msg_format(MsgKind::AlreadyExistsTryEdit(MsgArgs {
                    primary_keyword: k,
                    primary_path: p.to_string(),
                    ..Default::default()
                }),)
            );
            InputCase::PathExists
        }
        _ if p.starts_with('.') => {
            println!(
                "{}",
                msg_format(MsgKind::RuleInfo(MsgArgs {
                    primary_keyword: k,
                    primary_path: current_dir_str.to_string(),
                    secondary_path: target_in_current_dir,
                    ..Default::default()
                }),)
            );
            InputCase::CurrentDir
        }

        _ if p.is_empty() => {
            match menu::get_yn_input(msg_format(MsgKind::PathNotProvided(MsgArgs {
                primary_path: target_in_current_dir.clone(),
                primary_keyword: k.clone(),
                ..Default::default()
            }))) {
                true => {
                    println!(
                        "{}",
                        msg_format(MsgKind::RuleInfo(MsgArgs {
                            primary_keyword: k,
                            primary_path: current_dir_str.to_string(),
                            secondary_path: target_in_current_dir,
                            ..Default::default()
                        }))
                    );
                    InputCase::CurrentDir
                }
                false => {
                    eprintln!("{}", error_format(ErrorKind::PathShouldBeGiven));
                    InputCase::Invalid
                }
            }
        }
        _ => {
            println!(
                "{}",
                msg_format(MsgKind::RuleInfo(MsgArgs {
                    primary_keyword: k,
                    primary_path: current_dir_str.to_string(),
                    secondary_path: p,
                    ..Default::default()
                }))
            );
            InputCase::Normal
        }
    }
}

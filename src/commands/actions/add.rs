use crate::{
    cli::cli_format::{msg_format, MsgArgs, MsgKind},
    data::{
        check_input::{check_input, InputCase},
        data_manager::DataManager,
        model::{DataModel, DataObject},
    },
};
use camino::Utf8PathBuf;
use std::{env, fs, io, process};
use symlink::symlink_dir;

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
            InputCase::InputInvalid => process::exit(1),
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
        self.set_symlink(target_path.as_str());
        let new_obj = DataObject {
            alias: String::new(),
            source: source_path.to_string(),
            targets: [(keyword, target_path)].iter().cloned().collect(),
        };
        data.data.push(new_obj)
    }

    fn set_symlink(&self, target_path: &str) -> String {
        let exe_path = env::current_exe().unwrap();
        let exe_dir = exe_path.parent().unwrap();
        let link_name = target_path.rsplit_once("/").expect("").1;
        let link_path = format!("links/{}", link_name);

        let links_dir = exe_dir.join("links");
        if !Utf8PathBuf::from(link_path).exists() {
            fs::create_dir(links_dir).expect("")
        }

        symlink_dir(target_path, link_name).expect("");
        return format!("links/{}", target_path.rsplit_once("/").expect("").1);
    }
}

#[cfg(test)]
mod tests {
    use crate::data::data_manager::DataManager;
    use std::{fs, path::Path};

    #[test]
    fn symlink() {
        let data_manager = DataManager;
        let target_path = "../KR";
        let link_name = "links/KR";
        assert_eq!(data_manager.set_symlink(target_path), link_name);
        fs::remove_file(Path::new(&format!("./{}", link_name))).expect("Failed to remove symlink");
    }
}

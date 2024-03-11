use super::{
    data_manager::{self, DataManager},
    json_manager,
};
use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize, Debug)]
// pub struct DataModel {
//     pub pairs: Vec<Pair>,
//     pub sources: Vec<Source>,
//     pub targets: Vec<Target>,
//     pub json_manager: json_manager::JsonManager,
//     pub data_manager: data_manager::DataManager, // pub symlinks: Vec<SymlinkInfo>,
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct Pair {
    pub source_path: String,
    pub source_targets: Vec<SourceTarget>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SourceTarget {
    pub target: String,
    pub keyword: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Source {
    pub source_path: String,
    pub keywords: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Target {
    pub target_path: String,
    pub keyword: String,
}

#[derive(Serialize, Deserialize)]
pub struct DataModel {
    // pub data_manager: &'a data_manager::DataManager<'a>,
    pub pairs: Vec<Pair>,
}

#[derive(Debug)]
pub struct ErrorAvailableKeywords {
    pub message: String,
    pub context: Vec<String>,
}

impl DataModel {
    pub fn new(data_manager: &DataManager) -> Self {
        let pairs = data_manager
            .parse_json_data()
            .unwrap_or_else(|_| DataModel { pairs: Vec::new() })
            .pairs;
        DataModel { pairs }
    }

    pub fn add_source_target(
        &mut self,
        source_path: &str,
        target_path: &str,
        keyword: &str,
    ) -> Result<(), std::io::Error> {
        let data_manager: DataManager;

        if let Some(pair) = self
            .pairs
            .iter_mut()
            .find(|pair| pair.source_path == source_path)
        {
            let target_exist = pair
                .source_targets
                .iter()
                .any(|target| target.target == target_path && target.keyword == keyword);
            if !target_exist {
                pair.source_targets.push(SourceTarget {
                    target: target_path.to_string(),
                    keyword: keyword.to_string(),
                })
            }
        } else {
            let new_pair = Pair {
                source_path: source_path.to_string(),
                source_targets: vec![SourceTarget {
                    target: target_path.to_string(),
                    keyword: keyword.to_string(),
                }],
            };
            self.pairs.push(new_pair);
        }

        // DataManager::save_json_data(&data_manager)
        // .save_json_data("data.json", &self.pairs)
        // .map_err(|e| e);
        Ok(())
    }
}

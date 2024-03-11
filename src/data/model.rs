use super::{
    data_manager::{self, DataManager},
    json_manager,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// #[derive(Serialize, Deserialize, Debug)]
// pub struct DataModel {
//     pub pairs: Vec<Pair>,
//     pub sources: Vec<Source>,
//     pub targets: Vec<Target>,
//     pub json_manager: json_manager::JsonManager,
//     pub data_manager: data_manager::DataManager, // pub symlinks: Vec<SymlinkInfo>,
// }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataModel {
    // pub data_manager: &'a data_manager::DataManager<'a>,
    pub pairs: HashMap<String, HashMap<String, String>>,
}

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct Pair {
//     pub pairmap: HashMap<String, String>,
//     // pub source_path: String,
//     // pub source_targets: Vec<SourceTarget>,
// }

// #[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
// pub struct Keyword {
//     pub keyword: String,
// }
// #[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
// pub struct Target {
//     pub target_path: String,
// }

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct SourceTarget {
//     pub target: String,
//     pub keyword: String,
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct Source {
//     pub source_path: String,
//     pub keywords: Vec<String>,
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct Target {
//     pub target_path: String,
//     pub keyword: String,
// }

// #[derive(Debug)]
// pub struct ErrorAvailableKeywords {
//     pub message: String,
//     pub context: Vec<String>,
// }
// impl Pair {
//     pub fn new() -> Self {
//         Pair {
//             pairmap: HashMap::new(),
//         }
//     }
// }
impl DataModel {
    fn new() -> Self {
        DataModel {
            pairs: HashMap::new(),
        }
    }

    // fn add_pair(&mut self, source_path: String, target_path: String, keyword: String) {
    //     let pair = self.pairs.entry(source_path).or_insert(Pair {
    //         pairmap: HashMap::new(),
    //     });
    //     pair.pairmap
    //         .insert(Keyword { keyword }, Target { target_path });
    // }

    // fn delete_pair(&mut self, source_path: &str, target_path: &str, keyword: &str) {
    //     if let Some(pair) = self.pairs.get_mut(source_path) {
    //         pair.pairmap.remove(&Keyword {
    //             keyword: keyword.to_string(),
    //         });
    //     }
    // }

    // fn get_target(&self, path: &str, keyword: &str) -> Option<&String> {
    //     self.pairs
    //         .get(path)
    //         .and_then(|inner_map| inner_map.get(keyword))
    // }

    // pub fn add_source_target(
    //     &mut self,
    //     source_path: &str,
    //     target_path: &str,
    //     keyword: &str,
    // ) -> Result<(), std::io::Error> {
    //     let data_manager: DataManager;

    //     if let Some(pair) = self
    //         .pairs
    //         .iter_mut()
    //         .find(|pair| pair.source_path == source_path)
    //     {
    //         let target_exist = pair
    //             .source_targets
    //             .iter()
    //             .any(|target| target.target == target_path && target.keyword == keyword);
    //         if !target_exist {
    //             pair.source_targets.push(SourceTarget {
    //                 target: target_path.to_string(),
    //                 keyword: keyword.to_string(),
    //             })
    //         }
    //     } else {
    //         let new_pair = Pair {
    //             source_path: source_path.to_string(),
    //             source_targets: vec![SourceTarget {
    //                 target: target_path.to_string(),
    //                 keyword: keyword.to_string(),
    //             }],
    //         };
    //         self.pairs.push(new_pair);
    //     }

    //     // DataManager::save_json_data(&data_manager)
    //     // .save_json_data("data.json", &self.pairs)
    //     // .map_err(|e| e);
    //     Ok(())
    // }
}

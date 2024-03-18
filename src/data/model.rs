use super::data_manager::{self, DataManager};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct DataModel {
    pub data: Vec<DataObject>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct DataObject {
    pub alias: String,
    pub sources: Vec<String>,
    pub targets: HashMap<String, String>,
}

impl DataModel {
    fn new() -> Self {
        DataModel { data: Vec::new() }
    }
}

// impl Target {
//     pub fn new() -> Self {
//         Target {
//             targets: HashMap::new(),
//         }
//     }
//     pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> {
//         self.targets.iter()
//     }
// }

// impl std::ops::Deref for Target {
//     type Target = HashMap<String, String>;
//     fn deref(&self) -> &Self::Target {
//         &self.targets
//     }
// }

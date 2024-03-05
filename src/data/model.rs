use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DataModel {
    pub pairs: Vec<Pair>,
    pub sources: Vec<Source>,
    pub targets: Vec<Target>,
    // pub symlinks: Vec<SymlinkInfo>,
}

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

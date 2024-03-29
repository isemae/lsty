use super::data_manager::{self, DataManager};
use camino::Utf8PathBuf;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, io};

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct DataModel {
    pub data: Vec<DataObject>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct DataObject {
    pub alias: String,
    pub source: String,
    pub targets: HashMap<String, String>,
}

impl DataModel {
    pub fn new() -> Self {
        DataModel { data: Vec::new() }
    }

    pub fn object_by_source(
        &mut self,
        source_path: Utf8PathBuf,
    ) -> Result<&mut DataObject, io::Error> {
        if let Some(obj) = self
            .data
            .iter_mut()
            .find(|o| o.source == source_path.as_str())
        {
            Ok(obj)
        } else {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "no rules found for the current directory.",
            ));
        }
    }
}

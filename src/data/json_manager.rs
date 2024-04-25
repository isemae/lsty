use crate::data::model::DataModel;
use serde::ser::Error;
use std::{
    env,
    fs::File,
    io::{self, Read},
};

pub struct JsonManager;
impl JsonManager {
    pub fn parse_json_data(&self) -> Result<DataModel, serde_json::Error> {
        let exe_path = env::current_exe().unwrap();
        let exe_dir = exe_path.parent().unwrap();
        match File::open(exe_dir.join("lsty.json")) {
            Err(e) => Err(serde_json::Error::custom(format!(
                "failed to load data file: {}",
                e
            ))),
            Ok(mut file) => {
                let mut data = String::new();
                match file.read_to_string(&mut data) {
                    Ok(..) => {}
                    Err(e) => eprintln!("Error: {}", e),
                }
                serde_json::from_str(data.as_str())
            }
        }
    }

    pub fn save_json_data(&self, data: &DataModel) -> Result<(), io::Error> {
        let exe_path = env::current_exe().unwrap();
        let exe_dir = exe_path.parent().unwrap();

        let mut file = File::create(exe_dir.join("lsty.json"))?;
        serde_json::to_writer_pretty(&mut file, data)?;
        Ok(())
    }
}
// #[derive(Debug)]
// struct SerdeJsonError(serde_json::Error);
// impl fmt::Display for SerdeJsonError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "serde json Error: {}", self.0)
//     }
// }

// impl Error for SerdeJsonError {
//     fn source(&self) -> Option<&(dyn Error + 'static)> {
//         Some(&self.0)
//     }
// }

// #[derive(Debug)]
// enum Errors {
//     IOError(io::Error),
//     SerdeJsonError(serde_json::Error),
// }

// impl fmt::Display for Errors {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Errors::IOError(err) => write!(f, "IO Error: {}", err),
//             Errors::SerdeJsonError(err) => write!(f, "serde json Error: {}", err),
//         }
//     }
// }

// impl Error for Errors {}

// impl From<io::Error> for Errors {
//     fn from(err: io::Error) -> Self {
//         Errors::IOError(err)
//     }
// }
// impl From<serde_json::Error> for Errors {
//     fn from(err: serde_json::Error) -> Self {
//         Errors::SerdeJsonError(err)
//     }
// }

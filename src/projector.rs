use std::{collections::HashMap, path::PathBuf};

use serde::{Serialize, Deserialize};

use crate::config::Config;


#[derive(Debug, Serialize, Deserialize)]
struct Data {
    pub projector: HashMap<PathBuf, HashMap<String, String>>,
}

pub struct Projector {
    config: Config,
    data: Data,
}

// Another method for this is using traits to accomplish this as well.
// impl TryFrom<Config> for Projector

impl Projector {
    fn FromConfig(config: Config) -> Self {
        // Ensure that it exists
        if std::fs::metadata(config.config).is_ok() {
            let contents = std::fs::read_to_string(config.config);
            let contents = contents.unwrap_or("{\"projector\": {}}");
            let data = serde_json::from_str(&contents);
            let data = data.unwrap_or(Data {
                projector: HashMap::new(),
            });

            return Projector {
                config,
                data,
            };
        } 
    }
}
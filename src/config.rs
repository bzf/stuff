use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    client_id: uuid::Uuid,
    data_directory: String,
}

impl Config {
    pub fn new(data_directory: &Path) -> Self {
        Self {
            client_id: uuid::Uuid::new_v4(),
            data_directory: data_directory.to_str().unwrap().to_string(),
        }
    }
}

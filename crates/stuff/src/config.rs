use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Config {
    client_id: uuid::Uuid,
    data_directory: PathBuf,
}

impl Config {
    pub fn new(data_directory: &Path) -> Self {
        Self {
            client_id: uuid::Uuid::new_v4(),
            data_directory: data_directory.to_path_buf(),
        }
    }

    pub fn client_id(&self) -> &uuid::Uuid {
        &self.client_id
    }

    pub fn data_directory(&self) -> &Path {
        &self.data_directory
    }
}

use std::fs::File;
use std::io::prelude::*;

use crate::config::Config;
use crate::Store;

const CONFIG_FILENAME: &'static str = "config.yml";

pub struct Client {
    _name: String,
    xdg_dirs: xdg::BaseDirectories,
}

impl Client {
    pub fn new(name: &str) -> Self {
        Self {
            _name: name.to_string(),
            xdg_dirs: xdg::BaseDirectories::with_prefix(name).unwrap(),
        }
    }

    pub fn initialize(&self) {
        let data_directory = self.xdg_dirs.get_data_home();
        let initial_configuration = Config::new(data_directory.as_path());
        let filepath = self
            .xdg_dirs
            .place_config_file(CONFIG_FILENAME)
            .expect("Failed to create config directory");

        let mut file = File::create(filepath).unwrap();
        file.write_all(
            serde_yaml::to_string(&initial_configuration)
                .unwrap()
                .as_bytes(),
        )
        .unwrap();
    }

    pub fn config(&self) -> Option<Config> {
        self.xdg_dirs
            .find_config_file(CONFIG_FILENAME)
            .and_then(|filepath| File::open(filepath).ok())
            .and_then(|file| serde_yaml::from_reader(file).ok())
    }

    pub fn store(&self, config: &Config) -> Store {
        Store::new(&self.xdg_dirs, config)
    }
}

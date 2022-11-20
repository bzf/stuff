use std::fs::File;
use std::io::prelude::*;

const CONFIG_FILENAME: &'static str = "config.yml";

mod config;
mod project;
mod store;
mod task;

use config::Config;
pub use store::Store;

pub fn initialize_configuration(xdg_dirs: &xdg::BaseDirectories) {
    let data_directory = xdg_dirs.get_data_home();

    let initial_configuration = Config::new(data_directory.as_path());

    let filepath = xdg_dirs
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

pub fn get_config(xdg_dirs: &xdg::BaseDirectories) -> Option<Config> {
    xdg_dirs
        .find_config_file(CONFIG_FILENAME)
        .and_then(|filepath| File::open(filepath).ok())
        .and_then(|file| serde_yaml::from_reader(file).ok())
}

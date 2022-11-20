use clap::{Args, Parser, Subcommand, ValueEnum};
use std::ffi::OsStr;
use std::ffi::OsString;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::exit;
use xdg::BaseDirectories;

use crate::config::Config;

mod config;

const CONFIG_FILENAME: &'static str = "config.yml";

#[derive(Debug, Parser)]
#[command(name = "stuff")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Creates an initial configuration
    #[command()]
    Init,
}

fn main() {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("stuff").unwrap();
    let args = Cli::parse();

    match args.command {
        Commands::Init => match xdg_dirs.find_config_file(CONFIG_FILENAME) {
            Some(_) => {
                eprintln!("Configuration already exists");
                exit(1);
            }

            None => {
                let data_directories = xdg_dirs.get_data_dirs();
                let data_directory = data_directories
                    .first()
                    .expect("Could not get any data directories through xdg");

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
        },
    }
}

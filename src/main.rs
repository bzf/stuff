#![allow(unstable_name_collisions)]

use clap::{Args, Parser, Subcommand, ValueEnum};
use itertools::Itertools;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::exit;
use xdg::BaseDirectories;

use crate::config::Config;
use crate::store::Store;

mod config;
mod store;
mod task;

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

    /// Adds a task
    #[command(arg_required_else_help = true)]
    Add {
        #[arg(required = true)]
        title_parts: Vec<String>,
    },

    /// Lists all tasks
    #[command()]
    Tasks,

    /// Mark as task as completed
    #[command()]
    Done {
        #[arg(required = true)]
        task_id: uuid::Uuid,
    },
}

fn main() {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("stuff").unwrap();
    let args = Cli::parse();

    match (args.command, get_config(&xdg_dirs)) {
        (Commands::Init, configuration) => match configuration {
            Some(_) => {
                eprintln!("Configuration already exists");
                exit(1);
            }

            None => {
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
        },

        (Commands::Add { title_parts }, Some(config)) => {
            let mut store = Store::new(&xdg_dirs, &config);

            let task_title: String = title_parts
                .into_iter()
                .intersperse(" ".to_string())
                .collect();

            store.add_task(&task_title)
        }

        (Commands::Tasks, Some(config)) => {
            let store = Store::new(&xdg_dirs, &config);

            for task in store.tasks() {
                let done_label = match task.completed_at() {
                    Some(_) => "(done)",
                    None => "",
                };

                println!("{} | {} {}", task.id(), task.title(), done_label);
            }
        }

        (Commands::Done { task_id }, Some(config)) => {
            let mut store = Store::new(&xdg_dirs, &config);
            store.mark_task_as_complete(&task_id);
        }

        (_, None) => {
            eprintln!("No configuration file found");
            exit(1);
        }
    }
}

fn get_config(xdg_dirs: &xdg::BaseDirectories) -> Option<Config> {
    xdg_dirs
        .find_config_file(CONFIG_FILENAME)
        .and_then(|filepath| File::open(filepath).ok())
        .and_then(|file| serde_yaml::from_reader(file).ok())
}

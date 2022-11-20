#![allow(unstable_name_collisions)]

use clap::{Parser, Subcommand};
use itertools::Itertools;
use std::process::exit;

use stuff::{get_config, initialize_configuration, Store};

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

    /// Create a new project
    #[command()]
    CreateProject {
        #[arg(required = true)]
        name_parts: Vec<String>,
    },

    /// Lists all projects
    #[command()]
    Projects,
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
                initialize_configuration(&xdg_dirs);
            }
        },

        (Commands::Add { title_parts }, Some(config)) => {
            let mut store = Store::new(&xdg_dirs, &config);
            store.add_task(&concat_parts(title_parts));
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

        (Commands::CreateProject { name_parts }, Some(config)) => {
            let mut store = Store::new(&xdg_dirs, &config);
            store.create_project(&concat_parts(name_parts));
        }

        (Commands::Projects, Some(config)) => {
            let store = Store::new(&xdg_dirs, &config);

            for project in store.projects() {
                println!("{} | {}", project.id(), project.name());
            }
        }

        (_, None) => {
            eprintln!("No configuration file found");
            exit(1);
        }
    }
}

fn concat_parts(parts: Vec<String>) -> String {
    parts.into_iter().intersperse(" ".to_string()).collect()
}

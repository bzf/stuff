#![allow(unstable_name_collisions)]

use clap::{Parser, Subcommand};
use itertools::Itertools;
use std::process::exit;

use stuff::Client;

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
    let client = Client::new("stuff");
    let args = Cli::parse();

    match (args.command, client.config()) {
        (Commands::Init, configuration) => match configuration {
            Some(_) => {
                eprintln!("Configuration already exists");
                exit(1);
            }

            None => {
                client.initialize();
            }
        },

        (Commands::Add { title_parts }, Some(config)) => {
            client.store(&config).add_task(&concat_parts(title_parts));
        }

        (Commands::Tasks, Some(config)) => {
            for task in client.store(&config).tasks() {
                let done_label = match task.completed_at() {
                    Some(_) => "(done)",
                    None => "",
                };

                println!("{} | {} {}", task.id(), task.title(), done_label);
            }
        }

        (Commands::Done { task_id }, Some(config)) => {
            client.store(&config).mark_task_as_complete(&task_id);
        }

        (Commands::CreateProject { name_parts }, Some(config)) => {
            client
                .store(&config)
                .create_project(&concat_parts(name_parts));
        }

        (Commands::Projects, Some(config)) => {
            for project in client.store(&config).projects() {
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

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use crate::config::Config;
use crate::project::Project;
use crate::task::Task;

#[derive(Serialize, Deserialize)]
struct EventPayload {
    event: Event,
    timestamp: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
enum Event {
    AddTask { uuid: uuid::Uuid, title: String },
    MarkTaskAsComplete { task_id: uuid::Uuid },
    CreateProject { uuid: uuid::Uuid, name: String },
}

pub struct Store {
    _config: Config,
    payload_path: PathBuf,

    events: Vec<EventPayload>,
}

impl Store {
    pub fn new(xdg_dirs: &xdg::BaseDirectories, config: &Config) -> Self {
        let filename = format!("{}.json", config.client_id());
        let payload_path = xdg_dirs.place_data_file(&filename).unwrap();

        let events: Vec<EventPayload> = File::open(&payload_path)
            .ok()
            .and_then(|file_reader| serde_json::from_reader(file_reader).ok())
            .unwrap_or(Vec::new());

        Self {
            _config: config.clone(),
            payload_path,

            events,
        }
    }

    pub fn tasks(&self) -> Vec<Task> {
        reduce_events_to_tasks(&self.events)
    }

    pub fn projects(&self) -> Vec<Project> {
        reduce_events_to_projects(&self.events)
    }

    pub fn add_task(&mut self, title: &str) {
        self.push_event(Event::AddTask {
            uuid: uuid::Uuid::new_v4(),
            title: title.to_string(),
        })
    }

    pub fn mark_task_as_complete(&mut self, task_id: &uuid::Uuid) {
        self.push_event(Event::MarkTaskAsComplete {
            task_id: task_id.clone(),
        });
    }

    pub fn create_project(&mut self, name: &str) {
        self.push_event(Event::CreateProject {
            uuid: uuid::Uuid::new_v4(),
            name: name.to_string(),
        })
    }

    fn push_event(&mut self, event: Event) {
        let event_payload = EventPayload {
            event,
            timestamp: Utc::now(),
        };

        self.events.push(event_payload);
        self.write_to_disk();
    }

    fn write_to_disk(&mut self) {
        let payload =
            serde_json::to_string(&self.events).expect("Failed to serialize events to JSON");

        let mut file = File::create(&self.payload_path).unwrap();
        file.write_all(payload.as_bytes()).unwrap();
    }
}

fn reduce_events_to_tasks(event_payloads: &Vec<EventPayload>) -> Vec<Task> {
    let mut tasks = HashMap::new();

    for event_payload in event_payloads {
        match &event_payload.event {
            Event::AddTask { uuid, title } => {
                tasks.insert(
                    uuid.clone(),
                    Task::new(uuid.clone(), title.clone(), event_payload.timestamp),
                );
            }

            Event::MarkTaskAsComplete { task_id } => {
                tasks
                    .entry(*task_id)
                    .and_modify(|task| task.set_completed_at(&event_payload.timestamp));
            }

            _ => (),
        }
    }

    return tasks.into_values().collect();
}

fn reduce_events_to_projects(event_payloads: &Vec<EventPayload>) -> Vec<Project> {
    let mut projects = HashMap::new();

    for event_payload in event_payloads {
        match &event_payload.event {
            Event::CreateProject { uuid, name } => {
                projects.insert(
                    uuid.clone(),
                    Project::new(uuid.clone(), name.clone(), event_payload.timestamp),
                );
            }

            _ => (),
        }
    }

    return projects.into_values().collect();
}

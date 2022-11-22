use chrono::prelude::*;
use glob::glob;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use crate::config::Config;
use crate::project::Project;
use crate::task::Task;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct EventPayload {
    event: Event,
    timestamp: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
enum Event {
    AddTask { uuid: uuid::Uuid, title: String },
    MarkTaskAsComplete { task_id: uuid::Uuid },
    MarkTaskAsIncomplete { task_id: uuid::Uuid },
    CreateProject { uuid: uuid::Uuid, name: String },
}

pub struct Store {
    _config: Config,
    payload_path: PathBuf,

    events: HashMap<uuid::Uuid, Vec<EventPayload>>,
}

impl Store {
    pub fn new(config: &Config) -> Self {
        let filename = format!("{}.json", config.client_id());
        let mut payload_path = config.data_directory().to_path_buf();
        payload_path.push(filename);

        let mut events = HashMap::new();

        for entry in glob(&format!(
            "{}/*.json",
            &config
                .data_directory()
                .to_str()
                .expect("Failed to cast data directory to string")
        ))
        .expect("Failed to read glob pattern")
        {
            match entry {
                Ok(path) => {
                    let uuid = path
                        .file_stem()
                        .and_then(|stem| stem.to_str())
                        .and_then(|stem| uuid::Uuid::parse_str(stem).ok());

                    if let Some(client_id) = uuid {
                        let client_events: Vec<EventPayload> = File::open(&path)
                            .ok()
                            .and_then(|file_reader| serde_json::from_reader(file_reader).ok())
                            .unwrap_or(Vec::new());

                        events.insert(client_id, client_events);
                    }
                }

                Err(_e) => (),
            }
        }

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

    pub fn mark_task_as_incomplete(&mut self, task_id: &uuid::Uuid) {
        self.push_event(Event::MarkTaskAsIncomplete {
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

        self.events
            .entry(*self._config.client_id())
            .and_modify(|v| v.push(event_payload.clone()))
            .or_insert(Vec::from([event_payload]));

        self.write_to_disk();
    }

    fn write_to_disk(&mut self) {
        let payload = self
            .events
            .get(self._config.client_id())
            .and_then(|events| serde_json::to_string(&events).ok())
            .unwrap_or("[]".to_string());

        let mut file = File::create(&self.payload_path).unwrap();
        file.write_all(payload.as_bytes()).unwrap();
    }
}

fn reduce_events_to_tasks(event_payloads: &HashMap<uuid::Uuid, Vec<EventPayload>>) -> Vec<Task> {
    let mut tasks = HashMap::new();

    for event_payload in event_payloads.values().flatten() {
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

            Event::MarkTaskAsIncomplete { task_id } => {
                tasks
                    .entry(*task_id)
                    .and_modify(|task| task.set_incomplete());
            }

            _ => (),
        }
    }

    return tasks.into_values().collect();
}

fn reduce_events_to_projects(
    event_payloads: &HashMap<uuid::Uuid, Vec<EventPayload>>,
) -> Vec<Project> {
    let mut projects = HashMap::new();

    for event_payload in event_payloads.values().flatten() {
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

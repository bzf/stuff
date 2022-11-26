use chrono::prelude::*;
use glob::glob;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver, Sender};

use crate::config::Config;
use crate::event::{Event, EventPayload};
use crate::state::State;

pub struct Store {
    _config: Config,
    payload_path: PathBuf,

    events: HashMap<uuid::Uuid, Vec<EventPayload>>,

    /// Up-to-date represantation of `events` for passing to clients.
    state: State,

    notify_sender: Sender<State>,
}

impl Store {
    pub fn new(config: &Config) -> (Self, Receiver<State>) {
        let (notify_sender, notify_receiver) = channel();

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

        let mut event_payloads: Vec<&EventPayload> = events.values().flatten().collect();
        event_payloads.sort_by(|&a, &b| a.timestamp.partial_cmp(&b.timestamp).unwrap());

        (
            Self {
                _config: config.clone(),
                payload_path,
                notify_sender,

                state: State::from_events(event_payloads),
                events,
            },
            notify_receiver,
        )
    }

    pub fn state(&self) -> &State {
        &self.state
    }

    pub fn add_task(
        &mut self,
        title: &str,
        description: Option<String>,
        project_id: Option<uuid::Uuid>,
        project_heading_id: Option<uuid::Uuid>,
    ) {
        self.push_event(Event::AddTask {
            uuid: uuid::Uuid::new_v4(),
            title: title.to_string(),
            description,
            project_id,
            project_heading_id,
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

    pub fn move_task_to_project(&mut self, task_id: &uuid::Uuid, project_id: &uuid::Uuid) {
        self.push_event(Event::MoveTaskToProject {
            task_id: task_id.clone(),
            project_id: project_id.clone(),
        })
    }

    pub fn move_task_to_inbox(&mut self, task_id: &uuid::Uuid) {
        self.push_event(Event::MoveTaskToInbox {
            task_id: task_id.clone(),
        })
    }

    pub fn create_project(&mut self, name: &str) {
        self.push_event(Event::CreateProject {
            uuid: uuid::Uuid::new_v4(),
            name: name.to_string(),
        })
    }

    pub fn add_project_heading(&mut self, project_id: &uuid::Uuid, name: &str) {
        self.push_event(Event::AddProjectHeading {
            uuid: uuid::Uuid::new_v4(),
            project_id: *project_id,
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
            .or_insert(Vec::from([event_payload.clone()]));

        self.write_to_disk();
        self.state.apply_event(&event_payload);
        self.notify_sender.send(self.state.clone()).unwrap();
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

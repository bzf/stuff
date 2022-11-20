use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use crate::config::Config;

#[derive(Serialize, Deserialize)]
struct EventPayload {
    event: Event,
    timestamp: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
enum Event {
    AddTask { uuid: uuid::Uuid, title: String },
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

    pub fn add_task(&mut self, title: &str) {
        self.push_event(Event::AddTask {
            uuid: uuid::Uuid::new_v4(),
            title: title.to_string(),
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

        println!("{:?}", self.payload_path);

        let mut file = File::create(&self.payload_path).unwrap();
        file.write_all(payload.as_bytes()).unwrap();
    }
}

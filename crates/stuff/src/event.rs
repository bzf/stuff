use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EventPayload {
    pub event: Event,
    pub timestamp: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Event {
    AddTask { uuid: uuid::Uuid, title: String },
    MarkTaskAsComplete { task_id: uuid::Uuid },
    MarkTaskAsIncomplete { task_id: uuid::Uuid },
    CreateProject { uuid: uuid::Uuid, name: String },
}

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EventPayload {
    pub event: Event,
    pub timestamp: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Event {
    AddTask {
        uuid: Uuid,
        title: String,
        description: Option<String>,
        project_id: Option<uuid::Uuid>,
        project_heading_id: Option<uuid::Uuid>,
    },
    MarkTaskAsComplete {
        task_id: Uuid,
    },
    MarkTaskAsIncomplete {
        task_id: Uuid,
    },
    MoveTaskToProject {
        task_id: Uuid,
        project_id: Uuid,
    },
    MoveTaskToInbox {
        task_id: Uuid,
    },
    CreateProject {
        uuid: Uuid,
        name: String,
    },
    AddProjectHeading {
        uuid: Uuid,
        project_id: Uuid,
        name: String,
        index: usize,
    },
    CreateArea {
        uuid: Uuid,
        name: String,
    },
}

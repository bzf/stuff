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
        area_id: Option<uuid::Uuid>,
    },
    MoveTaskToPosition {
        task_id: Uuid,
        position: usize,
    },
    UpdateTaskTitle {
        task_id: Uuid,
        title: String,
    },
    UpdateTaskDescription {
        task_id: Uuid,
        description: Option<String>,
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
        area_id: Option<uuid::Uuid>,
    },
    RenameProject {
        project_id: Uuid,
        name: String,
    },
    MoveProjectToPosition {
        project_id: Uuid,
        position: usize,
    },
    AddProjectHeading {
        uuid: Uuid,
        project_id: Uuid,
        name: String,
        index: usize,
    },
    MoveTaskToProjectHeading {
        task_id: Uuid,
        project_heading_id: Uuid,
    },
    ClearTaskProjectHeading {
        task_id: Uuid,
    },
    CreateArea {
        uuid: Uuid,
        name: String,
    },
}

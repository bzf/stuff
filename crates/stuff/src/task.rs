use chrono::prelude::*;
use serde::Serialize;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    id: uuid::Uuid,
    title: String,
    created_at: DateTime<Utc>,

    completed_at: Option<DateTime<Utc>>,
}

impl Task {
    pub fn new(id: uuid::Uuid, title: String, created_at: DateTime<Utc>) -> Self {
        Self {
            id,
            title,
            created_at,

            completed_at: None,
        }
    }

    pub fn id(&self) -> &uuid::Uuid {
        &self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn completed_at(&self) -> &Option<DateTime<Utc>> {
        &self.completed_at
    }

    pub fn set_completed_at(&mut self, timestamp: &DateTime<Utc>) {
        self.completed_at = Some(*timestamp);
    }

    pub fn set_incomplete(&mut self) {
        self.completed_at = None;
    }
}

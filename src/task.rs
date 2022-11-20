use chrono::prelude::*;

#[derive(Clone)]
pub struct Task {
    id: uuid::Uuid,
    title: String,
    _created_at: DateTime<Utc>,
}

impl Task {
    pub fn new(id: uuid::Uuid, title: String, created_at: DateTime<Utc>) -> Self {
        Self {
            id,
            title,
            _created_at: created_at,
        }
    }

    pub fn id(&self) -> &uuid::Uuid {
        &self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }
}

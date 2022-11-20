use chrono::prelude::*;

#[derive(Clone)]
pub struct Project {
    id: uuid::Uuid,
    name: String,
    _created_at: DateTime<Utc>,
}

impl Project {
    pub fn new(id: uuid::Uuid, name: String, created_at: DateTime<Utc>) -> Self {
        Self {
            id,
            name,
            _created_at: created_at,
        }
    }

    pub fn id(&self) -> &uuid::Uuid {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

use chrono::prelude::*;
use serde::Serialize;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectHeading {
    id: uuid::Uuid,
    project_id: uuid::Uuid,
    name: String,
    created_at: DateTime<Utc>,
}

impl ProjectHeading {
    pub fn new(
        id: uuid::Uuid,
        project_id: uuid::Uuid,
        name: String,
        created_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            project_id,
            name,
            created_at,
        }
    }
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    id: uuid::Uuid,
    pub name: String,
    created_at: DateTime<Utc>,
    pub area_id: Option<uuid::Uuid>,
}

impl Project {
    pub fn new(
        id: uuid::Uuid,
        name: String,
        area_id: Option<uuid::Uuid>,
        created_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            name,
            created_at,
            area_id,
        }
    }

    pub fn id(&self) -> &uuid::Uuid {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

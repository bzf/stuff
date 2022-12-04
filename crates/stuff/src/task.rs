use chrono::prelude::*;
use serde::Serialize;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    id: uuid::Uuid,
    pub title: String,
    pub description: String,
    created_at: DateTime<Utc>,

    completed_at: Option<DateTime<Utc>>,
    pub project_id: Option<uuid::Uuid>,
    pub project_heading_id: Option<uuid::Uuid>,
    pub area_id: Option<uuid::Uuid>,

    pub defer_date: Option<chrono::NaiveDate>,
}

impl Task {
    pub fn new(
        id: uuid::Uuid,
        title: String,
        description: String,
        created_at: DateTime<Utc>,
        project_id: Option<uuid::Uuid>,
        project_heading_id: Option<uuid::Uuid>,
        area_id: Option<uuid::Uuid>,
    ) -> Self {
        Self {
            id,
            title,
            description,
            created_at,
            project_id,
            project_heading_id,
            area_id,

            completed_at: None,
            defer_date: None,
        }
    }

    pub fn id(&self) -> &uuid::Uuid {
        &self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> &str {
        &self.description
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

    pub fn set_project_id(&mut self, project_id: Option<uuid::Uuid>) {
        self.project_id = project_id;
    }
}

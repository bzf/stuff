use serde::Serialize;

use crate::event::{Event, EventPayload};
use crate::{Project, Task};

#[derive(Serialize, Clone)]
pub struct State {
    tasks: indexmap::map::IndexMap<uuid::Uuid, Task>,
    projects: indexmap::map::IndexMap<uuid::Uuid, Project>,
}

impl State {
    pub fn new() -> Self {
        Self {
            tasks: indexmap::map::IndexMap::new(),
            projects: indexmap::map::IndexMap::new(),
        }
    }

    pub fn tasks(&self) -> Vec<Task> {
        self.tasks.values().cloned().collect()
    }

    pub fn projects(&self) -> Vec<Project> {
        self.projects.values().cloned().collect()
    }

    pub fn from_events(event_payloads: Vec<&EventPayload>) -> Self {
        let mut state = Self::new();

        for event in event_payloads {
            state.apply_event(event);
        }

        return state;
    }

    pub fn apply_event(&mut self, event_payload: &EventPayload) {
        match &event_payload.event {
            Event::AddTask {
                uuid,
                title,
                project_id,
            } => {
                self.tasks.insert(
                    uuid.clone(),
                    Task::new(
                        uuid.clone(),
                        title.clone(),
                        event_payload.timestamp,
                        *project_id,
                    ),
                );
            }

            Event::MarkTaskAsComplete { task_id } => {
                self.tasks
                    .entry(*task_id)
                    .and_modify(|task| task.set_completed_at(&event_payload.timestamp));
            }

            Event::MarkTaskAsIncomplete { task_id } => {
                self.tasks
                    .entry(*task_id)
                    .and_modify(|task| task.set_incomplete());
            }

            Event::MoveTaskToInbox { task_id } => {
                self.tasks
                    .entry(*task_id)
                    .and_modify(|task| task.set_project_id(None));
            }

            Event::MoveTaskToProject {
                task_id,
                project_id,
            } => {
                self.tasks
                    .entry(*task_id)
                    .and_modify(|task| task.set_project_id(Some(*project_id)));
            }

            Event::CreateProject { uuid, name } => {
                self.projects.insert(
                    uuid.clone(),
                    Project::new(uuid.clone(), name.clone(), event_payload.timestamp),
                );
            }
        }
    }
}

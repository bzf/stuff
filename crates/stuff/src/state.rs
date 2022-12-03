use serde::Serialize;

use crate::event::{Event, EventPayload};
use crate::{Area, Project, ProjectHeading, Task};

#[derive(Serialize, Clone)]
pub struct State {
    #[serde(with = "indexmap::serde_seq")]
    tasks: indexmap::map::IndexMap<uuid::Uuid, Task>,

    #[serde(with = "indexmap::serde_seq")]
    pub projects: indexmap::map::IndexMap<uuid::Uuid, Project>,

    #[serde(with = "indexmap::serde_seq")]
    pub areas: indexmap::map::IndexMap<uuid::Uuid, Area>,

    project_headings: indexmap::map::IndexMap<uuid::Uuid, ProjectHeading>,
}

impl State {
    pub fn new() -> Self {
        Self {
            tasks: indexmap::map::IndexMap::new(),
            projects: indexmap::map::IndexMap::new(),
            project_headings: indexmap::map::IndexMap::new(),
            areas: indexmap::map::IndexMap::new(),
        }
    }

    pub fn tasks(&self) -> Vec<Task> {
        self.tasks.values().cloned().collect()
    }

    pub fn projects(&self) -> Vec<Project> {
        self.projects.values().cloned().collect()
    }

    pub fn project_headings(&self) -> Vec<ProjectHeading> {
        self.project_headings.values().cloned().collect()
    }

    pub fn areas(&self) -> Vec<Area> {
        self.areas.values().cloned().collect()
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
                description,
                project_id,
                project_heading_id,
                area_id,
            } => {
                self.tasks.insert(
                    uuid.clone(),
                    Task::new(
                        uuid.clone(),
                        title.clone(),
                        description.clone().unwrap_or(String::new()),
                        event_payload.timestamp,
                        *project_id,
                        *project_heading_id,
                        *area_id,
                    ),
                );
            }

            Event::MoveTaskToPosition { task_id, position } => {
                if let Some(current_position) = self.tasks.get_index_of(task_id) {
                    self.tasks.move_index(current_position, *position);
                }
            }

            Event::UpdateTaskTitle { task_id, title } => {
                self.tasks
                    .entry(*task_id)
                    .and_modify(|task| task.title = title.to_string());
            }

            Event::UpdateTaskDescription {
                task_id,
                description,
            } => {
                self.tasks.entry(*task_id).and_modify(|task| {
                    task.description = description.clone().unwrap_or(String::new())
                });
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

            Event::AddProjectHeading {
                uuid,
                project_id,
                name,
                index,
            } => {
                let (internal_index, _) = self.project_headings.insert_full(
                    *uuid,
                    ProjectHeading::new(*uuid, *project_id, name.clone(), event_payload.timestamp),
                );

                self.project_headings.move_index(internal_index, *index);
            }

            Event::CreateArea { uuid, name } => {
                self.areas
                    .insert(uuid.clone(), Area::new(*uuid, name.clone()));
            }
        }
    }
}

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct Area {
    id: uuid::Uuid,
    pub name: String,
}

impl Area {
    pub fn new(id: uuid::Uuid, name: String) -> Self {
        Self { id, name }
    }
}

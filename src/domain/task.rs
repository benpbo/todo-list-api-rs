use serde::{Deserialize, Serialize};
use std::fmt::Display;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub id: TaskId,
    pub description: String,
    pub is_completed: bool,
}

impl Task {
    pub fn new(id: TaskId, description: String, is_completed: bool) -> Self {
        Self {
            id,
            description,
            is_completed,
        }
    }
}

#[derive(Deserialize, Serialize, Hash, Eq, PartialEq, Ord, PartialOrd, Debug, Clone)]
pub struct TaskId(Uuid);

impl TaskId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Display for TaskId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

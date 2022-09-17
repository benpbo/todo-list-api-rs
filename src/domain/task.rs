use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize)]
pub struct Task {
    pub id: TaskId,
    pub description: String,
    pub completed: TaskCompleted,
}

impl Task {
    pub fn new(id: TaskId, description: String, completed: TaskCompleted) -> Self {
        Self {
            id,
            description,
            completed,
        }
    }
}

#[derive(Serialize, Hash, Eq, PartialEq, Ord, PartialOrd, Debug, Clone)]
pub struct TaskId(Uuid);

impl TaskId {
    pub fn new() -> TaskId {
        TaskId(Uuid::new_v4())
    }
}

#[derive(Clone, Debug, Serialize)]
pub enum TaskCompleted {
    Completed,
    NotCompleted,
}

use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize)]
pub struct Task {
    pub id: TaskId,
    pub description: String,
    pub completed: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct TaskId(Uuid);

impl TaskId {
    pub fn new() -> TaskId {
        TaskId(Uuid::new_v4())
    }
}

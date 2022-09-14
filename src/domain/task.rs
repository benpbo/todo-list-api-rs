use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Task {
    pub id: i64,
    pub description: String,
    pub completed: bool,
}

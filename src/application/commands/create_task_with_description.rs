use crate::domain::Task;
use async_trait::async_trait;

#[async_trait]
pub trait CreateTaskWithDescriptionCommand {
    async fn execute(&mut self, description: String) -> anyhow::Result<Task>;
}

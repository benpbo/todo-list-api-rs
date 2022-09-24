use crate::{
    application::TaskRepository,
    domain::{Task, TaskId},
};
use anyhow::Result;
use async_trait::async_trait;
use aws_sdk_dynamodb::Client;
use serde_dynamo::aws_sdk_dynamodb_0_19::{from_item, from_items, to_attribute_value, to_item};

pub struct DynamoDbTaskRepository<'a> {
    client: Client,
    table: &'a str,
}

impl<'a> DynamoDbTaskRepository<'a> {
    pub fn new(client: Client, table: &'a str) -> Self {
        Self { client, table }
    }
}

#[async_trait]
impl TaskRepository for DynamoDbTaskRepository<'_> {
    async fn get_task_by_id(&self, id: &TaskId) -> Result<Option<Task>> {
        let id_value = to_attribute_value(id)?;
        let result = self
            .client
            .get_item()
            .table_name(self.table)
            .key("id", id_value)
            .send()
            .await?;

        match result.item() {
            Some(item) => Ok(Some(from_item(item.to_owned())?)),
            None => Ok(None),
        }
    }

    async fn get_all_tasks(&self) -> Result<Vec<Task>> {
        let result = self.client.scan().table_name(self.table).send().await?;

        match result.items() {
            Some(items) => Ok(from_items(items.to_vec())?),
            None => Ok(Vec::<Task>::new()),
        }
    }

    async fn add_task(&mut self, task: &Task) -> Result<()> {
        let item = to_item(task)?;
        self.client
            .put_item()
            .table_name(self.table)
            .set_item(Some(item))
            .send()
            .await?;

        Ok(())
    }
}

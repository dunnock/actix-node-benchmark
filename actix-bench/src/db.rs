use crate::{errors::BenchError, models::Task, GetTasksQuery};
use deadpool_postgres::Pool;
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_postgres::types::Type;
use std::sync::Arc;

impl GetTasksQuery {
    pub fn get_statement(&self) -> &'static str {
        if self.full.is_some() && self.full.unwrap() { 
            include_str!("sql/get_tasks_full.sql")
        } else {
            include_str!("sql/get_tasks.sql")
        }
    }
}

fn like(s: Option<String>) -> Option<String> {
    s.map(|s| format!("%{}%", s))
}

pub async fn get_tasks(pool: Arc<Pool>, query: GetTasksQuery) -> Result<Vec<Task>, BenchError> {
    let _stmt = query.get_statement();

    let client = pool.get().await?;
    let stmt = client.prepare_typed(&_stmt, &[Type::VARCHAR, Type::VARCHAR, Type::OID]).await.unwrap();

    client.query(
            &stmt,
            &[ &like(query.assignee_name), &like(query.summary), &query.limit.or(Some(10)) ],
        ).await?
        .iter()
        .map(|row| Task::from_row_ref(row).map_err(BenchError::from))
        .collect()
}

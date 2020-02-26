use crate::{errors::BenchError, models::Task, GetTasksQuery};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_postgres::types::Type;

impl GetTasksQuery {
	pub fn is_full(&self) -> bool {
		self.full.is_some() && self.full.unwrap()
	}
}

pub async fn get_tasks(client: &Client, query: GetTasksQuery) -> Result<Vec<Task>, BenchError> {
	let _stmt = if query.is_full() { 
		include_str!("sql/get_tasks_full.sql")
	} else {
		include_str!("sql/get_tasks.sql")
	};
	let stmt = client.prepare_typed(&_stmt, &[Type::VARCHAR, Type::VARCHAR, Type::OID]).await.unwrap();
	let like = |s: Option<String>| s.map(|s| format!("%{}%", s));

	client
		.query(
			&stmt,
			&[
				&like(query.assignee_name),
				&like(query.summary),
				&query.limit
			],
		)
		.await?
		.iter()
		.map(|row| Task::from_row_ref(row).map_err(BenchError::from))
		.collect()
}

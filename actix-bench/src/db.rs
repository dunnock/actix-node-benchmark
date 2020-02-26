use crate::{errors::BenchError, models::Task, GetTasksQuery};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_postgres::types::Type;
use std::convert::TryFrom;
use deadpool_postgres::Pool;
use std::sync::Arc;

impl GetTasksQuery {
	pub fn is_full(&self) -> bool {
		self.full.is_some() && self.full.unwrap()
	}
}

pub async fn get_tasks(db_pool: Arc<Pool>, query: GetTasksQuery) -> Result<Vec<Task>, BenchError> {
	let (_stmt, name) = if query.is_full() { 
		(include_str!("sql/get_tasks_full.sql"), "f")
	} else {
		(include_str!("sql/get_tasks.sql"), "l")
	};
	
	{
		let mut client = db_pool.get().await?;
		let stmt = client.prepare_typed_named(&name, &_stmt, &[Type::VARCHAR, Type::VARCHAR, Type::OID]).await.unwrap();
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
	}
		.iter()
		.map(Task::try_from)
		.collect()
}

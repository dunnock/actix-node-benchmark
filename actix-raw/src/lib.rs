use serde::Serialize;

#[derive(Serialize)]
pub struct Task {
    id: i32,
    summary: String,
    description: Option<String>,
    assignee_id: i32,
    assignee_name: String,
}

mod pg_connection;
pub use pg_connection::PgConnection;

mod get_task;
pub use get_task::GetTask;

mod get_tasks;
pub use get_tasks::GetTasks;

mod create_tasks;
pub use create_tasks::CreateTasks;

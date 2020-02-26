use actix_web::{get, web, Error, HttpResponse};
use crate::{errors::BenchError, db};
use deadpool_postgres::{Client, Pool};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetTasksQuery {
    pub summary: Option<String>,
    pub assignee_name: Option<String>,
    pub limit: Option<u32>,
    pub full: Option<bool>
}

/// Get tasks data matching criteria:
/// - assignee_name LIKE :assignee_name
/// - summary LIKE :summary
/// - LIMIT :limit
/// - full=true will return task.description
#[get("/tasks")]
pub async fn get_tasks(
    query: web::Query<GetTasksQuery>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client: Client =
        db_pool.get().await.map_err(BenchError::PoolError)?;

    let tasks = db::get_tasks(&client, query.into_inner()).await?;

    Ok(HttpResponse::Ok().json(tasks))
}
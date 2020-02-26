use serde::Serialize;
use tokio_pg_mapper::PostgresMapper;
use tokio_postgres::row::Row;
use crate::errors::BenchError;

#[derive(Serialize, PostgresMapper)]
#[pg_mapper(table = "task")]
pub struct Task {
    pub id: i32,
    pub summary: String,
    pub description: Option<String>,
    pub assignee_id: i32,
    pub assignee_name: String,
}


impl std::convert::TryFrom<&Row> for Task {
    type Error = BenchError;
    fn try_from(row: &Row) -> Result<Self, BenchError> {
        Ok(Self {
            id: row.try_get(0)?,
            summary: row.try_get(1)?,
            assignee_id: row.try_get(2)?,
            assignee_name: row.try_get(3)?,
            description: row.try_get(4)?
        })
    }
}
use std::io;

use futures::FutureExt;

use super::{PgConnection, Task};
use actix::{Handler, Message, ResponseFuture};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetTasks {
    summary: Option<String>,
    assignee_name: Option<String>,
    limit: Option<u32>
}

impl Message for GetTasks {
    type Result = io::Result<Vec<Task>>;
}

impl Handler<GetTasks> for PgConnection {
    type Result = ResponseFuture<Result<Vec<Task>, io::Error>>;

    fn handle(
        &mut self, GetTasks { summary, assignee_name, limit }: GetTasks, _: &mut Self::Context,
    ) -> Self::Result {
		let cl = self.client();
		let like = |s: Option<String>| s.map(|s| format!("%{}%", s));
        let query = async move {
            let assignee_name = like(assignee_name);
            let summary = like(summary);
            cl.conn.query(&cl.tasks, &[&assignee_name, &summary, &limit]).await
        };

        let get_tasks = query.map(|res| match res {
            Err(e) => Err(io::Error::new(io::ErrorKind::Other, format!("{:?}", e))),
            Ok(rows) => Ok(rows
                .iter()
                .map(|row| Task {
                    id: row.get(0),
                    summary: row.get(1),
                    assignee_id: row.get(2),
                    assignee_name: row.get(3),
                    description: None
                })
                .collect()),
        });
        Box::pin(get_tasks)
    }
}

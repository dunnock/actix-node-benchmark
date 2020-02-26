use std::io;

use futures::FutureExt;

use super::{PgConnection, Task};
use actix::{Handler, Message, ResponseFuture};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetTasks {
    summary: Option<String>,
    assignee_name: Option<String>,
    limit: Option<u32>,
    full: Option<bool>
}

impl Message for GetTasks {
    type Result = io::Result<Vec<Task>>;
}

impl Handler<GetTasks> for PgConnection {
    type Result = ResponseFuture<Result<Vec<Task>, io::Error>>;

    fn handle(
        &mut self, GetTasks { summary, assignee_name, limit, full }: GetTasks, _: &mut Self::Context,
    ) -> Self::Result {
        let cl = self.client();
        let full = full.is_some() && full.unwrap();
        let like = |s: Option<String>| s.map(|s| format!("%{}%", s));
        let query = async move {
            let assignee_name = like(assignee_name);
            let summary = like(summary);
            let limit = limit.unwrap_or(10);
            if full {
                cl.conn.query(&cl.tasks_full, &[&assignee_name, &summary, &limit]).await
            } else {
                cl.conn.query(&cl.tasks, &[&assignee_name, &summary, &limit]).await
            }
        };

        let get_tasks = query.map(move |res| match res {
            Err(e) => Err(io::Error::new(io::ErrorKind::Other, format!("{:?}", e))),
            Ok(rows) => Ok(rows
                .iter()
                .map(|row| Task {
                    id: row.get(0),
                    summary: row.get(1),
                    assignee_id: row.get(2),
                    assignee_name: row.get(3),
                    description: if full { Some(row.get(4)) } else { None }
                })
                .collect()),
        });
        Box::pin(get_tasks)
    }
}

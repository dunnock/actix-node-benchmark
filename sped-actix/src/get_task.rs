use std::io;

use futures::FutureExt;

use actix::{Handler, Message, ResponseFuture};
use super::{Task, PgConnection};


pub struct GetTask(pub u32);

impl Message for GetTask {
    type Result = io::Result<Task>;
}

impl Handler<GetTask> for PgConnection {
    type Result = ResponseFuture<Result<Task, io::Error>>;

    fn handle(&mut self, msg: GetTask, _: &mut Self::Context) -> Self::Result {
        let get_task = self
            .cl
            .query_one(&self.task, &[&msg.0])
            .map(|res| match res {
                Err(e) => Err(io::Error::new(io::ErrorKind::Other, format!("{:?}", e))),
                Ok(row) => Ok(Task {
                    id: row.get(0),
                    summary: row.get(1),
                    description: row.get(2),
                    assignee_id: row.get(3),
                    assignee_name: row.get(4),
                }),
            });
        Box::pin(get_task)
    }
}

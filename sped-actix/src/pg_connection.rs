use actix::{Actor, Addr, Context};
use std::io;
use tokio_postgres::{connect, Client, NoTls, Statement};

/// Postgres interface
pub struct PgConnection {
    pub(crate) cl: Client,
    pub(crate) task: Statement,
    pub(crate) tasks: Statement,
    pub(crate) tasks_name: Statement,
    pub(crate) tasks_summary: Statement,
    pub(crate) tasks_name_summary: Statement,
}

impl Actor for PgConnection {
    type Context = Context<Self>;
}

impl PgConnection {
    pub async fn connect(db_url: String) -> Result<Addr<PgConnection>, io::Error> {
        let (cl, conn) = connect(db_url.as_str(), NoTls)
            .await
            .expect("can not connect to postgresql");
        actix_rt::spawn(async move {
            if let Err(e) = conn.await {
                eprintln!("connection error: {}", e);
            }
        });

        let query = |q: &str| {
            format!("SELECT tasks.id, tasks.summary, tasks.description, assignee.id, assignee.name FROM tasks INNER JOIN workers as assignee ON assignee.id = tasks.assignee_id {}", q)
        };

        let task = cl.prepare(&query("WHERE tasks.id = $1")).await.unwrap();

        let tasks = cl.prepare(&query("")).await.unwrap();
        let tasks_name = cl
            .prepare(&query("WHERE assignee.name LIKE '$1'"))
            .await
            .unwrap();
        let tasks_summary = cl.prepare(&query("WHERE summary LIKE '$1'")).await.unwrap();
        let tasks_name_summary = cl
            .prepare(&query(
                "WHERE assignee.name LIKE '$1' AND summary LIKE '$1'",
            ))
            .await
            .unwrap();

        Ok(PgConnection::create(move |_| PgConnection {
            cl,
            task,
            tasks,
            tasks_name,
            tasks_summary,
            tasks_name_summary,
        }))
    }
}

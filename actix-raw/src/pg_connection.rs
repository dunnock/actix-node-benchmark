use actix::{Actor, Addr, Context};
use tokio_postgres::{connect, Client, NoTls, Statement, types::Type};
use std::rc::Rc;

/// Postgres interface
pub struct PgConnection {
    clients: Vec<Rc<PreparedClient>>,
    current: usize,
    pool: usize
}

/// Client connection with preparted statements
pub struct PreparedClient {
    pub conn: Client,
    pub task: Statement,
    pub tasks: Statement,
    pub tasks_full: Statement
}

impl Actor for PgConnection {
    type Context = Context<Self>;
}

impl PgConnection {
    pub async fn connect(db_url: String, pool: usize) -> Result<Addr<PgConnection>, tokio_postgres::error::Error> {
        let mut clients = Vec::new();
        for _ in 0..pool {
            let (cl, conn) = connect(db_url.as_str(), NoTls).await?;
            actix_rt::spawn(async move {
                if let Err(e) = conn.await {
                    eprintln!("connection error: {}", e);
                }
            });
            clients.push(Rc::new(PreparedClient::init(cl).await?));
        }

        Ok(PgConnection::create(move |_| PgConnection { clients, current: 0, pool }))
    }

    pub fn client(&mut self) -> Rc<PreparedClient> {
        self.current = (self.current + 1) % self.pool;
        self.clients[self.current].clone()
    }
}

impl PreparedClient {
    pub async fn init(conn: Client) -> Result<Self, tokio_postgres::error::Error> {
        let task = conn.prepare(
            "SELECT task.id, task.summary, assignee.id, assignee.name 
            FROM task INNER JOIN worker as assignee ON assignee.id = task.assignee_id
            WHERE task.id = $1"
        ).await?;

        let tasks = conn.prepare_typed(
            "SELECT task.id, task.summary, assignee.id, assignee.name 
            FROM task INNER JOIN worker as assignee ON assignee.id = task.assignee_id
            WHERE ($1 is null or assignee.name LIKE $1) AND ($2 is null or summary LIKE $2) LIMIT $3",
            &[Type::VARCHAR, Type::VARCHAR, Type::OID]
        ).await?;

        let tasks_full = conn.prepare_typed(
            "SELECT task.id, task.summary, assignee.id, assignee.name, task.description
            FROM task INNER JOIN worker as assignee ON assignee.id = task.assignee_id
            WHERE ($1 is null or assignee.name LIKE $1) AND ($2 is null or summary LIKE $2) LIMIT $3",
            &[Type::VARCHAR, Type::VARCHAR, Type::OID]
        ).await?;

        Ok(Self {
            conn,
            task,
            tasks,
            tasks_full
        })
    }

    pub fn client(&self) -> Client {
        self.conn.clone()
    }
}

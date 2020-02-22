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
        let query = |q: &str| {
            format!("SELECT tasks.id, tasks.summary, assignee.id, assignee.name FROM tasks INNER JOIN workers as assignee ON assignee.id = tasks.assignee_id {}", q)
        };

        let task = conn.prepare(
			"SELECT tasks.id, tasks.summary, assignee.id, assignee.name 
			FROM tasks INNER JOIN workers as assignee ON assignee.id = tasks.assignee_id
			WHERE tasks.id = $1"
		).await?;

        let tasks = conn.prepare_typed(
			"SELECT tasks.id, tasks.summary, assignee.id, assignee.name 
			FROM tasks INNER JOIN workers as assignee ON assignee.id = tasks.assignee_id
			WHERE ($1 is null or assignee.name LIKE $1) or ($2 is null or summary LIKE $2) LIMIT $3",
			&[Type::VARCHAR, Type::VARCHAR, Type::OID]
		).await?;

        Ok(Self {
            conn,
            task,
            tasks,
        })
	}

	pub fn client(&self) -> Client {
		self.conn.clone()
	}

	pub fn task(&self) -> Statement {
		self.task.clone()
	}

	pub fn tasks(&self) -> Statement {
		self.tasks.clone()
	}
}

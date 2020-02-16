use std::io;

use futures::TryFutureExt;

use actix::{Handler, Message, ResponseFuture};
use serde::{Deserialize};
use super::{PgConnection};
use rand::{RngCore, FromEntropy};
use rand::rngs::SmallRng;
use tokio_postgres::{binary_copy::BinaryCopyInWriter, types::Type};
use futures::{pin_mut};
use lipsum::MarkovChain;

static FNAMES: [&str; 4946] = include!("texts/first-names.json");
static LNAMES: [&str; 21986] = include!("texts/names.json");
static DIALOGUES: &str = include!("texts/ring.txt");

/// Create N tasks assigned to M workers
/// Can be run only on empty DB
#[derive(Deserialize)]
pub struct CreateTasks {
    tasks: i32,
    workers: i32,
}

impl Message for CreateTasks {
    type Result = io::Result<usize>;
}

impl Handler<CreateTasks> for PgConnection {
    type Result = ResponseFuture<io::Result<usize>>;

    fn handle(
        &mut self, CreateTasks { tasks, workers }: CreateTasks, _: &mut Self::Context,
    ) -> Self::Result {
        let cl = self.cl.clone();

        let mut rng = SmallRng::from_entropy();
        let mut gen_text = MarkovChain::new_with_rng(SmallRng::from_entropy());
        gen_text.learn(DIALOGUES);

        let workers_data: Vec<Worker> = (1..=workers).map(|id| Worker::gen(id, &mut rng)).collect();
        let tasks_data: Vec<Task> = (1..=tasks).map(|id| Task::gen(id, &workers_data, &mut rng, &mut gen_text)).collect();

        let fut = async move {
            // Copy in workers
            let workers_sink = cl.copy_in("COPY workers (id, name, email) FROM STDIN BINARY").await?;
            let writer = BinaryCopyInWriter::new(workers_sink, &[Type::INT4, Type::VARCHAR, Type::VARCHAR]);
            pin_mut!(writer);
            for worker in workers_data {
                writer.as_mut().write(&[&worker.id, &worker.name, &worker.email]).await?
            }
            writer.finish().await?;

            // Copy in tasks
            let tasks_sink = cl.copy_in("COPY tasks (id, summary, description, assignee_id) FROM STDIN BINARY").await?;
            let writer = BinaryCopyInWriter::new(tasks_sink, &[Type::INT4, Type::VARCHAR, Type::VARCHAR, Type::INT4]);
            pin_mut!(writer);
            for task in tasks_data {
                writer.as_mut().write(&[&task.id, &task.summary, &task.description, &task.assignee_id]).await?
            }
            writer.finish().await?;

            Ok(tasks as usize)
        };

        Box::pin(fut.map_err(|err: tokio_postgres::error::Error| { dbg!(&err); io::Error::new(io::ErrorKind::Other, format!("{:?}", err)) } ))
    }
}

// CREATE TABLE workers (
// 	id SERIAL PRIMARY KEY,
// 	name varchar(255) NOT NULL,
// 	email varchar(255) NULL,
// 	score integer DEFAULT 0
// );
struct Worker {
    id: i32,
    name: String,
    email: String
}

impl Worker {
    pub fn gen(id: i32, rng: &mut impl RngCore) -> Self {
        let mut name: String = FNAMES[(rng.next_u32() as usize % 100) * 10].to_string();
        name.push(' ');
        name.push_str(LNAMES[(rng.next_u32() as usize % 100) * 10]);
        let mut email = name.replace(" ", ".");
        email.push_str("@gmail.com");
        Worker {
            id,
            name,
            email
        }
    }
}

// CREATE TABLE tasks (
// 	id SERIAL PRIMARY KEY,
// 	summary varchar(255) NOT NULL,
// 	description text NOT NULL,
// 	assignee_id integer NULL REFERENCES workers,
// 	created TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
// );
struct Task {
    id: i32,
    summary: String,
    description: String,
    assignee_id: i32
}

impl Task {
    pub fn gen<R: RngCore>(id: i32, workers: &Vec<Worker>, rng: &mut impl RngCore, gen_text: &mut MarkovChain<R>) -> Self {
        let assignee = &workers[rng.next_u32() as usize % workers.len()];
        let assignee_id = assignee.id;
        let mut summary: String = assignee.name.clone();
        summary.push(' ');
        summary.push_str(gen_text.generate(rng.next_u32() as usize % 10).as_str());
        let mut description = summary.clone();
        description.push(' ');
        description.push_str(gen_text.generate(rng.next_u32() as usize % 1000).as_str());
        Task {
            id,
            summary,
            description,
            assignee_id
        }
    }
}

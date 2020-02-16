use actix::Addr;
use actix_web::{get, post, web, App, Error, HttpServer, Responder};
use sped_actix::{CreateTasks, GetTask, GetTasks, PgConnection};
use std::time::Duration;
use tokio::time::delay_for;
use tokio_postgres::NoTls;

/// Get task data by id
/// Return 404 if no task found
#[get("/tasks/{id}")]
async fn get_task(params: web::Path<i32>, db: web::Data<Addr<PgConnection>>) -> impl Responder {
    db.send(GetTask(params.into_inner()))
        .await?
        .map(web::Json)
        .map_err(Error::from)
}

/// Get tasks data matching criteria:
/// - assignee_name LIKE ..
/// - summary LIKE ..
#[get("/tasks")]
async fn filter_tasks(
    web::Query(get_tasks): web::Query<GetTasks>,
    db: web::Data<Addr<PgConnection>>,
) -> impl Responder {
    db.send(get_tasks)
        .await?
        .map(web::Json)
        .map_err(Error::from)
}

/// Fill database with random records
/// POST /actions/filldb?tasks=<N>&workers=<M>
#[post("/actions/filldb")]
async fn filldb(
    web::Query(create_tasks): web::Query<CreateTasks>,
    db: web::Data<Addr<PgConnection>>,
) -> impl Responder {
    db.send(create_tasks)
        .await?
        .map(web::Json)
        .map_err(Error::from)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let db_host = std::env::var("DB_HOST").unwrap_or("localhost".to_owned());
    let db_url = format!("postgres://sped:sped@{}:5432/sped", db_host);
    let workers: usize = std::env::var("WORKERS").unwrap_or("1".to_owned()).parse().unwrap();

    while let Err(err) = tokio_postgres::connect(db_url.as_str(), NoTls).await {
        println!("Failed connection to PostgreSQ {}", err);
        delay_for(Duration::from_millis(1_000)).await;
        println!("Retrying connection to PostgreSQL...");
    }

    HttpServer::new(move || {
        let db_url = db_url.clone();
        App::new()
            .data_factory(move || PgConnection::connect(db_url.clone()))
            .service(get_task)
            .service(filter_tasks)
            .service(filldb)
    })
    .bind("127.0.0.1:3001")?
    .workers(workers)
    .run()
    .await
}

use actix_web::{get, web, App, HttpServer, Responder, Error};
use sped_actix::{PgConnection, GetTask, GetTasks};
use actix::{Addr};
use tokio::time::delay_for;
use std::time::Duration;
use tokio_postgres::{NoTls};

#[get("/tasks/{id}")]
async fn get_task(params: web::Path<u32>, db: web::Data<Addr<PgConnection>>) -> impl Responder {
    db.send(GetTask(params.into_inner())).await?.map(web::Json).map_err(Error::from)
}

#[get("/tasks")]
async fn filter_tasks(web::Query(get_tasks): web::Query<GetTasks>, db: web::Data<Addr<PgConnection>>) -> impl Responder {
    db.send(get_tasks).await?.map(web::Json).map_err(Error::from)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let db_host = std::env::var("DB_HOST").unwrap_or("localhost".to_owned());
    let db_url = format!("postgres://sped:sped@{}:5432/sped", db_host);

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
    })
    .bind("127.0.0.1:3001")?
    .run()
    .await
}

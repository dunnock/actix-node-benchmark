use actix_web::{App, HttpServer};
use dotenv::dotenv;
use tokio_postgres::NoTls;
use actix_bench::config::Config;
use actix_bench::routes;


/// Main test server, configurable via env variables:
/// DB_HOST - host name of PostgreSQL DB
/// WORKERS - number of workers (busy CPU cores)
/// POOL_SIZE - number of DB connections per worker (busy Postgres cores)
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let config = Config::from_env().unwrap();
    let pool = config.pg.create_pool(NoTls).unwrap();

    println!("Server available at http://127.0.0.1:3002/");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(routes::get_tasks)
    })
    .bind("127.0.0.1:3002")?
    .workers(config.workers)
    .run()
    .await

}

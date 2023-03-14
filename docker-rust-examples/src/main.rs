mod logging;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use deadpool_postgres::Runtime;
use postgres::{Client, NoTls};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/db")]
async fn db_status() -> impl Responder {
    tracing::debug!("in db status");

    let mut config = deadpool_postgres::Config::new();

    config.dbname = Some("testdb".into());
    config.user = Some("postgres".into());
    config.password = Some("postgres".into());
    config.port = Some(5432);
    config.host = Some(std::env::var("PG_HOST").unwrap_or_else(|_| "localhost".into()));

    let pool = config.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();

    let client = pool.get().await.unwrap();

    let status = client.query("select 'ok';", &[]).await.unwrap()[0].try_get::<usize, String>(0).unwrap_or_else(|_| "failed".into());

    HttpResponse::Ok().body(status)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    logging::setup_logger();

    HttpServer::new(|| {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .service(hello)
            .service(echo)
            .service(db_status)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

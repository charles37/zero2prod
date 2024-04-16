use crate::routes::{health_check, subscribe};
use actix_web::{web, App, HttpServer};
use actix_web::dev::Server;
use std::net::TcpListener;
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;

pub fn run(
    listener: TcpListener,
    db_pool: PgPool
) -> Result<Server, std::io::Error> {
    let connection = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}



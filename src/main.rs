
//! main.rs

use zero2prod::startup::run;
use zero2prod::configuration::get_configuration;
use std::net::TcpListener;
use sqlx::PgPool;

#[tokio::main]
async fn main() -> Result<(), std::io::Error>{
    //TcpListener::new("127.0.0.1");
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection = PgPool::connect(
            &configuration.database.connection_string()
        )
        .await
        .expect("Failed to connect to Postgres");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    let port = listener.local_addr().unwrap();
    println!("Listening on http://127.0.0.1:{}", port);
    run(listener, connection)?.await
}

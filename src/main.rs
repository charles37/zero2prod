
//! main.rs

use zero2prod::startup::run;
use zero2prod::configuration::get_configuration;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error>{
    //TcpListener::new("127.0.0.1");
    let configuration = get_configuration().expect("Failed to read configuration");
    let listener = TcpListener::bind(format!("127.0.0.1:{}", configuration.application_port))
        .expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap();
    println!("Listening on http://127.0.0.1:{}", port);
    run(listener)?.await
}


use std::net::TcpListener;
use sqlx::{PgConnection, PgPool, Connection, Executor};
use zero2prod::configuration::{DatabaseSettings, get_configuration};
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};
use uuid::Uuid;
use once_cell::sync::Lazy;
use secrecy::ExposeSecret;

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name : String = "test".into();
    if std::env::var("TEST_LOG").is_ok(){
        let subscriber = get_subscriber(
                subscriber_name,
                default_filter_level,
                std::io::stdout
            );
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(
                subscriber_name,
                default_filter_level,
                std::io::sink
            );
        init_subscriber(subscriber);
    };
});

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool
}

async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let mut configuration = get_configuration()
        .expect("failed to get configuration");
    configuration.database.database_name = Uuid::new_v4().to_string();

    let connection_pool = configure_database(&configuration.database).await;

    let server = run(listener, connection_pool.clone()).expect("Faield to bind address");
    let _ = tokio::spawn(server);

    TestApp {
        address,
        db_pool: connection_pool,
    }

}


pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect(
            &config.connection_string_without_db().expose_secret()
        )
        .await
        .expect("Failed to connect to postgres");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("failed to create database");

    let connection_pool = PgPool::connect(
            &config.connection_string().expose_secret()
        )
        .await
        .expect("failed to connect to postgres");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}




#[tokio::test]
async fn health_check_works() {
    let app = spawn_app().await;
    println!("{}", app.address);

    let client = reqwest::Client::new();
    println!("{:?}", client);

    let response = client
        .get(&format!("{}/health_check", app.address))
        .send()
        .await
        .expect("Failed to execute request");

    println!("{:?}", response);

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());

}

#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data(){
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status().as_u16(), 200);

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscriptions");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}


#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
// Arrange

    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email")
    ];
    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(&format!("{}/subscriptions", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");
        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            // Additional customised error message on test failure
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}




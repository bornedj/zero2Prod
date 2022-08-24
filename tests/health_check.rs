use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

async fn spawn_app() -> TestApp {
    // address setup
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port.");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{port}");

    // db setup
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    // App/server setup
    let server =
        zero2prod::startup::run(listener, connection_pool.clone()).expect("Failed to bind address");
    // launcing the server as a background task
    // using tokio spawn to return a handle of a future
    let _ = tokio::spawn(server);

    //return the address with the randomly assigned port to be used in the response check
    TestApp {
        address,
        db_pool: connection_pool,
    }
}

#[tokio::test]
async fn health_check_works() {
    let app = spawn_app().await;

    // Bringing in reqwest to perform HTTP requests against our application
    let client = reqwest::Client::new();

    let response = client
        // Use the returned application address
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data() {
    // setup
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // act
    let body = "name=daniel%20borne&email=danielborne%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert response is correct
    assert_eq!(200, response.status().as_u16());
    // Assert database record written
    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscriptions");

    assert_eq!(saved.email, "danielborne@gmail.com");
    assert_eq!(saved.name, "daniel borne");
}

#[tokio::test]
async fn subscribe_returns_400_when_data_is_missing() {
    // setup
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    // iterate through the tupes in test cases
    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        // Assert response correct
        assert_eq!(
            400,
            response.status().as_u16(),
            //customized error message on test failure
            "The API did not fail with 400 on Request when payload was {}",
            error_message
        );
    }
}

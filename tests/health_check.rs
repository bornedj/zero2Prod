use std::net::TcpListener;

#[tokio::test]
async fn heaht_check_works() {
    let address = spawn_app();

    // Bringing in reqwest to perform HTTP requests against our application
    let client = reqwest::Client::new();

    let response = client
        // Use the returned application address
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port.");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address");
    // launcing the server as a background task
    // using tokio spawn to return a handle of a future
    let _ = tokio::spawn(server);

    //return the address with the randomly assigned port to be used in the response check
    format!("http://127.0.0.1:{port}")
}

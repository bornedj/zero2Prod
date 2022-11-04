use crate::utils::spawn_app;
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

#[tokio::test]
async fn subscribe_returns_400_when_fields_are_present_but_empty() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=&email=test_email%40test.com", "empty name"),
        ("name=test_name&email=", "empty email"),
        ("name=test_name&email=definitely-not-email", "invalid email"),
    ];

    for (body, description) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not return a 400 bad request when payload was {}.",
            description
        )
    }
}

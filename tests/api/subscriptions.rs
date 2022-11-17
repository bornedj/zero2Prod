use crate::utils::{spawn_app, ConfirmationLinks};
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};
#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data() {
    // setup
    let app = spawn_app().await;
    let body = "name=daniel%20borne&email=danielborne%40gmail.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    // act
    let response = app.post_subscriptions(body.into()).await;
    // Assert response is correct
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_persists_the_new_suibscriber() {
    // arrange
    let app = spawn_app().await;
    let body = "name=daniel%20borne&email=danielborne%40gmail.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    // act
    app.post_subscriptions(body.into()).await;

    // Assert database record written
    let saved = sqlx::query!("SELECT email, name, status FROM subscriptions")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscriptions");

    assert_eq!(saved.email, "danielborne@gmail.com");
    assert_eq!(saved.name, "daniel borne");
    assert_eq!(saved.status, "pending_confirmation");
}

#[tokio::test]
async fn subscribe_returns_400_when_data_is_missing() {
    // setup
    let app = spawn_app().await;
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    // iterate through the tupes in test cases
    for (invalid_body, error_message) in test_cases {
        let response = app.post_subscriptions(invalid_body.into()).await;
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
    let test_cases = vec![
        ("name=&email=test_email%40test.com", "empty name"),
        ("name=test_name&email=", "empty email"),
        ("name=test_name&email=definitely-not-email", "invalid email"),
    ];

    for (body, description) in test_cases {
        let response = app.post_subscriptions(body.into()).await;
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not return a 400 bad request when payload was {}.",
            description
        )
    }
}

#[tokio::test]
async fn subscribe_sends_a_confirmation_email_for_valid_data() {
    // arrange
    let app = spawn_app().await;
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    // act
    app.post_subscriptions(body.into()).await;

    // mock asserts on drop
}

#[tokio::test]
async fn subscribe_sends_a_confirmation_email_with_link() {
    // arrange
    let app = spawn_app().await;
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    //act
    app.post_subscriptions(body.into()).await;

    //assert
    let email_request = &app.email_server.received_requests().await.unwrap()[0];
    let confirmation_links = app.get_confirmation_links(&email_request);

    assert_eq!(confirmation_links.html, confirmation_links.plain_text);
}

#[tokio::test]
async fn single_user_multi_subscription_sends_multiple_emails() {
    // arrange
    let app = spawn_app().await;
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    // setup mock email server
    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    // send first request
    let first_req = app.post_subscriptions(body.into()).await;
    // send second request
    let second_req = app.post_subscriptions(body.into()).await;

    // assert
    let email_requests = &app.email_server.received_requests().await.unwrap();
    dbg!(first_req, second_req);
    let confirmation_links: Vec<ConfirmationLinks> = email_requests
        .iter()
        .map(|req| {
            return app.get_confirmation_links(req);
        })
        .collect();

    // check that more than one request was received
    assert_eq!(confirmation_links.len(), 2);

    // assert that each emails confirmation links match
    assert_eq!(confirmation_links[0].html, confirmation_links[0].plain_text);
    assert_eq!(confirmation_links[1].html, confirmation_links[1].plain_text);

    // check that the separate emails have different links via tokens
    assert_ne!(
        confirmation_links[0].plain_text,
        confirmation_links[1].plain_text
    );
}

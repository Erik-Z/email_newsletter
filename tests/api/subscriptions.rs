use reqwest::Client;

use crate::helpers::{drop_database, spawn_app};

#[tokio::test]
async fn subscribe_valid_form_data_200() {
    let app = spawn_app().await;
    let body = "name=test&email=testemail%40gmail.com";

    let response = app.post_subscriptions(body.into()).await;
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "testemail@gmail.com");
    assert_eq!(saved.name, "test");

    drop_database(&app).await;
}

#[tokio::test]
async fn subscribe_data_missing_400() {
    let app = spawn_app().await;
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = app.post_subscriptions(invalid_body.into()).await;

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }

    drop_database(&app).await;
}

#[tokio::test]
async fn subscribe_data_empty_400() {
    let app = spawn_app().await;
    let test_cases = vec![
        ("name=&email=test%40gmail.com", "empty name"),
        ("name=test&email=", "empty email"),
        ("name=test&email=definitely-not-an-email", "invalid email"),
    ];

    for (body, description) in test_cases {
        let response = app.post_subscriptions(body.into()).await;

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not return a 400 Bad Request when the payload was {}.",
            description
        );
    }

    drop_database(&app).await;
}

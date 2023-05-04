use crate::helpers::{drop_database, spawn_app};

#[tokio::test]
async fn health_check_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());

    drop_database(&app).await;
}

use actix_web::{web, HttpResponse};
use sqlx::types::Uuid as SqlxUuid;
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct Parameters {
    subscription_token: String,
}

#[tracing::instrument(name = "Confirm a pending subscriber", skip(parameters, pool))]
pub async fn confirm(parameters: web::Query<Parameters>, pool: web::Data<PgPool>) -> HttpResponse {
    let id = match get_subscriber_id_from_token(&pool, &parameters.subscription_token).await {
        Ok(id) => id,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    match id {
        None => HttpResponse::Unauthorized().finish(),
        Some(subscriber_id) => {
            let status = get_subscriber_status(&pool, subscriber_id).await;
            match status {
                Ok(Some(status)) if status == "confirmed" => {
                    HttpResponse::Ok().body("Email is already confirmed")
                }
                Ok(_) => {
                    if confirm_subscriber(&pool, subscriber_id).await.is_err() {
                        return HttpResponse::InternalServerError().finish();
                    }
                    HttpResponse::Ok().finish()
                }
                Err(_) => HttpResponse::InternalServerError().finish(),
            }
        }
    }
}

#[tracing::instrument(name = "Mark subscriber as confirmed", skip(subscriber_id, pool))]
pub async fn confirm_subscriber(pool: &PgPool, subscriber_id: SqlxUuid) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"UPDATE subscriptions SET status = 'confirmed' WHERE id = $1"#,
        subscriber_id,
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}

#[tracing::instrument(name = "Get subscriber_id from token", skip(subscription_token, pool))]
pub async fn get_subscriber_id_from_token(
    pool: &PgPool,
    subscription_token: &str,
) -> Result<Option<SqlxUuid>, sqlx::Error> {
    let result = sqlx::query!(
        r#"SELECT subscriber_id FROM subscription_tokens WHERE subscription_token = $1"#,
        subscription_token,
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(result.map(|r| r.subscriber_id))
}

#[tracing::instrument(name = "Get subscriber status", skip(subscriber_id, pool))]
pub async fn get_subscriber_status(
    pool: &PgPool,
    subscriber_id: SqlxUuid,
) -> Result<Option<String>, sqlx::Error> {
    let result = sqlx::query!(
        r#"SELECT status FROM subscriptions WHERE id = $1"#,
        subscriber_id,
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(result.map(|r| r.status))
}

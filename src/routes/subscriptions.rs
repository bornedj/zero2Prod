use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    // setup logging
    let request_id = Uuid::new_v4();
    log::info!(
        "Request id: '{}' - Adding '{}' '{}' as a new subscriber.",
        request_id,
        form.email,
        form.name
    );

    // run insert query
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => {
            log::info!(
                "request id {} - New subscriber details have been saved",
                request_id
            );
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            log::error!(
                "request id {} - Failed to execute query {:?}.",
                request_id,
                e
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}

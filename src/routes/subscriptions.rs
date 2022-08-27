use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use tracing::Instrument;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    // setup tracing
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!(
        "Adding a new subscriber.",
        %request_id,
        subscriber_email = %form.email,
        subscriber_name = %form.name,
    );
    // do not use enter in prod, will be changed
    let _request_span_guard = request_span.enter();
    // `_request_span_guard` is dropped at the end of subscribe
    // that is when we exit the span

    let query_span = tracing::info_span!("Saving a new subscriber details in the database.");

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
    .instrument(query_span) // using the query_span to enter the span when the future is resolved
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            tracing::error!("Failed to execute query {:?}.", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

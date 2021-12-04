use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct HolderData {
    token_name: String, // TODO: will still need to make a separate table
    contract_address: String, // TODO: will still need to make a separate table
    holder_address: String,
    place: u16, // TODO: will still need to make a separate table
    amount: f64, // TODO: will still need to make a separate table
    timestamp: i64, // TODO: will be from Utc::now().timestamp() from a time passed in by the front end.
}

pub async fn add_holder(_form: web::Form<HolderData>, pool: web::Data<PgPool>) -> HttpResponse {
    sqlx::query!(
        r#"
        INSERT INTO holders (id, holder_address) VALUES ($1, $2)
        "#,
        Uuid:: new_v4(),
        _form.holder_address,
    )
        .execute(pool.get_ref())
        .await;
    HttpResponse::Ok().finish()
}

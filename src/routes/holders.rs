use actix_web::{web, HttpResponse};
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct HolderData {
    network: String,
    token_name: String,       // TODO: will still need to make a separate table
    contract_address: String, // TODO: will still need to make a separate table
    holder_address: String,
    place: u16,     // TODO: will still need to make a separate table
    amount: f64,    // TODO: will still need to make a separate table
    timestamp: i64, // TODO: will be from Utc::now().timestamp() from a time passed in by the front end.
}

pub async fn add_holder(_form: web::Form<HolderData>, pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query!(
        r#"
        WITH par_key AS (INSERT INTO networks (network_name) VALUES ($1) RETURNING network_id)
        INSERT INTO addresses (network_id, address)
        VALUES ((select par_key.network_id from par_key), $2)
        "#,
        _form.network,
        _form.contract_address
    )
        .execute(pool.get_ref())
        .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }

    // match sqlx::query!(
    //     r#"
    //     INSERT INTO addresses (holder_address) VALUES ($1)
    //     "#,
    //     _form.holder_address,
    // )
    // .execute(pool.get_ref())
    // .await
    // {
    //     Ok(_) => HttpResponse::Ok().finish(),
    //     Err(e) => {
    //         println!("Failed to execute query: {}", e);
    //         HttpResponse::InternalServerError().finish()
    //     }
    // }
}

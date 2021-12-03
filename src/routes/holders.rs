use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct HolderData {
    token_name: String,
    contract_address: String,
    holder_address: String,
    place: u16,
    amount: f64,
    timestamp: u64
}

pub async fn add_holder(_form: web::Form<HolderData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

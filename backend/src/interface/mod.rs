use actix_web::{http::StatusCode, web::{Data, Json}, HttpResponse, Responder};
use sqlx::PgPool;

use crate::model::user::User;

/// The card module contains the endpoints for the card API.
pub mod card;

/// The user module contains the endpoints for the user API.
pub mod user;

/// The deck module contains the endpoints for the deck API.
pub mod deck;

// -> the following endpoints are merely used for testing,
//    development and debugging purposes, and will not be 
//    included in the final release of the backend.

#[derive(serde::Deserialize)]
pub struct PostEntryBody {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[actix_web::post("/user")]
pub async fn post_entry(pool: Data<PgPool>, body: Json<PostEntryBody>) -> impl Responder {
    let uuid = uuid::Uuid::new_v4().to_string();


    match sqlx::query_as::<_, User>("INSERT INTO users (uuid, username, email, password) VALUES ($1, $2, $3, $4) RETURNING uuid, username, email, password, decks")
        .bind(uuid)
        .bind(body.username.as_str())
        .bind(body.email.as_str())
        .bind(body.password.as_str())
        .fetch_one(pool.as_ref())
        .await {
            Ok(ok) => HttpResponse::Created().json(ok),
            Err(e) => {
                eprintln!("{:?}", e);
                HttpResponse::InternalServerError().finish()
            }
        }
}

#[actix_web::get("/user/{id}")]
pub async fn get_entry() -> impl Responder {
    Json("{ \"message\": \"not yet implemented\" }").customize()
        .with_status(StatusCode::NOT_IMPLEMENTED)
}

#[actix_web::get("/users")]
pub async fn get_entries(pool: Data<PgPool>) -> impl Responder {
    let entries = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(pool.as_ref())
        .await;

    match entries {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

use serde::Serialize;
use sqlx::prelude::FromRow;

use super::deck::DeckUUID;

/// The `User` struct represents a user of
/// the application.
#[derive(Debug, Clone, Serialize, FromRow)]
pub struct User {
    /// Unique Identifier by `uuid` crate.
    pub uuid: String,

    /// The username of the user.
    pub username: String,

    /// The email of the user.
    pub email: String,

    /// The password of the user. (hashed)
    pub password: String,

    /// The collection of decks that the user
    /// has created and therefore has access to.
    pub decks: Vec<DeckUUID>,
}


/// The `UserUUID` type is a unique identifier for a `User`.
/// Note: This is and will be a `String` type.
pub type UserUUID = String;

use serde::Serialize;
use sqlx::prelude::FromRow;

use super::{card::CardUUID, user::UserUUID};

/// The `Deck` struct represents a collection of
/// flashcards which can be accessed together.
#[derive(Debug, Clone, Serialize, FromRow)]
pub struct Deck {
    /// Unique Identifier by `uuid` crate.
    pub uuid: String,

    /// The name or title of the deck.
    pub name: String,

    /// The description of the deck.
    pub description: String,

    /// The collection of cards in the deck.
    pub cards: Vec<CardUUID>,

    /// The user who created the deck and therefore
    /// has access to it.
    pub user: UserUUID
}

/// The `DeckUUID` type is a unique identifier for a `Deck`.
/// Note: This is and will be a `String` type.
pub type DeckUUID = String;

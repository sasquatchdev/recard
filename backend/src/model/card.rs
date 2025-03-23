use super::deck::DeckUUID;

/// The `Card` struct represents a single flashcard.
pub struct Card {
    /// Unique Identifier by `uuid` crate.
    pub uuid: String,

    /// The front or "question" side of the card.
    pub front: String,

    /// The back or "answer" side of the card.
    pub back: String,

    /// The deck that the card belongs to.
    pub deck: DeckUUID,
}

/// The `CardUUID` type is a unique identifier for a `Card`.
/// Note: This is and will be a `String` type.
pub type CardUUID = String;

/// The model module contains data (specifically types) that
/// are used to represent and parse generic data from the 
/// `database` module.
pub mod model;

/// The interface module contains the actual API endpoints and
/// *interfaces* that the frontend will use to interact with the
/// backend.
pub mod interface;

/// The database module contains the actual database connection
/// logic and implementation, along with helper functions to
/// interact with the database.
pub mod database;

fn main() {
    println!("Hello, world!");
}

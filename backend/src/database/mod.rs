use sqlx::postgres::PgPoolOptions;

/// Tries to establish a connection to the database.
/// Returns a `Result` with the `PgPool` if successful,
/// or an `sqlx::Error` if not.
pub async fn try_establish() -> Result<sqlx::PgPool, sqlx::Error> {
    // Can be set in a .env file or
    // as a regular environment variable.
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await;

    pool
}

/// Since most applications will panic if the database
/// connection fails, this function is a simple wrapper
/// around `try_establish` that panics if the connection
/// fails.
pub async fn establish() -> sqlx::PgPool {
    try_establish()
        .await
        .expect("Failed to establish a connection to the database.")
}

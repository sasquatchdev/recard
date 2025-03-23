use actix_web::{middleware::Logger, web::Data, HttpServer};
use dotenvy::dotenv;

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

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();

    // Safe due to single-threaded nature of actix-web.
    unsafe {
        std::env::set_var("RUST_LOG", "debug");
        std::env::set_var("RUST_BACKTRACE", "1");
    }
    env_logger::init();

    let pool = database::establish().await;

    HttpServer::new(move || {
        let logger = Logger::default();

        actix_web::App::new()
            .wrap(logger)
            .app_data(Data::new(pool.clone()))
            .service(interface::post_entry)
            .service(interface::get_entry)
            .service(interface::get_entries)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

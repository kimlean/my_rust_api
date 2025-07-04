mod config;
mod db;
mod routes;
mod models;

use axum::{Router, routing::get, routing::post};
use config::Settings;
use db::connect;
use routes::user::{create_user, get_user, update_user, delete_user, list_user};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Load configuration
    let settings = Settings::load();

    // Connect to the database
    let pool = connect(&settings.database_url).await.expect("Failed to connect to database");

    // Build the application router
    let app = Router::new()
        .route("/users", post(create_user).get(list_user))
        .route("/users/:id", get(get_user).put(update_user).delete(delete_user))
        .with_state(pool);

    // Define the address to run the server on
    let addr = SocketAddr::from(([127, 0, 0, 1], settings.port));
    
    // Start the server
    println!("Listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Server failed");
}
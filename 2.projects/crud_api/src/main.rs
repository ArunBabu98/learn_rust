/*
HTTP REST API — CRUD API for notes or tasks using axum or warp.
Skills: async, DB, modular structure, error handling

--- Notes ---
1. Add note
2. Edit note
3. delete note
4. show notes

-struct-

1. note title
2. note creation date
3. note -full
4. note update date

*/

mod entities;
mod handlers;
mod routes;

use tracing_subscriber;

use dotenvy::dotenv;
use std::env;
use std::sync::Arc;

use sea_orm::Database;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db = match Database::connect(&db_url).await {
        Ok(result) => {
            tracing::info!("Database connected!");
            Arc::new(result)
        }
        Err(err) => {
            tracing::error!("Database connection error: {err}");
            return; // or panic!, or Err(err.into()), depending on your function type
        }
    };

    let app = routes::create_routes(db.clone());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    tracing::info!("🚀 Server is running on http://localhost:5000");
    axum::serve(listener, app).await.unwrap();
}

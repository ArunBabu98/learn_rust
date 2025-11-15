use std::sync::Arc;

use axum::{Router, routing::get};
use sea_orm::DatabaseConnection;

use crate::handlers::root_handlers;

pub fn create_routes(db: Arc<DatabaseConnection>) -> Router {
    Router::new()
        .route("/", get(root_handlers::root))
        .route("/ping", get(root_handlers::ping))
        .route("/notes", get(root_handlers::show))
        .route("/add", get(root_handlers::add))
        .route("/edit", get(root_handlers::edit))
        .route("/delete", get(root_handlers::delete))
        .with_state(db)
}

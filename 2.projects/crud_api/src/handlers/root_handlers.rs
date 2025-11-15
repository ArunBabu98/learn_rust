use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait, sqlx::types::chrono::Utc,
};

use crate::entities::notes;
use std::sync::Arc;

pub async fn root() -> &'static str {
    "Hello World!"
}

pub async fn ping() -> impl IntoResponse {
    "pong"
}

pub async fn show(State(db): State<Arc<DatabaseConnection>>) -> impl IntoResponse {
    let notes_list = match notes::Entity::find().all(&*db).await {
        Ok(result) => result,
        Err(err) => {
            tracing::error!("Error fetching notes: {err}");
            Vec::new()
        }
    };

    tracing::info!("Notes: {:?}", notes_list);

    Json(notes_list)
}

#[derive(serde::Deserialize)]
pub struct NoteInput {
    pub title: String,
    pub content: String,
}

pub async fn add(
    State(db): State<Arc<DatabaseConnection>>,
    Json(payload): Json<NoteInput>,
) -> impl IntoResponse {
    let now = Utc::now().fixed_offset();
    let new_note = notes::ActiveModel {
        title: Set(payload.title),
        content: Set(payload.content),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };

    match new_note.insert(&*db).await {
        Ok(note) => {
            tracing::info!("Added note: {:?}", note);
            (StatusCode::CREATED, Json(note)).into_response()
        }
        Err(err) => {
            tracing::error!("Error inserting note: {:?}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Failed to add note"})),
            )
                .into_response()
        }
    }
}

#[derive(serde::Deserialize)]
pub struct UpdateNoteInput {
    id: i32,
    title: Option<String>,
    content: Option<String>,
}

pub async fn edit(
    State(db): State<Arc<DatabaseConnection>>,
    Json(payload): Json<UpdateNoteInput>,
) -> impl IntoResponse {
    let now = Utc::now().fixed_offset();
    let note_to_edit = match notes::Entity::find_by_id(payload.id).one(&*db).await {
        Ok(Some(note)) => note,
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({"error": "Note not found"})),
            )
                .into_response();
        }
        Err(err) => {
            tracing::error!("Error finding note: {err}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Failed to find note"})),
            )
                .into_response();
        }
    };
    let mut note: notes::ActiveModel = note_to_edit.into();

    if let Some(title) = payload.title {
        note.title = Set(title);
    }

    if let Some(content) = payload.content {
        note.content = Set(content);
    }

    note.updated_at = Set(now);

    match note.update(&*db).await {
        Ok(updated_note) => (StatusCode::OK, Json(updated_note)).into_response(),
        Err(err) => {
            tracing::error!("Error: {err};");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error":"Failed to update note"})),
            )
                .into_response()
        }
    }
}

#[derive(serde::Deserialize)]
pub struct DeleteNoteInput {
    pub id: i32,
}

pub async fn delete(
    State(db): State<Arc<DatabaseConnection>>,
    Json(payload): Json<DeleteNoteInput>,
) -> impl IntoResponse {
    match notes::Entity::delete_by_id(payload.id).exec(&*db).await {
        Ok(result) => {
            if result.rows_affected == 0 {
                (
                    StatusCode::NOT_FOUND,
                    Json(serde_json::json!({"error": "Note not found"})),
                )
                    .into_response()
            } else {
                tracing::info!("Deleted note with id: {}", payload.id);
                (
                    StatusCode::OK,
                    Json(serde_json::json!({"message": "Note deleted successfully", "rows_affected": result.rows_affected}))
                ).into_response()
            }
        }
        Err(err) => {
            tracing::error!("Error deleting note: {err}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Failed to delete note"})),
            )
                .into_response()
        }
    }
}

use axum::{extract::State, Json};
use sea_orm::EntityTrait;
use crate::state::AppState;
use crate::entities::keypack;

pub async fn list_keypacks(
    State(state): State<AppState>
) -> Json<Vec<keypack::Model>> {
    let keypacks = keypack::Entity::find()
        .all(&state.db)
        .await
        .unwrap_or_default();
    Json(keypacks)
}
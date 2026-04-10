use axum::{extract::State, Json};
use sea_orm::{EntityTrait, PaginatorTrait}; // 必须导入 PaginatorTrait 才能使用 count()
use crate::state::AppState;
use crate::entities::{user, device, keypack};
use serde::Serialize;

#[derive(Serialize)]
pub struct SystemStatus {
    pub user_count: u64,
    pub device_count: u64,
    pub keypack_count: u64,
}

pub async fn get_status(State(state): State<AppState>) -> Json<SystemStatus> {
    let user_count = user::Entity::find().count(&state.db).await.unwrap_or(0);
    let device_count = device::Entity::find().count(&state.db).await.unwrap_or(0);
    let keypack_count = keypack::Entity::find().count(&state.db).await.unwrap_or(0);

    Json(SystemStatus {
        user_count,
        device_count,
        keypack_count,
    })
}
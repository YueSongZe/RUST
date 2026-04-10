use axum::{extract::{State, Path}, Json, http::StatusCode};
use sea_orm::EntityTrait;
use crate::state::AppState;
use crate::entities::device;

// --- 1. 查看设备列表 ---
pub async fn list_devices(
    State(state): State<AppState>
) -> Json<Vec<device::Model>> {
    let devices = device::Entity::find()
        .all(&state.db)
        .await
        .unwrap_or_default();
    Json(devices)
}

// --- 2. 查看设备详情 ---
pub async fn get_device(
    State(state): State<AppState>,
    Path(id): Path<u32>, // 这里的 id 会自动对应路由里的 {id}
) -> Result<Json<device::Model>, StatusCode> {
    let device = device::Entity::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(device))
}
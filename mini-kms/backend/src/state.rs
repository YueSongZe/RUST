use sea_orm::DatabaseConnection;

// 核心思想：AppState 必须实现 Clone，因为 Axum 会为每个线程克隆一份状态
#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}
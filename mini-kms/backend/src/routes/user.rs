use axum::{extract::State, Json, http::StatusCode};
use sea_orm::{EntityTrait, ActiveModelTrait, ActiveValue};
use crate::state::AppState;
use chrono::Utc;
use crate::entities::user;
use bcrypt::{hash, DEFAULT_COST}; // 核心思想：永远不要在数据库存明文密码
use crate::entities::audit_log;
// --- 1. 获取用户列表 (GET) ---
// 核心思想：利用注入的 AppState 访问数据库连接
pub async fn list_users(
    State(state): State<AppState>
) -> Json<Vec<user::Model>> {
    let users = user::Entity::find()
        .all(&state.db)
        .await
        .unwrap_or_default();
    Json(users)
}

// --- 2. 数据传输对象 (DTO) ---
// 用于接收前端发送的 JSON 数据
#[derive(serde::Deserialize)]
pub struct CreateUserRequest {
    pub account: String,
    pub role: String,
    pub password: String,
}//前端传来的 JSON 会被自动反序列化为 Rust 结构体。如果字段不匹配，系统会自动拒绝，这在入口处就保证了数据的合规性。

// --- 3. 创建用户 (POST) ---      create_user 就是一个接口控制器，它定义了谁能来、传什么、回什么
// 实现逻辑：接收 JSON -> 密码哈希 -> 构造 ActiveModel -> 存入数据库
pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> (StatusCode, Json<user::Model>) {
    // A. 核心思想：将密码转化为不可逆的哈希值
    // 即使数据库泄露，黑客也无法直接看到用户密码
    let hashed_password = hash(payload.password, DEFAULT_COST)
        .expect("密码加密过程中发生致命错误");

    // B. 构造数据库操作模型 (ActiveModel)
    let new_user = user::ActiveModel {
    // 使用 .clone() 复制一份传进去，保留 payload.account 的所有权
    account: ActiveValue::Set(payload.account.clone()), //账号
    role: ActiveValue::Set(payload.role),//角色
    password_hash: ActiveValue::Set(hashed_password),//密码哈希
    created_at: ActiveValue::Set(chrono::Local::now().naive_local()),//创建时间
    ..Default::default()
    };

    // C. 执行数据库插入
    let saved_user = new_user.insert(&state.db)
        .await
        .expect("数据库写入失败，可能账号已存在");

    
    // D. 记录审计日志
    let log = audit_log::ActiveModel {
        user_id: ActiveValue::Set(0), // 演示暂设为0，实际应从 Token 提取
        action: ActiveValue::Set("CREATE_USER".to_string()),
        details: ActiveValue::Set(format!("Created user: {}", payload.account)),
        created_at: ActiveValue::Set(Utc::now().naive_utc()),
        ..Default::default()
    };
    let _ = log.insert(&state.db).await;

    (StatusCode::CREATED, Json(saved_user)) // 返回：201 Created 状态和新用户信息
}
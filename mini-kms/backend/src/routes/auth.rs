use axum::{extract::State, Json, http::StatusCode};
use crate::state::AppState;
use crate::entities::user;
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter}; 
use bcrypt::verify;
use serde::{Deserialize, Serialize};

// --- 数据传输对象 (DTO) ---
// 核心思想：强类型约束。
// 这里定义了前后端约定的“交互协议”，确保发进来的数据和传出去的数据格式严格对齐。
#[derive(Deserialize)]
pub struct LoginRequest {
    pub account: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

// 登录处理函数
// 逻辑链路：接收请求 -> 查询用户 -> 校验密码 -> 发放令牌
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    
    // 逻辑第一步：在数据库中寻找该账号。
    // 这里体现了 ORM 的查询思想：通过 Account 字段进行过滤，只取一条结果。
    let user = user::Entity::find()
        .filter(user::Column::Account.eq(payload.account))
        .one(&state.db)
        .await
        // 如果数据库查询出错，返回 500 服务器错误
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        // 如果账号不存在，返回 401 未授权（这里不区分“用户不存在”还是“密码错误”，是出于安全考虑，防止枚举账号）
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // 逻辑第二步：安全验证（系统的灵魂）。
    // 核心思想：永远不比对明文。
    // payload.password 是用户输入的明文，&user.password_hash 是数据库存的哈希值。
    // bcrypt::verify 会提取哈希值中的盐（Salt）重新计算，确保安全。
    if verify(payload.password, &user.password_hash).unwrap_or(false) {
        
        // 逻辑第三步：发放令牌（Token）。
        // 这里的实现是一个简易的身份标识。在真实场景下，这通常会是一个 JWT 字符串。
        // 我们在 Token 前面加上 "Bearer " 前缀，是为了符合 HTTP 协议的 Authorization 通用规范。
        let token = format!("Bearer token-{}", user.id); 
        Ok(Json(LoginResponse { token }))
    } else {
        // 密码比对失败，同样返回 401
        Err(StatusCode::UNAUTHORIZED)
    }
}
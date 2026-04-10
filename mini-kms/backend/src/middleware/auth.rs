use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
    http::{StatusCode, header},
};

// --- 鉴权中间件 ---
// 核心思想：拦截器模式（Interceptor）。
// 它是业务逻辑外的一层“套壳”，确保只有合法的请求才能触达数据库操作。
pub async fn auth_middleware(
    req: Request,
    next: Next,//Request 封装了整个 HTTP 请求，Next 决定了请求是否继续向下流转
) -> Result<Response, StatusCode> {
    
    // 逻辑第一步：从 HTTP 请求头中提取身份证明。
    // 我们约定前端必须在 Header 中包含 AUTHORIZATION 字段。
    let auth_header = req.headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok());

    // 逻辑第二步：令牌（Token）校验。
    // 这里体现了“最小化实现原则”：我们不检查数据库，而是检查令牌的格式。
    if let Some(token) = auth_header {
        // 核心判断：令牌是否以约定的固定前缀开始。
        // 在实际生产环境中，这里会替换为 JWT 的签名解析。
        if token.starts_with("Bearer token-") {
            
            // 逻辑第三步：放行（Pass）。
            // 如果校验通过，调用 next.run(req) 将请求交给下一个中间件或最终的业务函数。
            // 这是一个异步操作，体现了系统的非阻塞特性。
            return Ok(next.run(req).await); 
        }
    }

    // 逻辑第四步：拦截（Block）。
    // 如果没有任何合法的 Header，或者格式不对，直接在这里中断请求。
    // 返回 401 Unauthorized 状态码，请求永远不会到达后端的业务层。
    Err(StatusCode::UNAUTHORIZED) 
}
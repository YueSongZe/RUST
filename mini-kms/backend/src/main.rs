mod routes;
mod state;
mod entities;
mod middleware;

use crate::middleware::auth::auth_middleware;
use axum::{routing::{get, post}, Router};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use state::AppState;
use sea_orm::{Database, ConnectionTrait, Schema, Statement};

// 程序的异步入口。Rust 默认不处理异步任务，这里通过 tokio 开启多线程运行环境，
// 确保后端可以并发处理前端发送的多个接口请求。
#[tokio::main]
async fn main() {
    
    // 初始化数据库连接。使用 SQLite 是为了实现最小化运行环境，不需要配置复杂的数据库服务器。
    let database_url = "sqlite://minikms.db?mode=rwc"; 
    let db = Database::connect(database_url).await.expect("无法连接数据库");

    // --- 自动化建表逻辑 ---
    // 核心思想：利用 ORM 的 Schema 工具实现“代码驱动”。
    // 只要定义好了 Rust 的 Entity 结构，程序启动时就会自动同步数据库表结构，保证开发一致性。
    let builder = db.get_database_backend();
    let schema = Schema::new(builder);
    
    // 将所有定义的实体（用户、设备、密钥包、审计日志）转化为建表语句。
    let tables: Vec<Statement> = vec![
        builder.build(&schema.create_table_from_entity(entities::user::Entity)),
        builder.build(&schema.create_table_from_entity(entities::device::Entity)),
        builder.build(&schema.create_table_from_entity(entities::keypack::Entity)),
        builder.build(&schema.create_table_from_entity(entities::audit_log::Entity)),
    ];

    // 循环执行建表操作。这里使用了忽略错误的逻辑，因为如果表已存在，SQLite 会报错但我们不需要中断程序。
    for op in tables {
        let _ = db.execute(op).await;
    }
    println!("✅ 数据库初始化完成");

    // 将数据库连接注入 AppState。
    // 这是典型的状态管理思想，方便后续所有的路由处理函数都能共享同一个数据库连接池。
    let state = AppState { db };

    // --- 路由与中间件配置 ---
    let app = Router::new()
        // 登录接口属于公开路由，不需要鉴权即可访问。
        .route("/api/login", post(routes::auth::login))
        
        // 使用 nest 机制对业务接口进行分组管理。
        .nest("/api", Router::new()// 业务接口组,另一个分类
            .route("/users", post(routes::user::create_user))
            .route("/users", get(routes::user::list_users))
            .route("/devices", get(routes::device::list_devices))
            .route("/devices/{id}", get(routes::device::get_device))
            .route("/keypacks", get(routes::keypack::list_keypacks))
            .route("/status", get(routes::status::get_status))
            // 核心安全设计：在 /api 路径下挂载鉴权中间件。
            // 只有通过 Token 校验的请求才能进入该区域，实现了对核心业务的统一保护。
            .layer(axum::middleware::from_fn(auth_middleware)) //layer对这个分类进行拦截
        )
        // 配置 CORS 允许跨域请求。这是为了解决前后端分离开发中，浏览器同源策略导致的访问限制问题。
        .layer(CorsLayer::permissive())
        // 将全局状态 state 绑定到整个应用。
        .with_state(state);

    // 设置后端服务监听地址及端口。
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("\n🚀 Mini KMS Admin 后端已启动: http://{}", addr);

    // 启动 Web 服务。
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
//1.定义数据结构
// 输入：客户端发给我们的 JSON 格式
// Deserialize (反序列化): 意思是从 JSON 变成 Rust 结构体
#[derive(Deserialize)]
struct RequestData {
    number: f64,
}
// 输出：我们要发回给客户端的 JSON 格式
// Serialize (序列化): 意思是把 Rust 结构体变成 JSON 字符串
#[derive(Serialize)]
struct RespondData{
    original:f64,
    result:f64,
    msg:String,
}
//2.核心逻辑函数
async fn calc_square(Json(payload):Json<RequestData>)->Json<RespondData>{
    // 1. 提取数据 (Axum 已经帮我们把 JSON 解析好放在 payload 里了)
    let x=payload.number;
    //2.业务计算
    println!("计算{}的平方....",x);
    let square=x*x;
    //3. 构造返回 (构造一个结构体，Axum 会自动把它变成 JSON 发回去)
    let response=RespondData{
        original:x,
        result:square,
        msg:"success！".to_string(),
    };
    Json(response)
}
async fn hello()->& 'static str{
    "你好，我是Aumx计算平方服务器！"
}
//3.主函数启动服务器
#[tokio::main]
async fn main(){
    // A. 构建路由 (Router)
    // 类似于把电话分机号和人对应起来
    let app=Router::new()
        .route("/",get(hello)) //根路径调用hello
        .route("/calc_square",post(calc_square)); // /calc_square 路径调用 calc_square 函数
    // B. 绑定端口
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("服务器运行在：127.0.0.1:3000");
    // C. 启动服务器
    axum::serve(listener,app).await.unwrap();
    
}
use std::io::{self, Write};
use serde::{Deserialize, Serialize};

// 客户端也需要定义同样的结构体，以便打包数据
#[derive(Serialize)]
struct RequestData {
    number: f64,
}

#[derive(Deserialize, Debug)] // Debug 方便直接打印结果
struct ResponseData {
    original: f64,
    result: f64,
    msg: String,
}

fn main() {
    let client = reqwest::blocking::Client::new();
    println!("已连接到 Axum 服务器，请输入数字 (输入 quit 退出):");

    loop {
        // 1. 获取用户输入 (这部分跟以前一样)
        let mut input = String::new();
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "quit" { break; }

        // 尝试把字符串转成数字
        let number: f64 = match input.parse() {
            Ok(n) => n,
            Err(_) => {
                println!("⚠️ 请输入有效的数字！");
                continue;
            }
        };

        // 2. 构造 JSON 数据
        let body = RequestData { number };

        // 3. 发送 HTTP POST 请求
        // 就像你在浏览器里访问一样，但这次是发 POST
        let response = client.post("http://127.0.0.1:3000/calc_square")
            .json(&body) // 自动把结构体变成 JSON 塞进去
            .send();

        // 4. 处理返回结果
        match response {
            Ok(res) => {
                // 把服务器返回的 JSON 自动变回 Struct
                match res.json::<ResponseData>() {
                    Ok(data) => println!("✅ 服务器回复: {:.2} 的平方是 {:.2} ({})", data.original, data.result, data.msg),
                    Err(e) => println!("❌ 解析响应失败: {}", e),
                }
            },
            Err(e) => println!("❌ 请求失败: {}", e),
        }
    }
}
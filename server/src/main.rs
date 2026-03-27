use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt}; // 引入异步读写工具

// --- 1.计算平方 ---
fn process_math(input: &str) -> Result<String, String> {
    // 1. 去除空格
    let input = input.trim();
    
    // 2. 尝试转成数字
    // map_err: 如果转换失败，把错误变成自定义的字符串
    let x: f64 = input.parse().map_err(|_| "请输入一个有效的数字！")?;
    //成功继续，否则自定义err
    // 3. 计算平方
    let result = x * x;

    // 4. 返回格式化后的结果
    Ok(format!("{} 的平方是: {:.2}\n", x, result))
}

#[tokio::main] // 启动异步运行时
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // --- 2. 启动服务器 ---
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    //
    println!(" 计算服务器已启动，正在监听 8080 端口...");

    loop {
        // --- 3. 等待连接 ---
        // listener.accept().await 会等待，直到有人连上来
        let (mut socket, addr) = listener.accept().await?;
        println!(" 新用户连接: {}", addr);

        // --- 4. 开启并发任务 ---
        // tokio::spawn ，为每个用户创建一个独立的任务
        tokio::spawn(async move {
            let mut buffer = [0; 1024]; // 准备一个 1KB 的缓冲区

            // 循环读取用户的输入
            loop {
                // socket.read 也是异步的，读取网络数据
                let n = match socket.read(&mut buffer).await {
                    Ok(n) if n == 0 => return, //读取到 0 字节表示用户断开了
                    Ok(n) => n,
                    Err(e) => {
                        println!(" 读取错误: {:?}", e);
                        return;
                    }
                };

                // 把字节转换成字符串 (Lossy 表示如果遇到乱码字符就替换掉，不报错)
                let input_str = String::from_utf8_lossy(&buffer[0..n]);

                // 调用核心逻辑
                let response = match process_math(&input_str) {
                    Ok(res) => res,
                    Err(err_msg) => format!("⚠️ 错误: {}\n", err_msg),
                };

                // 把结果写回给用户
                if let Err(e) = socket.write_all(response.as_bytes()).await {
                    println!(" 发送失败: {:?}", e);
                    return;
                }
            }
        });
    }
}
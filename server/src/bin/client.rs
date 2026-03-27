use std::io::{self, Read, Write};
use std::net::TcpStream;

fn main() {
    // 1. 尝试连接服务器
    let mut stream = match TcpStream::connect("127.0.0.1:8080") {
        Ok(s) => {
            println!("已连接到服务器 (127.0.0.1:8080)");
            s
        }
        Err(e) => {
            println!(" 连接失败: {}", e);
            return;
        }
    };

    println!("👉 请输入数字并回车 (输入 'quit' 退出):");

    loop {
        // 2. 获取用户在终端的输入
        let mut input = String::new();
        print!("> ");
        io::stdout().flush().unwrap(); // 刷新缓冲区，确保 > 显示出来
        io::stdin().read_line(&mut input).expect("读取输入失败");

        let input = input.trim();
        if input == "quit" {
            println!(" 再见！");
            break;
        }

        // 3. 发送给服务器
        if let Err(e) = stream.write_all(input.as_bytes()) {
            println!(" 发送失败: {}", e);
            break;
        }

        // 4. 读取服务器的回复
        let mut buffer = [0; 512];
        match stream.read(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    println!(" 服务器已断开连接");
                    break;
                }
                let response = String::from_utf8_lossy(&buffer[0..n]);
                println!(" 服务器回复: {}", response.trim());
            },
            Err(e) => {
                println!(" 读取响应失败: {}", e);
                break;
            }
        }
    }
}
use async_trait::async_trait; // 引入宏，允许在 Trait 中使用异步函数并支持动态分发
use std::sync::Arc;          // 引入原子引用计数，用于跨线程共享对象
use std::time::Duration;     // 引入时间长度单位
use tokio;                   // 引入异步运行时

// --- 1. 错误模型定义 ---
#[derive(Debug, Clone)]      // 自动实现 Debug（方便打印）和 Clone 特征
pub enum PaymentError {
    InsufficientBalance,     // 余额不足
    NetworkError(String),    // 网络错误，携带一个 String 类型的错误原因
}

// --- 2. 异步 Trait 定义 ---
// 使用 #[async_trait] 解决原生 async fn 无法用于 dyn Payment 的问题
// 同时要求实现者必须满足 Send 和 Sync，以便在多线程间共享
#[async_trait] // 关键：将异步函数转换为返回 Boxed Future，解决 dyn 兼容性问题 
pub trait Payment: Send + Sync { 
    // Send + Sync 约束保证了实现此特征的对象可以安全地在线程间转移和共享
    async fn pay(&self, amount: u64) -> Result<(), PaymentError>;
    
    // 默认实现的方法，所有实现类都可以直接调用
    fn log_transaction(&self, amount: u64) {
        println!("📝 准备处理账单: {} 元", amount);
    }
}

// --- 3. 具体的支付实现 ---

// WeChatPay: 演示生命周期 'a (引用外部字符串)
pub struct WeChatPay<'a> {
    pub username: &'a str, // 'a 是生命周期，表示 username 只是对外部字符串的引用
}

#[async_trait]
impl<'a> Payment for WeChatPay<'a> {
    async fn pay(&self, amount: u64) -> Result<(), PaymentError> {
        // 模拟网络 IO，.await 会挂起当前任务并释放 CPU
        tokio::time::sleep(Duration::from_millis(800)).await;
        
        // 模拟特定的网络错误条件
        if amount == 888 {
             return Err(PaymentError::NetworkError("连接超时".to_string()));
        }

        if amount > 1000 {
            return Err(PaymentError::NetworkError("微信单笔限额 1000".to_string()));
        }
        println!("✅ 微信用户 {} 支付 {} 元成功", self.username, amount);
        Ok(()) // 成功返回
    }
}

// CreditCard: 演示所有权模式
pub struct CreditCard {
    pub card_number: String, // 持有 String 的所有权，不需要生命周期标注
}

#[async_trait]
impl Payment for CreditCard {
    async fn pay(&self, amount: u64) -> Result<(), PaymentError> {
        // 信用卡支付耗时较短
        tokio::time::sleep(Duration::from_millis(500)).await;
        println!("✅ 信用卡 {} 扣款 {} 元成功", self.card_number, amount);
        Ok(())
    }
}
// --- 4. 核心逻辑：自动重试函数 ---
async fn pay_with_retry(
    method: &dyn Payment,    // 接受特征对象引用，利用多态处理不同支付方式
    amount: u64, 
    max_retries: u32         // 最大重试次数
) -> Result<(), PaymentError> {
    let mut attempts = 0;

    loop { // 开启循环重试流
        method.log_transaction(amount);
        
        match method.pay(amount).await { // 调用具体的支付实现并等待结果
            Ok(_) => return Ok(()),     // 成功则退出函数
            Err(e) => {
                attempts += 1;
                // 模式匹配：仅当错误类型是 NetworkError 时才重试
                if let PaymentError::NetworkError(ref msg) = e {
                    if attempts <= max_retries {
                        println!("⚠️ [重试] 第 {} 次尝试失败: {}。正在重试...", attempts, msg);
                        // 异步等待 500ms 后进行下一次循环
                        tokio::time::sleep(Duration::from_millis(500)).await;
                        continue; 
                    }
                }
                // 如果是其他错误（如余额不足）或超过重试次数，则彻底报错
                return Err(e);
            }
        }
    }
}
// --- 5. 异步入口 ---
//*准备：数据存入 Box ---Box 存入 Vec --- Vec 存入 Arc。
//派发：for 循环---Arc::clone ---spawn move ---任务进入后台执行。
//收网：tasks.await ---等待所有结果 ---退出程序.        
#[tokio::main] // 启动多线程异步执行环境

async fn main() {
    let user = "亚瑟"; // 字符串字面量，生命周期是 'static
    
    // 异构集合：Box<dyn ...> 将不同类型的对象封装成相同大小的指针，存入 Vec
    let methods: Vec<Box<dyn Payment>> = vec![
        Box::new(WeChatPay { username: user }),
        Box::new(CreditCard { card_number: "6222-1234-5678".into() }),
    ];

    // Arc 用于在多个并发任务（Tasks）之间共享 Vec 的只读引用
    let shared_methods = Arc::new(methods);
    //：把数据包裹起来，引用计数设为 1。
    let mut tasks = vec![];

    println!("🚀 启动并行支付处理引擎...\n");

    for i in 0..shared_methods.len() {
        // 克隆指针，计数器加 1
        let methods_ptr = Arc::clone(&shared_methods);
        
        // spawn 开启一个独立并发任务。move 将 methods_ptr 移动到该任务内部
        let task = tokio::spawn(async move {
            let method = &methods_ptr[i]; // 索引访问特定支付方式
            
            // 逻辑分支模拟：让第一个任务（微信）触发重试逻辑
            let test_amount = if i == 0 { 888 } else { 500 };

            // 调用重试装饰函数
            match pay_with_retry(method.as_ref(), test_amount, 2).await {
                Ok(_) => println!("✨ 任务 {} 执行最终成功", i),
                Err(e) => println!("❌ 任务 {} 最终失败: {:?}", i, e),
            }
        });
        tasks.push(task); // 收集任务句柄
    }

    // 等待所有并发任务结束，主线程才会继续往下走
    for t in tasks {
        let _ = t.await; 
    }

    println!("\n🏁 所有支付流程处理完毕。");
}
/*
总体分析：
太棒了。我们要像造一台精密的机器一样，把这四个模块组装起来。为了让你听得最明白，我们按**“数据流向”**的顺序来拆解：从定义规则开始，到内存布局，再到并发调度。

---

### 第一模块：定义标准 (Trait & Error)

**——“协议先行，确定游戏规则”**

在 Rust 里，写复杂程序的第一步不是写结构体，而是写 **Trait**。

* **为什么要 #[async_trait]？**
Rust 的编译器目前还无法直接在 Trait 里处理异步函数。
因为 `async` 函数本质上返回一个隐式的 `Future` 状态机，
而编译器无法预先知道这个状态机在内存里占多大空间。
`async_trait` 宏通过把返回值包装进 `Box`（堆内存指针），
统一了大小，让 Trait 能够支持动态分发（dyn）。
* **Send + Sync 的深意**：
这是为了后续的并发做准备。`Send` 意味着“可以坐车去别的线程”，
`Sync` 意味着“可以被多个线程同时看”。如果你的支付方式里包含了某些不支持跨线程的原始指针，
编译器在这里就会直接拦住你。

---

### 第二模块：内存布局 (Ownership & Box)

**——“如何把长短不一的东西塞进一个盒子里”**

这是新手最容易迷糊的地方：`WeChatPay` 和 `CreditCard` 的字段不同，占用的内存大小也不同。

* **Vec<Box<dyn Payment>>**：
`Vec`（动态数组）要求它内部的所有元素大小必须**完全一致**。
但是 `WeChatPay`（包含一个指针）和 `CreditCard`（包含一个 String 结构体）大小是不一样的。
* **解法：Box（堆分配）**
我们不直接把结构体存进 `Vec`，而是把它们丢到堆上，然后在 `Vec` 里只存一个 `Box` 指针。
指针的大小是固定的（在 64 位系统上是 8 字节）。
* **dyn 的含义**：
它告诉编译器：“这个指针指向的对象，只要符合 `Payment` 特征就行，
具体的具体类型我们在程序运行的时候再去查表决定。”

---

### 第三模块：安全引用的纽带 (LifeCycles)

**——“活多久，谁说了算？”**

在这个程序里，`WeChatPay` 是最特殊的，因为它“借用”了外部的字符串。

```rust
pub struct WeChatPay<'a> {
    pub username: &'a str, 
}

```

* **'a 的角色**：
这是一个**生命周期契约**。它在声明：`WeChatPay` 这个实例的寿命，
绝对不能超过 `username` 背后那个字符串的寿命。
* **教学视角**：
在 `main` 函数里，`let user = "亚瑟"` 定义在最顶层。
它的生命周期是全局的（'static）。所以当我们将它传给 `WeChatPay` 时，
编译器检查发现：`user` 活得比 `methods` 长，安全！通过！

---

### 第四模块：并发与共享 (Arc & Tokio)

**——“多个人同时用一份数据，还不打架”**

这是程序的“发动机”部分。

1. **Arc (Atomic Reference Counting)**：
当我们要用 `tokio::spawn` 开启多个并发任务时，每个任务都需要访问那个 `methods` 列表。
* 普通的 `Box` 只能有一个主人，给了任务 A，任务 B 就拿不到了。
* `Arc` 解决了这个问题。它像是一个**引用计数器**。
每执行一次 `Arc::clone`，计数加 1。所有任务都持有一个指向同一内存的“分身”。


2. **move 关键字**：
在 `tokio::spawn(async move { ... })` 中，`move` 强制让异步闭包**捕获**变量的所有权。
* 如果没有 `move`，子任务会尝试“借用”主线程的变量。
* 编译器会抗议：“万一主线程跑完了，子任务还没跑完，那子任务引用的东西不就成废铁了吗？”
* 加了 `move`，子任务就把 `Arc` 的克隆体彻底抱走了，两边互不干扰。



---

### 总结：整个程序的“生命旅程”

1. **协议阶段**：通过 `Payment` Trait 确定了大家都必须会 `pay`。
2. **实例化阶段**：创建 `WeChatPay`（借用字符串）和 `CreditCard`（拥有字符串）。
3. **封装阶段**：用 `Box` 统一大小，装入 `Vec`。
4. **共享阶段**：用 `Arc` 包装 `Vec`，为多线程读取做准备。
5. **并发执行**：`tokio::spawn` 派发任务，每个任务独立调用 `pay_with_retry`，
利用 `.await` 在等待网络时自动释放 CPU。

#[tokio::main]：租了个场地，请了几个搬运工。

#[async_trait]：给所有支付方式发了统一样式的包装箱。

async fn：定义了“支付”这个动作是需要等外卖（等网络响应）的。

tokio::spawn：赶紧派小哥出门，同时送好几份外卖（并发）。

.await：你在家门口等着，哪份外卖到了就处理哪份。*/
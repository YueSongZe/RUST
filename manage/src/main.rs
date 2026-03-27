//                       **“多功能资产管理审计系统”**


use std::fmt;
//一.这一步的核心是：如何定义数据（结构体）、如何分类数据（枚举）以及如何处理可能发生的意外（错误处理）。
//复习重点为：枚举变体 、String 与 &str 、Result 错误处理。
// --- 1. 定义错误枚举 ---
// 是什么：自定义错误类型。
// 为什么：Rust 强制要求显式处理错误，使用枚举可以把所有可能的失败情况列出来。
// 怎样做：定义枚举并派生 Debug，以便在控制台打印错误信息。
#[derive(Debug)] 
pub enum AccountError {
    InvalidAmount,      // 错误类型：金额无效（如负数）
    InsufficientFunds,  // 错误类型：余额不足
}

// --- 2. 定义资产分类枚举 ---
// 是什么：带有关联数据的枚举。
// 为什么：不同资产需要记录的信息不同。现金只需名字，股票需代码和数量。枚举能把它们统合在一起。
// 怎样做：在变体名后的括号里定义需要携带的数据类型。
pub enum AssetType {
    Cash(String),     // 关联一个 String，记录币种名称
    Crypto(String, f64),           
    Stock(String, u32),     // 关联 String（代码）和 u32（股数）
}
// --- 3. 定义资产核心结构体 ---
// 是什么：自定义复合数据类型。
// 为什么：要把 ID、类型和余额这些相关联的数据打包成一个整体。
// 怎样做：使用 struct 关键字，给每个字段命名并指定类型。
pub struct Asset {
    pub id: u32,            // 资产唯一标识
    pub kind: AssetType,    // 资产具体种类（引用上面定义的枚举）
    balance: f64,           // 余额：设为私有（不加 pub），防止外部直接修改导致逻辑错误
}
impl fmt::Display for Asset {
    // 这是一个标准接口，必须实现 fmt 方法
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 我们不直接 println!，而是要把内容写入到 f (格式化器) 中
        // write! 宏的用法和 println! 几乎一样
        match &self.kind {
            AssetType::Cash(name) => write!(f, "ID: {}, 现金: {}, 余额: {}", self.id, name, self.balance),
            AssetType::Stock(c, n) => write!(f, "ID: {}, 股票: {}, 股数: {}, 余额: {}", self.id, c, n, self.balance),
            AssetType::Crypto(n, a) => write!(f, "ID: {}, 加密货币: {}, 量: {}, 余额: {}", self.id, n, a, self.balance),
        }
    }
}
// --- 4. 实现结构体方法 ---
// 是什么：为结构体定义的关联函数和方法。
// 为什么：为了封装。外部不应直接改 balance，而应通过我们定义的“规则”来改。
// 怎样做：在 impl 块中定义函数。
impl Asset {
    // 【关联函数：new】
    // 是什么：类似其他语言的构造函数。
    // 为什么：创建对象时强制进行逻辑检查（如余额不能为负）。
    // 怎样做：返回 Result，成功给 Ok，失败给 Err。
    pub fn new(id: u32, kind: AssetType, balance: f64) -> Result<Self, AccountError> {
        if balance < 0.0 {
            // 返回错误变体：金额无效
            return Err(AccountError::InvalidAmount);
        }
        // 返回成功变体：包含创建好的结构体实例
        Ok(Asset { id, kind, balance })
    }
// 【方法：display】
    // 是什么：操作实例数据的方法。
    // 为什么：需要根据不同的资产种类展示不同的描述。
    // 怎样做：使用 &self 借用实例，用 match 穷尽匹配所有种类。
    pub fn display(&self) {
        // match 是什么：模式匹配。
        // 为什么：强制检查所有枚举可能性，漏掉一个编译器就会报错，极度安全。
        match &self.kind {
            AssetType::Cash(name) => {
                println!("ID: {}, 现金账户 ({}), 余额: {}", self.id, name, self.balance);
            }
            AssetType::Stock(code, count) => {
                println!("ID: {}, 股票代码 {}, 持股数: {}, 余额: {}", self.id, code, count, self.balance);
            }
            // --- 补齐这一块 ---
            // 是什么：对 Crypto 变体的模式匹配
            // 为什么：满足 match 的穷尽性，处理加密货币逻辑
            // 怎样做：解构出里面的 name 和 rate/amount 进行打印
            AssetType::Crypto(name, amount) => {
                println!("ID: {}, 加密货币 {}, 持有量: {}, 余额: {}", self.id, name, amount, self.balance);
            }
        }
    }
    pub fn withdraw(&mut self, amount: f64) -> Result<(), AccountError> {
        if amount > self.balance {
            return Err(AccountError::InsufficientFunds); // 余额不足
        }
        self.balance -= amount;
        Ok(())
    }
}
 //二.这一步的核心是：如何管理多个资产实例，如何实现《所有权转移》，以及如何使用《动态集合存储数据》hashmap。
use std::collections::HashMap;
// 引入标准库的哈希映射表
//引入crypto

// --- 5. 定义钱包结构体 ---
// 是什么：一个包含名称和资产集合的容器。
// 为什么：我们需要一个地方统一管理所有的 Asset 实例。
// 怎样做：用 String 存储名字，用 HashMap 存储资产，Key 是 ID (u32)，Value 是资产实例。
pub struct Wallet {
    pub holder: String,
    // HashMap<K, V>：K 是键，V 是值。这里 Value 存储的是 Asset 对象本身。
    assets: HashMap<u32, Asset>,
}

impl Wallet {
    // 【构造函数：new_wallet】
    pub fn new(holder: String) -> Self {
        Wallet {
            holder,
            assets: HashMap::new(), 
        }
    }

    // 【方法：add_asset】
    pub fn add_asset(&mut self, asset: Asset) {
        println!("📝 已将资产 ID: {} 存入 {} 的钱包中。", asset.id, self.holder);
        self.assets.insert(asset.id, asset);
    }

    // 【方法：get_total_balance】
    // 状态：✅ 完美
    pub fn get_total_balance(&self) -> f64 {
        self.assets.values()
            .map(|a| a.get_balance()) 
            .sum()
    }

    // 【方法：get_crypto_balance】
    // 状态：✅ 已修复引用问题 + 括号问题
    pub fn get_crypto_balance(&self) -> f64 {
        self.assets.values()
            // 💡 重点修复：加了 '&' 避免所有权转移报错
            .filter(|a| matches!(&a.kind, AssetType::Crypto(_, _))) 
            .map(|a| a.get_balance())
            .sum()
    } // <--- 之前可能在这里多打了一个 }，现在确保只有一个

    // 【方法：find_and_display】
    pub fn find_and_display(&self, id: u32) {
        match self.assets.get(&id) {
            Some(asset) => {
                print!("🔍 找到资产：");
                println!("{}", asset); 
            },
            None => println!("⚠️ 错误：未找到 ID 为 {} 的资产。", id),
        }
    } 
    pub fn try_withdraw(&mut self, id: u32, amount: f64) -> Result<(), AccountError> {
        // 1. 先拿到资产的可变引用
        // 如果找不到 ID，我们手动返回一个错误
        // (注意：这里还没法用 ?，因为 get 返回的是 Option 不是 Result，后面章节会讲如何转换)
        let asset = match self.assets.get_mut(&id) {
            Some(a) => a,
            None => return Err(AccountError::InvalidAmount), // 借用 InvalidAmount 暂时代表找不到
        };

        // 2. 调用底层的 withdraw
        // 🔥 重点来了！注意末尾的 ?
        // 它的含义是：如果 asset.withdraw 返回 Err，函数立刻在这里停止，并把那个 Err 抛出去。
        // 如果返回 Ok，代码就继续往下走。
        asset.withdraw(amount)?; 

        println!("💸 提现成功！ID: {} 扣除了 ${}", id, amount);
        Ok(())
    }
} // <--- 这里是 impl Wallet 的结束，对应第 181 行

// 别忘了这个补充的 impl，它也是必须的
impl Asset {
    pub fn get_balance(&self) -> f64 {
        self.balance
    }
}
//三.编写main函数，完成业务流程
//变量遮蔽，表达式块，语法糖
fn main() {
    // --- 6. 变量遮蔽 (Shadowing) ---
    // 是什么：用 let 重新声明一个同名变量。
    // 为什么：当我们转换数据类型（如从 &str 转 String）时，不需要想出像 user_name_str 这种冗余的名字。
    // 怎样做：再次使用 let。
    let holder_name = "  亚瑟  "; 
    let holder_name = holder_name.trim(); // 遮蔽：现在它是去掉空格的 &str
    
    // --- 7. 表达式块计算 (Expression Blocks) ---
    // 是什么：用大括号 {} 包围的一段逻辑，最后一行不加分号即为返回值。
    // 为什么：有些变量的初始化需要复杂的判断逻辑，直接写在块里能让代码更整洁。
    // 怎样做：let 变量 = { 逻辑; 返回值 };
    let initial_balance = {
        let base = 1000.0;
        let bonus = 500.0;
        base + bonus // 返回 1500.0
    };

    // 初始化钱包
    let mut my_wallet = Wallet::new(holder_name.to_string());

    // --- 8. 处理 Result (if let 模式) ---
    // 是什么：match 的简化版，只匹配一种特定情况。
    // 为什么：如果创建资产成功才添加，失败了暂时不处理，用 if let 比 match 更简洁。
    // 怎样做：if let Ok(变量) = Result对象 { ... }
    
    // 尝试创建加密货币资产
    let btc_kind = AssetType::Crypto("BTC".into(), 60000.0);
    if let Ok(btc_asset) = Asset::new(101, btc_kind, initial_balance) {
        // 成功则存入钱包
        my_wallet.add_asset(btc_asset);
    }

    // 尝试创建股票资产
    let stock_kind = AssetType::Stock("AAPL".into(), 10);
    if let Ok(apple_stock) = Asset::new(102, stock_kind, 2500.0) {
        my_wallet.add_asset(apple_stock);
    }

    // --- 9. 最终审计与展示 ---
    println!("\n--- 钱包审计开始 ({}) ---", my_wallet.holder);

    // 测试查找功能：存在的情况
    my_wallet.find_and_display(101);

    // 测试查找功能：不存在的情况
    my_wallet.find_and_display(999);

    // 计算总额
    let total = my_wallet.get_total_balance();
    println!("💰 钱包总估值: ${}", total);

    // --- 10. 循环复习 (Looping) ---
    // 演示如何遍历 ID 列表并自动查询
    println!("\n批量查询报告：");
    for id in [101, 102, 103].iter() {
        my_wallet.find_and_display(*id); // *id 是因为 iter 返回的是引用
    }
    println!("\n--- 提现测试 ---");
    
    // 场景 1：正常提现 (BTC 余额 60000)
    // 我们用 match 处理 try_withdraw 的结果
    match my_wallet.try_withdraw(101, 1000.0) {
        Ok(_) => println!("✅ 交易完成"),
        Err(e) => println!("❌ 交易失败: {:?}", e),
    }

    // 场景 2：余额不足 (BTC 剩余 59000，试图取 10万)
    match my_wallet.try_withdraw(101, 100000.0) {
        Ok(_) => println!("✅ 交易完成"),
        Err(e) => println!("❌ 交易失败: {:?}", e), // 这里应该打印 InsufficientFunds
    }
    my_wallet.find_and_display(101);
}

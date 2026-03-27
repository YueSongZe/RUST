#[derive(Debug)]
enum PaymentError {
    InsufficientBalance, // 余额不足
    NetworkError(String), // 网络错误（带上具体原因）
    InvalidCard,        // 无效卡片
}//定义一个枚举来表示可能发生的错误
trait Payment {
    fn pay(&self, amount: u64) -> Result<(), PaymentError>;
    
    // 默认实现：所有支付方式都要打印日志
    fn log_transaction(&self, amount: u64) {
        println!("正在处理额度为 {} 的账单...", amount);
    }
}
struct WeChatPay { pub username: String }
struct CreditCard { pub card_number: String }

impl Payment for WeChatPay {
    fn pay(&self, amount: u64) -> Result<(), PaymentError> {
        if amount > 1000 {
            // 假设微信限额 1000
            return Err(PaymentError::NetworkError("超过单笔限额".to_string()));
        }
        println!("微信支付 {} 元成功", amount);
        Ok(())
    }
}

impl Payment for CreditCard {
    fn pay(&self, amount: u64) -> Result<(), PaymentError> {
        let balance = 10000; // 模拟余额
        if amount > balance {
            return Err(PaymentError::InsufficientBalance);
        }
        println!("信用卡 {} 扣款成功，扣款了{}，剩余{}", self.card_number, amount, balance - amount);
        
        Ok(())
        
    }
}
fn process_order<T: Payment>(method: T, amount: u64) -> Result<(), PaymentError> {
    method.log_transaction(amount);                     
    method.pay(amount)?;
    println!("订单状态已更新");
    Ok(())
}
//<T: Payment>: 这是一个泛型约束。接受实现了Payment类型的函数。
fn main() {
// 使用 Box 装箱，实现不同类型的支付方式共存
    let payments: Vec<Box<dyn Payment>> = vec![
        Box::new(WeChatPay { username: String::from("亚瑟") }),
        Box::new(CreditCard { card_number: String::from("1234-5678") }),
    ];
/*1.dyn Payment: 表示“实现了 Payment 的某种类型”。因为编译器不知道它具体是谁，所以它的大小是不确定的（Unsized）。
  2.但Vec要求存储大小确定且相同的类型
  3.我们使用Box进行装箱，Box<dyn Payment> 实际上包含两个指针：一个指向数据本身，一个指向虚表（vtable）。
  虚表里记录了该类型具体的 pay 方法在哪里。
*/
    for method in payments {
        method.pay(100); 
        /*
        1.method就是B<T>,在rust中，尖括号用于泛型参数：B<T>
        2.dyn Payment本质上是一个不确定大小的类型type，用Box进行装箱，使其大小确定。
        3.栈上的Box指针指向堆上的数据和虚表*/
    }
}

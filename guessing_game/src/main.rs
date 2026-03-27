use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("开始猜数！");
    let secret_number = rand::thread_rng().gen_range(1,101);//生成1到100之间的随机数，包含1不包含101
    println!("神秘数字是：{}", secret_number);
    //对于这行代码：1.首先声明了一个可变变量guess，类型为String。2.String::new()用于创建一个新的空字符串实例。（utf-8）
    // let foo=1;//immutable
    // let bar=foo;
    //foo=2;foo和bar都是不可变变量，不能被重新赋值，这就引出了我们之前mutable的概念
    loop {
        let mut guess = String::new();//创建一个新的空字符串实例，用于存储用户的输入
        io::stdin().read_line(&mut guess).expect("读取行失败");//&mut guess表示传递guess的可变引用
        println!("你猜测的数是：{}", guess);
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("请输入一个有效的数字！");
                continue;
            }
        };//这里我们使用了模式匹配来处理parse()可能返回的错误。如果parse()成功，它会返回Ok(num)，我们将num赋值给guess。如果parse()失败
            //trim()去掉字符串首尾的空白字符，parse()将字符串转换为数字类型
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了！"),
            Ordering::Greater => println!("太大了！"),
            Ordering::Equal => {
                println!("你赢了！");
                break;
            }
        }
    }
}   //使用了一个无限循环loop，直到用户猜对数字为止

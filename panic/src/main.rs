
use std::fs::File;
use std::io; 
use std::io::Read;

fn read_username_from_file()->Result<String,io::Error>{
    let mut s=String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;//?运算符用于传播错误
    Ok(s)
}
#[derive(Debug)]
pub struct Guess{
    value:i32,
}
impl Guess{
    pub fn new(value:i32)->Guess{
        if value<1 || value>100{
            panic!("猜测值必须在1到100之间，当前值为: {}",value);
        }
        Guess{value}
    }//构造函数，验证输入值是否在1到100之间
    pub fn value(&self)->i32{
        self.value
    }//提供一个公共的方法来获取私有的猜测值
}
fn main() {
    // let f =File::open("hello.txt");//尝试打开一个文件，如果失败则调用unwrap导致程序崩溃
    // let a=match f {
    //     Ok(file)=>file,
    //     Err(error)=>panic!("无法打开文件: {:?}",error),
    // };
    //let f = File::open("hello.txt").unwrap();//尝试打开一个文件，如果失败则调用unwrap导致程序崩溃
    //let f = File::open("hello.txt").expect("无法打开文件");//尝试打开一个文件，如果失败则调用expect导致程序崩溃，并输出自定义错误信息
    let a=read_username_from_file();
    println!("a={:?}",a);
    loop {
        println!("请输入一个1到100之间的数字:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("无法读取行");
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("请输入一个有效的数字!");
                break;
            }
        };//将输入的字符串转换为整数，如果失败则提示用户重新输入
        let guess = Guess::new(guess);//创建Guess实例，验证输入值是否在1到100之间
        println!("你猜测的数字是: {:?}", guess.value());//打印用户猜测的数字
    }
}

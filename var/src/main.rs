//const MAX_POINTS: u32 = 100;声明常量，类型必须标注
//变量默认不可变，使用mut关键字声明可变变量
fn main() {
    let sapce = "   ";
    let sapce = sapce.len();//变量遮蔽，shadowing，可以用相同的变量名重新声明变量
    let x = 5;
    let y={
        let x = x + 1;
        x * 2//不带分号--表达式--值；带分号--语句--无值（返回一个空tuple）
    };//表达式块，最后一行没有分号，表示该块的值
    println!("The value of y is: {}", y);
    if y > 10 {//if是一个表达式，后必须跟一个布尔表达式
        println!("y is greater than 10");
    } 
    else if y < 10 {
        println!("y is less than   10");
    }
    else{
        println!("y is equal to 10");
    }//rust要求所有分支必须有相同类型的返回值
    let condition = true;
    let number = if condition {5} else {6};//if也是一个表达式
    println!("The value of number is: {}", number);
    println!("The length of sapce is: {}", sapce);
    //loop循环
    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 10 {
            break count * 2;//loop循环可以返回值，使用break语句 
        }
    };
    println!("The result is: {}", result);//result is: 20
    //while循环
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;    
    }; 
    let a = [10, 20, 30, 40, 50];
    for i in a.iter() {//for循环,安全简洁
        println!("the value is: {}", i);
    }
    for number in (1..4).rev() {//范围迭代器
        println!("{}!", number);
    }
}

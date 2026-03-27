#[derive(Debug)]
enum OnceState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California, 
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(OnceState),
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {//match表达式来匹配不同的枚举变体
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);//打印出州的信息
            25
        }
    }
}
fn main() {
    let coin = Coin::Dime;
    let value = value_in_cents(coin);
    println!("The value of the coin is {} cents.", value);
    let c=Coin::Quarter(OnceState::Alabama);//创建一个Quarter变体，表示阿拉巴马州的25美分硬币
    let five=Some(5);
    let six=plues_one(five);
    let none=plues_one(None);
    println!("six={:?},none={:?}",six,none);
    let v=22u8;//定义一个无符号8位整数变量v并赋值为22
    match v {
        1..=5 => println!("one to five"),
        6..=10 => println!("six to ten"),
        _ => println!("others"),//使用_通配符来匹配所有其他情况
    }
    if let v=100{
        println!("v is {}",v);
    }

   
}
fn plues_one(x: Option<i32>) -> Option<i32> {
    match x {//match表达式来处理Option类型,必须处理Some和None两种情况（穷举）。
        None => None,
        Some(i) => Some(i + 1),
    }
}

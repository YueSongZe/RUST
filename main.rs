#[derive(Debug)]//派生Debug特征，允许使用{:?}打印结构体
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {//impl块定义与结构体相关联的方法,可以有多个
    ///计算矩形面积的方法，借用self的引用
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
fn main() {
    
    let  a = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area is {} square pixels.", a.area());//传递引用，不转移所有权，使用a后仍然有效
    //println!("The area is {} square pixels.", area(a)); //错误，所有权被转移到area，a不再有效
    /*在 Rust 中，对于未实现 Copy 的类型（如 String, Vec 或包含这些类型的自定义结构体），
    将其赋值给新变量或作为函数参数传递时，会发生所有权的转移，这被称为移动语义。
    移动之后，原来的变量将无法再被访问，这是 Rust 在编译期保证内存安全的关键机制
    另外std::fmt::Display,std::fmt::Debug*/
    println!("{:#?}", a);//使用派生的Debug特征打印结构体
    //自己定义的结构体编译器不知道如何打印，需要实现Display或Debug特征
     let  b = Rectangle {
        width: 10,
        height: 20,
    };
    let  c = Rectangle {
        width: 40,
        height: 60,
    };
    println!("Can a hold b? {}", a.can_hold(&b));//传递引用，不转移所有权
    println!("Can a hold c? {}", a.can_hold(&c));
    let sq = Rectangle::square(32);//使用::关联函数，不需要实例化结构体即可调用
    println!("Square: {:#?}", sq.area()); 
}



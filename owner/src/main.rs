struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
fn main() {
    let  user1=User {
        email: String::from("eqweqwe"),
        username: String::from("eqweqwe"),
        sign_in_count: 1,
        active: true, 
    };
    println!("username: {}, email: {}", user1.username, user1.email);
    println!("sign_in_count: {}, active: {}", user1.sign_in_count, user1.active);
    let  s="hello";
    let mut a = String::from(s); 
    //String类型在堆上分配内存，可以动态增长和缩小。而&str类型是不可变的字符串切片，通常指向硬编码在可执行文件中的字符串。
    //String由三部分组成：指向堆上数据的指针、字符串的长度和字符串的最大容量（即从操作系统中总获得的内存字节数）。这部分存放在栈上。
     a.push_str(", world!");
    println!("{}", s);
    let b = a; //所有权转移，a不再有效
    println!("{}", b);
    let c = b.clone(); //深拷贝，b仍然有效
    println!("b = {}, c = {}", b, c);
    let x=5;
    let y=x; //基本类型实现了Copy trait，x仍然有效
    println!("x = {}, y = {}", x, y);
    take_ownership(b); //所有权转移到函数内，b不再有效
    makes_copy(x); //基本类型实现了Copy trait，x仍然有效
    println!("x = {}", x);
    //不可变引用
    let  s1 = String::from("hello");
    let len = calculate_length(&s1); //传递引用，不转移所有权
    println!("The length of '{}' is {}.", s1, len);
    //可变引用
    let mut s2 = String::from("hello");
    let lenn=calculate_length1(&mut s2);
    println!("The length of '{}' is {}.", s2, lenn);
    /*以下三种行为会发生数据竞争：
    1. 两个或多个指针同时访问同一数据。
    2. 至少有一个指针被用来写数据。
    3. 没有任何同步机制来协调这些指针的使用。*/
    {
        let r1 = &mut s2;
        //let r2 = &mut s2; //错误：同时存在多个可变引用
        println!("{}", r1);
    }
    let r3 = &mut s2; //在可变引用作用域结束后，可以创建可变引用
    println!("{}", r3);
    //不可变引用可以有多个，但可变应用在作用域内不能再创建其他任何类型的引用。
    //引用的作用域从创建引用的地方开始，一直持续到最后一次使用该引用为止。
    //悬垂引用：引用指向的值已经无效。
    let reference_to_nothing = dangle();
    println!("{}", reference_to_nothing);//不会悬垂
    //切片
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];//切片值不拥有数据所有权，不可变
    println!("{} {}", hello, world);
    let whole=&s[..];//语法糖，等同于&s[0..s.len()]
    println!("{}", whole);
}
//drop在变量离开作用域时自动调用，释放内存。
fn take_ownership(some_string: String) {
    println!("{}", some_string);
}
fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
fn calculate_length(s: &String) -> usize {
    //通过引用访问数据而不获取所有权，不可修改。
    
    s.len()
}
fn calculate_length1(s: &mut String) -> usize {
    //通过可变引用访问数据，可以修改。
    s.push_str(", world");
    s.len()
}
fn dangle() -> String {
    let s = String::from("hello");
    s //返回String，所有权转移到调用者，不会悬垂。但如果返回&s会悬垂。
}
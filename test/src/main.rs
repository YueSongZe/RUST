// use std::io;
// /*斐波那契数列
// 目标：编写一个函数，接收一个整数 n，并返回斐波那契数列的第 n 个数。
// 要求：请分别使用循环和递归两种方式来实现。
// 提示：斐波那契数列的前两个数是 1，从第三个数开始，每个数都是前两个数之和 (1, 1, 2, 3, 5, 8, ...)。*/
// fn main() {
//     println!("请输入一个整数");
//     let mut input=String::new();
//     io::stdin().read_line(&mut input).expect("读取失败");
//     let n=input.trim().parse::<u32>().expect("请输入一个整数");
//     let result_loop=fib_loop(n);
  
//     println!("使用循环方式计算的第 {} 个斐波那契数是： {}",n,result_loop);
    

// }
// fn fib_loop(n:u32)->u32{
//     if n==1 || n==2 {
//         return 1;
//     }
//     let mut a=1;
//     let mut b=1;
//     let mut fib=0;
//     for _ in 3..=n {//将a和b的值不断更新，直到计算出第n个斐波那契数
//         fib=a+b;
//         a=b;
//         b=fib;
//     }
//     fib
// }
//第二题
// fn main() {
//     let  s1 = String::from("hello");
//     let s2 = calculate_length(&s1); // 我们传递 s1 的引用，而不是 s1 本身
//     println!("The length of '{}' is {}.", s1, s2);
// }

// fn calculate_length(s1: &String) -> usize {// 函数签名现在接收一个 &String (对 String 的引用)
//     s1.len()
// }
//第三题
//use std::io;
// fn main() {
//     let mut c=String::new();
//     io::stdin().read_line(&mut c).expect("读取失败");
//     let c=c.trim().parse::<i32>().expect("请输入一个整数");
//     let mut f=String::new();
//     io::stdin().read_line(&mut f).expect("读取失败");
//     let f=f.trim().parse::<i32>().expect("请输入一个整数");
//     let F=FF(c);
//     let C=CC(f);
//     println!("转换后的℉温度是： {}",F);
//     println!("转换后的℃温度是： {}",C);
// }
// fn CC(f:i32)->i32{
//     (f-32)*5/9 
// }
// fn FF(c:i32)->i32{
//     c*9/5+32
// }



// use std::f64::consts::PI;
// use std::io::stdin;
// //计算几何图形的面积

// enum Shape {
//     Circle(f64),          // 半径
//     Rectangle(f64, f64),  // 长和宽
//     Triangle(f64, f64),   // 底和高
// }
// fn main(){
//     println!("请输入圆的半径：");
//     let mut circle=String::new();
//     stdin().read_line(&mut circle).expect("读取失败");
//     let circle=circle.trim().parse::<f64>().expect("请输入一个数字");
//     let circle=Shape::Circle(circle);//创建圆形实例
//     println!("请输入矩形的长和宽，用空格分隔：");
//     let mut rectangle=String::new();
//     stdin().read_line(&mut rectangle).expect("读取失败");
//     let dims: Vec<f64> = rectangle.trim().split_whitespace()
//         .map(|s| s.parse().expect("请输入数字"))
//         .collect();//将输入的字符串分割并转换为数字
//     if dims.len() != 2 {
//         panic!("请输入两个数字");
//     }
//     let rectangle=Shape::Rectangle(dims[0], dims[1]);//创建矩形实例
//     println!("请输入三角形的底和高，用空格分隔：");
//     let mut triangle=String::new();
//     stdin().read_line(&mut triangle).expect("读取失败");
//     let dims: Vec<f64> = triangle.trim().split_whitespace()
//         .map(|s| s.parse().expect("请输入数字"))
//         .collect();
//     if dims.len() != 2 {
//         panic!("请输入两个数字");
//     }
//     let triangle=Shape::Triangle(dims[0], dims[1]);//创建三角形实例
//     println!("Circle area: {}", area(&circle));
//     println!("Rectangle area: {}", area(&rectangle));
//     println!("Triangle area: {}", area(&triangle));


// }
// fn area(shape: &Shape) -> f64 {
//     match shape {
//         Shape::Circle(radius) => PI * radius * radius,
//         Shape::Rectangle(length, width) => length * width,
//         Shape::Triangle(base, height) => 0.5 * base * height,
//     }
// }



//在字符串末尾追加内容
// fn main() {
//     let mut s = String::from("hello");
//      append_world(&mut s);
//     println!("{}", s);
// }
// fn append_world( s: &mut String)  {
//     s.push_str(", world");
    
// }

// 

//计算中位数，平均数，众数
// fn main(){
//     println!("请输入一个Vec:i32类型的整数列表，用空格分开"); 
//     let mut input=String::new();
//     std::io::stdin().read_line(&mut input).expect("读取失败");
//     let mut vec:Vec<i32>=input.trim().split_whitespace()
//     .map(|s|s.parse().expect("请输入整数")).collect();
//     println!("列表的中位数是： {}",median(&mut vec));
//     println!("列表的平均数是： {}",average(&vec));
//     println!("列表的众数是： {}",mode(&vec));
// }
// fn median(v:&mut Vec<i32>)->f64{
//     v.sort();//排序
//     let len=v.len();
//     if len%2==0 {
//         (v[len/2-1]+v[len/2]) as f64/2.0
//     }else{
//         v[len/2] as f64
//     }
// }
// fn average(v:&Vec<i32>)->f64{
//     let sum:i32=v.iter().sum();
//     let len=v.len();
//     sum as f64/len as f64
// }
// use std::collections::HashMap;
// fn mode(v:&Vec<i32>)->i32{
//     let mut occurrences=HashMap::new();
//     for &value in v.iter() {
//         *occurrences.entry(value).or_insert(0) += 1;
//     }
//     let mut mode: i32 = 0;
//     let mut max_count=0;
//     for (&value, &count) in occurrences.iter() {
//         if count>max_count {                    
//             max_count=count;
//             mode=value;
//         }
//     }
//     mode
// }
// fn main() {
//     let  mut fb=String::new();
//     std::io::stdin().read_line(&mut fb).expect("读取失败");
//     let fb=fb.trim().parse::<u32>().expect("请输入一个整数");
//     println!("使用递归方式计算的第 {} 个斐波那契数是： {}",fb,fib_recursive(fb)); 
// }
// fn fib_recursive(n:u32)->u32{
//     if n==1 || n==2{
//         1
//     }
//     else{
//         fib_recursive(n-1)+fib_recursive(n-2)
//     }
// }
// fn main(){
//     let s="heelo";
//     let s=s.to_string();
//     let a=s.len();
//     println!("{}",a);
// }
// fn main() {
//     let s1 = String::from("hello");
//     let s2 = calculate_length(&s1);
//     println!("The length of '{}' is {}.", s1, s2);
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }
// fn main(){
//     println!("请输入摄氏温度：");
//     let mut c=String::new();
//     std::io::stdin().read_line(&mut c).expect("读取失败");
//     let c=c.trim().parse::<f64>().expect("请输入一个数字");
//     println!("请输入华氏温度：");
//     let mut f=String::new();
//     std::io::stdin().read_line(&mut f).expect("读取失败");
//     let f=f.trim().parse::<f64>().expect("请输入一个数字");
//     let F=FF(c);
//     let C=CC(f);
//     println!("转换后的℉温度是： {}",F);
//     println!("转换后的℃温度是： {}",C);

// use std::io::stdin;

// }
// fn CC(f:f64)->f64{
//     (f-32.0)*5.0/9.0
// }
// fn FF(c:f64)->f64{
//     (c+32.0)*5.0/9.0
// }
// fn append_world(s: &mut String){
//     s.push_str(", world");
//     }
// fn main(){
//     println!("请输入一个字符串：");
//     let mut s=String::new();
//     stdin().read_line(&mut s).expect("读取失败");
//     s=s.trim().to_string();
//     append_world(&mut s);
//     println!("{}",s);
// }

// use std::f64::consts::PI;
// enum Shape{
//     Circle(f64),
//     Rectangle(f64,f64),
//     Triangle(f64,f64),
// }
// fn main(){
//     println!("请输入圆的半径：");
//     let mut circle=String::new();
//     stdin().read_line(&mut circle).expect("读取失败");
//     let circle=circle.trim().parse::<f64>().expect("请输入一个数字");
//     //let circle=PI*circle*circle;
//     let circle=Shape::Circle(circle);
//     println!("请输入矩形的长和宽，用空格分隔：");
//     let mut rectangle=String::new();
//     stdin().read_line(&mut rectangle).expect("读取失败");
//     let dims:Vec<f64>=rectangle.trim().split_whitespace()
//     .map(|s|s.parse().expect("请输入数字"))
//     .collect();
//     if dims.len()!=2{
//         panic!("请输入两个数字");
//     }
//     let rectangle=Shape::Rectangle(dims[0], dims[1]);
//     println!("请输入三角形的底和高，用空格分隔：");
//     let mut triangle=String::new();
//     stdin().read_line(&mut triangle).expect("读取失败");
//     let dims:Vec<f64>=triangle.trim().split_whitespace()
//     .map(|s|s.parse().expect("请输入数字"))
//     .collect();
//     if dims.len()!=2{
//         panic!("请输入两个数字");
//     }
//     let triangle=Shape::Triangle(dims[0], dims[1]);
//     println!("三角形的面积是： {}",area(&triangle));
//     println!("圆形的面积是：{}",area(&circle));
//     println!("矩形的面积是：{}",area(&rectangle));




// }
// fn area(shape: &Shape)->f64{
//     match shape{
//         Shape::Circle(radius)=>PI*radius*radius,
//         Shape::Rectangle(length,width)=>length*width,
//         Shape::Triangle(base,height)=>0.5*base*height,
//     }
// }




// use std::io::*;
// struct User{
//     username:String,
//     email:String,
// }
// impl User {
//     fn new(username:&str,email:&str)->User{
//         User{
//             username:String::from(username),
//             email:String::from(email),
//         }
//     }
//     fn sum(&self)->String{
//         format!("Username:{}，email:{}",self.username,self.email)

//     }
    
// }
// fn main(){
//     let mut name=String::new();
//     stdin().read_line(& mut name).expect("请输入用户名！");
//     name=name.trim().to_string();
//     let mut em=String::new();
//     stdin().read_line(& mut em).expect("请输入邮箱！");
//     em=em.trim().to_string();
//     let user1=User::new(&name, &em);
//      println!("{}", user1.sum());
// }
// fn jisuan(mut v: Vec<i32>) -> String {
//     v.sort();
//     let len = v.len();
//     let midnum: f64 = if len == 0 {
//         0.0
//     } else if len % 2 == 0 {
//         (v[len / 2 - 1] + v[len / 2]) as f64 / 2.0
//     } else {
//         v[len / 2] as f64
//     };
//     midnum.to_string();
//     let sum:i32=v.iter().sum();
//     let ave=sum/len as i32 ;
//     format!("平均数是：{}，中位数是：{}",midnum,ave)
// }
// fn safe_divide(a:f64,b:f64)->Option<f64>{
//    if b!=0.0{
//     Some(a/b)
//    }
//    else{
//     None
//    }
// }

// fn main(){
//     match safe_divide(99.0,6.3) {
//         Some(result) => println!("结果为：{}", result),//
//         None => println!("无法除以零"),
//     }
// }
fn main(){
    
}

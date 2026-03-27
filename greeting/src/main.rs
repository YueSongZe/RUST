//输入一个Vec数组，计算数组的众数
use std::{collections::HashMap, io::stdin};
fn main(){
    let mut v=String::new();
    stdin().read_line(&mut v).expect("请输入数组，元素之间以空格隔开");
    let v:Vec<i32>=v.trim().split_whitespace().map(|s|s.parse().expect("请输入整数！")).collect();
    println!("{}",much(&v));
    //打印hello
    
}
fn much(v:&Vec<i32>)->i32{
    let mut occ: HashMap<i32, i32> = HashMap::new();
    for &value in v.iter(){
        *occ.entry(value).or_insert(0) += 1;
    }

    let mut best = v[0];
    let mut best_count = 0;
    for (&num, &count) in occ.iter() {
        if count > best_count {
            best = num;
            best_count = count;
        }
    }
    best
}
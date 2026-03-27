enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
use std::collections::HashMap;//引入HashMap类型
fn main() {
   let mut v:Vec<i32>=Vec::new();//使用标准库中的Vec类型,不需要引入即可使用,存储多个连续相同类型的数据
   v=vec![1,2,3,4,5];//使用宏创建一个包含初始值的向量,这个宏会在堆上分配内存，并返回一个Vec<T>类型的实例，常用
   v.push(6);//向向量末尾添加元素
    for i in &mut v{//使用可变引用遍历向量
        *i+=1;//解引用并修改元素
       
    }
    //打印整个向量
    println!("{:?}",v);//会自动处理为引用
    v.pop();//从向量末尾移除元素
    println!("{:?}",v);
    let row=vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];//创建一个包含不同类型数据的向量
    //字符串
    let mut s=String::new();//使用String类型创建可变字符串数据
    let data="initial contents";
    let s1=data.to_string();//将字符串字面值转换为String类型
    let s2="initial contents".to_string();//另一种转换方式
    s.push_str("bar");//向字符串追加内容切片，不会取得所有权
    s.push('!');//向字符串追加单个字符
    println!("{}",s);
    format!("{}-{}",s1,s2);//使用format!宏格式化字符串，类似于其他语言的sprintf，不会转移所有权
    let s3=s1+","+&s2;//使用+运算符连接字符串，s1的所有权被移动，s2通过引用传递
    println!("{}",s3); 
    //注：String本质上是一个Vec<u8>，存储UTF-8编码的字节序列
    let len=String::from("Здравствуйте").len();//获取字符串的字节长度
    println!("len={}",len);//每个俄文字母占用2个字节
    //let answer=&s3[0];//按字节索引访问字符串会导致错误，因为可能会切割UTF-8字符
    for c in "Здравствуйте".chars(){//按字符迭代字符串
        println!("{}",c);
    }
    for b in "Здравствуйте".bytes(){//按字节迭代字符串
        println!("{}",b);
    }
    let qq="Здравствуйте";
    let s4=&qq[..4];//按字符边界切片字符串,不会切割UTF-8字符
    println!("{}",s4);//输出"Зд"
    //HashMap,类似于其他语言中的字典或关联数组
    let mut scores=HashMap::new();//创建一个空的HashMap
    scores.insert(String::from("Blue"),10);//插入键值对
    scores.insert(String::from("Yellow"),50);
    //访问HashMap中的值
    let team_name=String::from("Blue");
    let score=scores.get(&team_name);//通过键获取值，返回Option<&Value> 
    match score {
        Some(&value)=>println!("Score for {}: {}",team_name,value),
        None=>println!("No score found for {}",team_name),
    }
    //遍历HashMap
    for (key,value) in &scores{
        println!("{}: {}",key,value);
    }
    //HashMap的所有权规则
    let field_name=String::from("Favorite color");
    let field_value=String::from("Blue");
    let mut map=HashMap::new();
    map.insert(field_name,field_value);//field_name和field_value的所有权被移动，可使用&field_value获取引用
    //println!("{}",field_name);//此处使用field_name会报错，因为所有权已被移动
    //更新HashMap中的值
    map.insert(String::from("Favorite color"),String::from("Red"));//更新键对应的值
    println!("{:?}",map);
    map.insert(String::from("Favorite color"),String::from("Gray"));//再次更新键对应的值
    println!("{:?}",map);
    map.entry(String::from("Second favorite color")).or_insert(String::from("Green"));
    //使用entry API插入键值对，如果键不存在则插入,存在则不修改
    println!("{:?}",map);
    //基于现有值更新HashMap中的值
    let text="hello world wonderful world";
    let mut word_count=HashMap::new();
    for word in text.split_whitespace(){//按空白字符分割字符串
        let count=word_count.entry(word).or_insert(0);//获取单词的计数引用,不存在则插入0
        *count+=1;//增加计数，解引用并修改值
    }
    println!("{:?}",word_count);
}

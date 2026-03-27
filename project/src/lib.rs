mod front_of_house;//声明模块front_of_house
//use crate::front_of_house::hosting;//使用绝对路径引入模块到作用域，推荐
//use crate::front_of_house::hosting::add_to_waitlist;//使用绝对路径引入函数，一般到父模块即可
//use front_of_house::hosting;//使用相对路径引入模块

//引入结构体和枚举
//use crate::back_of_house::Breakfast;//引入结构体，必须指定到本身
//use crate::back_of_house::Appetizer;//引入枚举，同上

//不同函数的同名结构体用as在本地重命名
//use crate::back_of_house::Breakfast as Bkfast;
pub fn eat_at_restaurant(){
    //绝对路径
    crate::front_of_house::hosting::add_to_waitlist();//根级路径不需要pub关键字即可相互调用 
    let mut meal=back_of_house::Breakfast::summer("Rye");
    meal.toast=String::from("Wheat");
    println!("I'd like {} toast please",meal.toast);
    //meal.seasonal_fruit=String::from("blueberries");//错误，字段seasonal_fruit是私有的 

    //相对路径
    front_of_house::hosting::add_to_waitlist();
}
mod back_of_house{
    fn fix_incorrect_order(){
        super::cook_order(); 
        super::serve_order();//调用上级模块的函数
    }
        pub struct Breakfast{//结构体默认是私有的
        pub toast:String,//字段默认是私有的，必须使用pub关键字才能在结构体外部使用
        seasonal_fruit:String,
    }
    impl Breakfast{
        pub fn summer(toast:&str)->Breakfast{
            Breakfast{
                toast:String::from(toast),
                seasonal_fruit:String::from("peaches"),
            }
        }
    }
    pub enum Appetizer{
        Soup,
        Salad,//枚举的所有变体默认都是公有的
    }
}
fn serve_order(){
}
fn cook_order(){
}


// fn main() {
//     let num_list =vec![10,9,15,45,88,2];
//     let mut lagest =num_list[0];
//     for i in num_list  {
//         if i>lagest{
//             lagest=i;
//         }
        
//     }
//     println!("最大元素为：{}",lagest);

// }
fn largest<T:PartialOrd+Clone>(list:&[T])->T{
    let mut largest=list[0].clone();
    for i in list{
        if *i>largest{
            largest=i.clone();  
        }
    }
    largest
}
fn main(){
    let list=vec![10,20,5,14];
    let largest=largest(&list);
    println!("最大值为：{}",largest);

}
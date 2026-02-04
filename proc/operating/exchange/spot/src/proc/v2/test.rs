struct Data {
   pub value: String,
    pub value2: String,
}

struct Data2 {
  pub  value: String,
}
fn take_ownership(data: Data) {  // 数据被移动进来
    println!("拥有: {}", data.value);
}  // data 被丢弃





fn main() {
    let my_data = Data { value: "hello".to_string() , value2: "hello".to_string() };

    let mut my_data2 = Data2 { value: "hello".to_string() };

    my_data2.value=my_data.value;

    // my_data.value2;
    // my_data.value;

    // println!("{}", my_data.value2);  // ❌ 编译错误！my_data 已无效
    // println!("{}", my_data.value);  // ❌ 编译错误！my_data 已无效
    // 
    // 
    // take_ownership(my_data.value2);  // 所有权转移
    // println!("{}", my_data.value);  // ❌ 编译错误！my_data 已无效
}
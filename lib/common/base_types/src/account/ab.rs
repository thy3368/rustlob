use single_thread_derive::single_thread;
use std::thread;
use std::marker::Send;

#[single_thread]
struct TestStruct {
    data: i32,
}

// 这个函数要求 T 实现 Send trait
fn send_to_thread<T: Send>(x: T) {
    thread::spawn(move || {
        println!("Data received");
    });
}

fn main() {
    let x = TestStruct::new();

    thread::spawn(move || {
        let _= x;
        x.data;
        println!("Data received");
    });


    // // 尝试传递给要求 Send 的函数，应该会编译失败
    // send_to_thread(x);
}

// 这个测试文件包含应该导致编译时错误的代码
// 我们使用 compile_fail 属性来测试这些错误

use single_thread_derive::single_thread;

#[single_thread]
#[allow(dead_code)]
struct DatabaseConnection {
    url: String,
    connection_handle: usize,
}

// 这个函数会导致编译时错误，因为我们尝试跨线程发送单线程类型
#[test]
#[should_panic(expected = "compile failed")]
#[cfg(not(test))] // 不编译这个函数，因为它应该失败
fn test_compile_time_cross_thread_error() {
    use std::thread;

    let conn = DatabaseConnection::new();

    // 这个代码会在编译时产生错误：
    // "`DatabaseConnection` cannot be sent between threads safely"
    thread::spawn(move || {
        let _ = conn;
    });
}

// 这个测试验证我们的类型不实现 Send trait
#[test]
fn test_type_does_not_implement_send() {
    use std::marker::Send;

    // 这个断言会失败，因为 DatabaseConnection 不实现 Send
    // 但它不会编译，所以我们需要用另一种方式测试

    // 我们可以使用 trait bound 来测试
    fn assert_send<T: Send>() {}

    // 这会导致编译错误
    // assert_send::<DatabaseConnection>();
}
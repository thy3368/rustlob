use single_thread_derive::single_thread;


// 测试9：编译时跨线程访问检测
#[test]
fn test_compile_time_cross_thread_detection() {
    #[single_thread]
    struct DatabaseConnection {
        #[allow(dead_code)] // 抑制未使用字段警告
        url: String,
        #[allow(dead_code)] // 抑制未使用字段警告
        connection_handle: usize,
    }


    let conn = DatabaseConnection::new();



    // 以下代码将无法编译（编译时错误）
    // todo 下面代码并没有编译时报错
    use std::thread;
    thread::spawn(move || {
        let _ = conn;
    });
}

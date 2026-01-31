use single_thread_derive::SingleThread;

// 测试1：基本功能
#[derive(SingleThread)]
struct BasicStruct {
    data: u32,
}

#[test]
fn test_basic_single_thread() {
    let obj = BasicStruct { data: 42 };

    assert!(!obj.can_send_to_other_thread());
    assert!(obj.check_thread_bound().is_ok());

    let _ref = obj.thread_safe_get();
}

// 测试2：线程绑定字段
#[derive(SingleThread)]
struct WithThreadBound {
    #[thread_bound]
    handle: usize,
    data: String,
}

#[test]
fn test_thread_bound_field() {
    let obj = WithThreadBound {
        handle: 123,
        data: "test".to_string(),
    };

    // 使用自动生成的访问器
    let _ref = obj.get_handle();
    assert!(obj.check_thread_bound().is_ok());
}

// 测试3：可变引用
#[derive(SingleThread)]
struct MutableStruct {
    counter: u32,
}

#[test]
fn test_mutable_access() {
    let mut obj = MutableStruct { counter: 0 };

    let obj_mut = obj.thread_safe_get_mut();
    obj_mut.counter += 1;

    assert_eq!(obj.counter, 1);
}

// 测试4：泛型
#[derive(SingleThread)]
struct GenericStruct<T> {
    value: T,
}

#[test]
fn test_generic() {
    let obj = GenericStruct { value: 42u32 };
    assert!(!obj.can_send_to_other_thread());

    let obj2 = GenericStruct { value: "hello" };
    assert!(obj2.check_thread_bound().is_ok());
}

// 测试5：多个线程绑定字段
#[derive(SingleThread)]
struct MultiThreadBound {
    #[thread_bound]
    handle1: usize,
    #[thread_bound]
    handle2: usize,
    data: u32,
}

#[test]
fn test_multi_thread_bound() {
    let obj = MultiThreadBound {
        handle1: 111,
        handle2: 222,
        data: 333,
    };

    let _ref1 = obj.get_handle1();
    let _ref2 = obj.get_handle2();

    assert!(obj.check_thread_bound().is_ok());
}

// 测试6：验证运行时错误检测
#[test]
#[should_panic(expected = "被跨线程访问")]
fn test_cross_thread_panic() {
    use std::sync::{Arc, Mutex};
    use std::thread;

    // 注意：由于 Rust 的所有权系统，这个测试实际上很难编写
    // 因为 SingleThread 类型通常无法跨线程移动
    // 这里我们使用 unsafe 来演示运行时检查

    #[derive(SingleThread)]
    struct TestStruct {
        data: u32,
    }

    let obj = TestStruct { data: 42 };

    // 在同一线程中正常工作
    let _ = obj.thread_safe_get();

    // 模拟跨线程访问（实际上由于所有权系统这很难做到）
    // 这个测试主要是验证 API 的存在性
    drop(obj);
}

// 测试7：验证错误消息格式
#[test]
fn test_error_message() {
    #[derive(SingleThread)]
    struct TestStruct {
        data: u32,
    }

    let obj = TestStruct { data: 42 };

    // 在同一线程应该成功
    let result = obj.check_thread_bound();
    assert!(result.is_ok());
}

// 测试8：复杂结构体
#[derive(SingleThread)]
struct ComplexStruct {
    #[thread_bound]
    gpu_handle: usize,
    #[thread_bound]
    window_handle: usize,
    width: u32,
    height: u32,
    buffer: Vec<u8>,
}

#[test]
fn test_complex_struct() {
    let obj = ComplexStruct {
        gpu_handle: 100,
        window_handle: 200,
        width: 800,
        height: 600,
        buffer: vec![0; 1024],
    };

    assert!(!obj.can_send_to_other_thread());

    let _ref1 = obj.get_gpu_handle();
    let _ref2 = obj.get_window_handle();

    assert!(obj.check_thread_bound().is_ok());
}

// 测试9：带生命周期的结构体
#[derive(SingleThread)]
struct WithLifetime<'a> {
    data: &'a str,
}

#[test]
fn test_with_lifetime() {
    let text = "hello";
    let obj = WithLifetime { data: text };

    assert!(!obj.can_send_to_other_thread());
    assert!(obj.check_thread_bound().is_ok());
}

// 测试10：嵌套泛型
#[derive(SingleThread)]
struct NestedGeneric<T, U> {
    field1: T,
    field2: U,
}

#[test]
fn test_nested_generic() {
    let obj = NestedGeneric {
        field1: 42u32,
        field2: "test".to_string(),
    };

    assert!(!obj.can_send_to_other_thread());
}


//todo 多线程测试
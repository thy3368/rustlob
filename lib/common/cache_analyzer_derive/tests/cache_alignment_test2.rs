use cache_analyzer_derive::{CacheAnalyzer, SingleThread};

// 测试1：基本的 SingleThread 功能
#[derive(SingleThread)]
struct BasicSingleThread {
    data: u32,
}

#[test]
fn test_basic_single_thread() {
    let obj = BasicSingleThread { data: 42 };

    // 检查线程检查方法
    assert!(!obj.can_send_to_other_thread());
    assert!(obj.check_thread_bound().is_ok());

    // 使用线程安全的引用获取
    let _ref = obj.thread_safe_get();

    println!("\n=== BasicSingleThread 测试 ===");
    println!("数据: {}", obj.data);
    println!("线程检查: {:?}", obj.check_thread_bound());
}

// 测试2：带线程绑定字段
#[derive(SingleThread)]
struct ThreadBoundStruct {
    #[thread_bound]
    handle: usize,
    data: String,
}

#[test]
fn test_thread_bound_fields() {
    let obj = ThreadBoundStruct {
        handle: 12345,
        data: "test data".to_string(),
    };

    // 访问线程绑定字段的方法
    let _ref = obj.get_handle();

    println!("\n=== ThreadBoundStruct 测试 ===");
    println!("句柄: {}", obj.handle);
    println!("数据: {}", obj.data);
    println!("线程检查: {:?}", obj.check_thread_bound());

    assert!(!obj.can_send_to_other_thread());
}

// 测试3：多个线程绑定字段
#[derive(SingleThread)]
struct MultiThreadBound {
    #[thread_bound]
    gpu_handle: usize,
    #[thread_bound]
    window_handle: usize,
    width: u32,
    height: u32,
}

#[test]
fn test_multi_thread_bound() {
    let obj = MultiThreadBound {
        gpu_handle: 111,
        window_handle: 222,
        width: 800,
        height: 600,
    };

    // 每个线程绑定字段都有访问器
    let _ref1 = obj.get_gpu_handle();
    let _ref2 = obj.get_window_handle();

    println!("\n=== MultiThreadBound 测试 ===");
    println!("GPU 句柄: {}", obj.gpu_handle);
    println!("窗口句柄: {}", obj.window_handle);
    println!("尺寸: {}x{}", obj.width, obj.height);

    assert!(obj.check_thread_bound().is_ok());
}

// 测试4：同时使用 CacheAnalyzer 和 SingleThread
#[repr(C)]
#[derive(CacheAnalyzer)]
#[derive(SingleThread)]
struct CombinedStruct {
    #[hot]
    counter: u64,
    #[thread_bound]
    handle: usize,
    data: [u8; 32],
}

#[test]
fn test_combined_macros() {
    let obj = CombinedStruct {
        counter: 100,
        handle: 999,
        data: [0; 32],
    };

    // 使用 CacheAnalyzer 生成的方法
    let report = CombinedStruct::detailed_cache_analysis();

    println!("\n=== CombinedStruct 分析 ===");
    println!("总大小: {} 字节", report.total_size);
    println!("缓存行数: {}", report.cache_lines_needed);

    // 使用 SingleThread 生成的方法
    assert!(!obj.can_send_to_other_thread());
    assert!(obj.check_thread_bound().is_ok());

    let _ref = obj.get_handle();

    println!("线程检查: {:?}", obj.check_thread_bound());
}

// 测试5：使用 repr(C) 的结构体
#[repr(C)]
#[derive(CacheAnalyzer)]
#[derive(SingleThread)]
struct CReprStruct {
    a: u32,
    b: u64,
}

#[test]
fn test_c_repr() {
    let obj = CReprStruct { a: 10, b: 20 };

    let report = CReprStruct::detailed_cache_analysis();

    println!("\n=== CReprStruct 分析 ===");
    println!("大小: {} 字节", report.total_size);

    // repr(C) 确保字段按声明顺序排列
    assert!(report.total_size >= 12); // 至少 4 + 8 字节

    // SingleThread 功能
    assert!(!obj.can_send_to_other_thread());
    assert!(obj.check_thread_bound().is_ok());
}

// 测试6：可变引用
#[derive(SingleThread)]
struct MutableStruct {
    counter: u32,
}

#[test]
fn test_mutable_access() {
    let mut obj = MutableStruct { counter: 0 };

    // 获取可变引用
    let obj_mut = obj.thread_safe_get_mut();
    obj_mut.counter += 1;

    println!("\n=== MutableStruct 测试 ===");
    println!("计数器: {}", obj.counter);

    assert_eq!(obj.counter, 1);
}

// 测试7：泛型结构体
#[derive(SingleThread)]
struct GenericStruct<T> {
    value: T,
}

#[test]
fn test_generic_single_thread() {
    let obj = GenericStruct { value: 42u32 };

    assert!(!obj.can_send_to_other_thread());
    assert!(obj.check_thread_bound().is_ok());

    println!("\n=== GenericStruct 测试 ===");
    println!("值: {}", obj.value);
}

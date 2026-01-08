use std::env;
use std::path::PathBuf;

fn main() {
    // 编译eBPF程序
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let src_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let bpf_src = src_dir.join("src/bpf/xdp_hello.bpf.c");
    let bpf_obj = out_dir.join("xdp_hello.bpf.o");

    // 查找Linux内核头文件
    let mut include_paths = vec![];

    // 尝试常见的内核头文件路径
    let possible_paths = [
        "/usr/include",
        "/usr/include/linux",
        "/usr/src/linux-headers-$(uname -r)/include",
        "/usr/src/linux-headers-generic/include",
    ];

    for path in possible_paths.iter() {
        let p = PathBuf::from(*path);
        if p.exists() && p.is_dir() {
            include_paths.push("-I".to_string());
            include_paths.push(p.to_str().unwrap().to_string());
        }
    }

    println!("Using include paths: {:?}", include_paths);

    // 使用clang编译eBPF程序
    let status = std::process::Command::new("clang")
        .arg("-target")
        .arg("bpf")
        .arg("-Wall")
        .arg("-Wextra")
        .arg("-O2")
        .arg("-c")
        .args(include_paths)
        .arg(bpf_src.to_str().unwrap())
        .arg("-o")
        .arg(bpf_obj.to_str().unwrap())
        .status()
        .expect("Failed to compile eBPF program");

    println!("Compilation status: {:?}", status);

    if !status.success() {
        println!("cargo:warning=Failed to compile eBPF program, skipping for now");
    } else {
        // 让cargo知道需要重新编译的条件
        println!("cargo:rerun-if-changed={}", bpf_src.to_str().unwrap());
    }
}
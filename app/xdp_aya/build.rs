use std::env;
use std::path::PathBuf;

fn main() {
    // 在 macOS 系统上，我们不编译 eBPF 程序，因为它需要特定的 Linux 工具链
    if cfg!(target_os = "macos") {
        println!("cargo:warning=eBPF program compilation is skipped on macOS");
        println!("cargo:rerun-if-changed=build.rs");
        return;
    }

    // 在 Linux 系统上编译 eBPF 程序
    let target = "bpfel-unknown-none";
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let ebpf_dir = PathBuf::from(dir).join("..").join("xdp-aya-ebpf");

    println!("Compiling eBPF program in {:?}...", ebpf_dir);

    let status = std::process::Command::new("cargo")
        .arg("build")
        .arg("--release")
        .arg("--target")
        .arg(target)
        .current_dir(ebpf_dir)
        .status()
        .expect("Failed to compile eBPF program");

    assert!(status.success(), "Failed to compile eBPF program");

    // 告诉 Cargo 何时重新编译
    println!("cargo:rerun-if-changed=../xdp-aya-ebpf/src/lib.rs");
    println!("cargo:rerun-if-changed=../xdp-aya-ebpf/Cargo.toml");
}
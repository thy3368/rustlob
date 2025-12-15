# Perf 性能分析完全指南

## 📋 目录
- [基础使用](#基础使用)
- [常用场景](#常用场景)
- [Rust 特定配置](#rust-特定配置)
- [火焰图生成](#火焰图生成)
- [高级分析](#高级分析)
- [低延迟优化](#低延迟优化)

---

## 基础使用

### 安装 perf

```bash
# macOS (需要安装 DTrace 或使用 Linux)
# perf 主要用于 Linux，macOS 使用 Instruments 或 DTrace

# Linux
sudo apt-get install linux-tools-common linux-tools-generic linux-tools-`uname -r`

# 或者
sudo yum install perf

# 验证安装
perf --version
```

### 核心命令

#### 1. `perf stat` - 统计概览

```bash
# 基本统计
perf stat ./target/release/your_app

# 详细统计
perf stat -d ./target/release/your_app

# 非常详细的统计
perf stat -d -d -d ./target/release/your_app

# 自定义事件
perf stat -e cycles,instructions,cache-references,cache-misses ./target/release/your_app

# 多次运行取平均
perf stat -r 10 ./target/release/your_app
```

**输出示例：**
```
 Performance counter stats for './target/release/app':

        234.56 msec task-clock                #    0.998 CPUs utilized
             3      context-switches          #   12.785 /sec
             0      cpu-migrations            #    0.000 /sec
           456      page-faults               #    1.944 K/sec
   987,654,321      cycles                    #    4.210 GHz
   1,234,567,890    instructions              #    1.25  insn per cycle
   123,456,789      branches                  #  526.234 M/sec
     1,234,567      branch-misses             #    1.00% of all branches

       0.235123456 seconds time elapsed
```

#### 2. `perf record` - 记录性能数据

```bash
# 基本记录（CPU 采样）
perf record ./target/release/your_app

# 指定采样频率（默认 4000 Hz）
perf record -F 999 ./target/release/your_app

# 记录调用图（call graph）
perf record -g ./target/release/your_app

# 使用 dwarf 格式（更准确的调用栈）
perf record -g --call-graph dwarf ./target/release/your_app

# 记录特定 PID
perf record -p <PID> -g

# 记录特定时间
perf record -g sleep 30  # 记录 30 秒

# 记录特定事件
perf record -e cache-misses ./target/release/your_app
```

#### 3. `perf report` - 查看分析报告

```bash
# 基本报告
perf report

# 交互式报告（带调用图）
perf report -g

# 按函数分组
perf report --sort comm,dso,symbol

# 显示源代码
perf report --sort symbol -g --show-total-period

# 导出为文本
perf report --stdio > perf_report.txt
```

---

## 常用场景

### 场景 1: CPU 热点分析

找出哪些函数消耗了最多 CPU 时间：

```bash
# 1. 编译 Release 版本（带调试符号）
cargo build --release
cargo build --release --profile release-with-debug

# 2. 记录性能数据
perf record -g --call-graph dwarf ./target/release/your_app

# 3. 查看报告
perf report -g 'graph,0.5,caller'

# 4. 生成火焰图（见后文）
perf script | flamegraph.pl > flamegraph.svg
```

### 场景 2: 缓存未命中分析

分析 L1/L2/L3 缓存性能：

```bash
# 记录缓存事件
perf record -e cache-references,cache-misses,L1-dcache-loads,L1-dcache-load-misses \
  ./target/release/your_app

# 查看缓存统计
perf stat -e L1-dcache-loads,L1-dcache-load-misses,\
L1-dcache-stores,LLC-loads,LLC-load-misses,LLC-stores \
  ./target/release/your_app

# 分析缓存未命中热点
perf record -e cache-misses -g --call-graph dwarf ./target/release/your_app
perf report
```

### 场景 3: 分支预测失败分析

```bash
# 记录分支预测
perf stat -e branches,branch-misses ./target/release/your_app

# 详细分析
perf record -e branch-misses -g ./target/release/your_app
perf report
```

### 场景 4: 内存访问模式分析

```bash
# 记录内存访问
perf mem record ./target/release/your_app

# 查看内存访问报告
perf mem report

# 详细的内存带宽分析
perf stat -e cycles,instructions,\
cpu/mem-loads/,cpu/mem-stores/ \
  ./target/release/your_app
```

### 场景 5: 上下文切换分析

```bash
# 记录调度事件
perf record -e sched:sched_switch -g ./target/release/your_app

# 查看上下文切换统计
perf stat -e context-switches,cpu-migrations ./target/release/your_app
```

---

## Rust 特定配置

### Cargo.toml 配置

为了获得准确的性能分析结果，需要在 Cargo.toml 中添加调试符号：

```toml
# 保留符号信息的 Release 配置
[profile.release]
debug = true  # 保留调试符号
opt-level = 3
lto = "fat"
codegen-units = 1

# 或者创建专门的性能分析 profile
[profile.release-with-debug]
inherits = "release"
debug = true
strip = false  # 不剥离符号
```

### 编译命令

```bash
# 使用带调试符号的 release 版本
cargo build --release

# 或使用自定义 profile
cargo build --profile release-with-debug

# 确保符号未被剥离
file target/release/your_app
# 应该显示 "not stripped"
```

### 解析 Rust 符号

```bash
# 安装符号解析工具
cargo install rustfilt

# 在 perf report 中使用
perf report --stdio | rustfilt

# 或者设置环境变量
export PERF_RECORD_OPTIONS="--call-graph dwarf"
```

---

## 火焰图生成

火焰图是最直观的性能分析工具。

### 安装火焰图工具

```bash
# 克隆 FlameGraph 仓库
git clone https://github.com/brendangregg/FlameGraph
cd FlameGraph

# 或者使用 cargo-flamegraph
cargo install flamegraph
```

### 方法 1: 使用 cargo-flamegraph (推荐)

```bash
# 安装
cargo install flamegraph

# 生成火焰图
cargo flamegraph --bin your_app

# 生成火焰图（带参数）
cargo flamegraph --bin your_app -- arg1 arg2

# 自定义采样频率
cargo flamegraph -F 9999 --bin your_app

# 输出到指定文件
cargo flamegraph -o my_flamegraph.svg --bin your_app

# 打开生成的 flamegraph.svg 文件
open flamegraph.svg  # macOS
xdg-open flamegraph.svg  # Linux
```

### 方法 2: 手动使用 perf + FlameGraph

```bash
# 1. 记录性能数据
perf record -F 999 -g --call-graph dwarf ./target/release/your_app

# 2. 生成 perf.data
# perf.data 已自动生成

# 3. 转换为火焰图格式
perf script > out.perf

# 4. 折叠调用栈
/path/to/FlameGraph/stackcollapse-perf.pl out.perf > out.folded

# 5. 生成火焰图 SVG
/path/to/FlameGraph/flamegraph.pl out.folded > flamegraph.svg

# 一行命令版本
perf script | /path/to/FlameGraph/stackcollapse-perf.pl | \
  /path/to/FlameGraph/flamegraph.pl > flamegraph.svg
```

### 火焰图解读

```
火焰图 X 轴: 按字母顺序排序的函数（不是时间！）
火焰图 Y 轴: 调用栈深度
颜色: 通常是随机的（某些工具会用颜色表示不同信息）

- 宽度越大 = 函数在 CPU 上运行时间越长
- 顶部的"平台" = CPU 热点
- 点击可以缩放到特定函数
```

---

## 高级分析

### 1. 离线分析（记录后分析）

```bash
# 在生产环境记录
perf record -a -g -F 99 -o perf.data.$(date +%Y%m%d_%H%M%S) sleep 60

# 复制到开发机器分析
perf report -i perf.data.20250101_120000
```

### 2. 实时监控

```bash
# 实时 top 视图
perf top -g

# 监控特定进程
perf top -p <PID> -g

# 监控特定事件
perf top -e cache-misses -g
```

### 3. 差异对比

```bash
# 记录优化前
perf record -o before.data ./target/release/app

# 记录优化后
perf record -o after.data ./target/release/app

# 对比
perf diff before.data after.data
```

### 4. 多核分析

```bash
# 记录所有 CPU
perf record -a -g ./target/release/app

# 查看每个 CPU 的使用情况
perf report --sort cpu,comm,dso,symbol
```

### 5. 脚本分析

```bash
# 导出原始数据用于自定义分析
perf script > perf.script

# 使用 Python 分析
perf script -s script.py

# 导出为 JSON
perf script --json > perf.json
```

---

## 低延迟优化

### 关键指标监控

```bash
# 延迟相关的关键指标
perf stat -e cycles,instructions,\
cache-references,cache-misses,\
branches,branch-misses,\
cpu-clock,task-clock,\
context-switches,cpu-migrations,\
page-faults \
  ./target/release/your_app
```

### 识别延迟来源

#### 1. 缓存未命中延迟

```bash
# L1 缓存未命中 ~4 周期
# L2 缓存未命中 ~10 周期
# L3 缓存未命中 ~40 周期
# 主内存访问 ~200 周期

perf stat -e L1-dcache-load-misses,L1-icache-load-misses,\
LLC-load-misses,LLC-store-misses \
  ./target/release/your_app

# 查看缓存未命中率
# 目标: < 1% 的 L1 未命中率, < 10% 的 L3 未命中率
```

#### 2. 分支预测失败延迟

```bash
# 分支预测失败 ~15-20 周期

perf stat -e branch-misses ./target/release/your_app

# 目标: < 1% 的分支预测失败率
```

#### 3. TLB 未命中

```bash
# TLB (Translation Lookaside Buffer) 未命中分析
perf stat -e dTLB-loads,dTLB-load-misses,\
iTLB-loads,iTLB-load-misses \
  ./target/release/your_app
```

### 系统调用分析

```bash
# 记录系统调用（系统调用会增加延迟）
perf trace ./target/release/your_app

# 统计系统调用次数
perf trace -s ./target/release/your_app

# 目标: 热路径中零系统调用
```

### CPU 亲和性验证

```bash
# 验证进程是否绑定到特定 CPU
perf stat -e cpu-migrations ./target/release/your_app

# 目标: 0 次 CPU 迁移
```

### 锁竞争分析

```bash
# 记录锁操作
perf record -e syscalls:sys_enter_futex -a -g

# 分析锁竞争
perf report
```

---

## 实际优化案例

### 案例 1: 发现热点函数

```bash
# 1. 生成火焰图
cargo flamegraph --bin diff_tracker_bench

# 2. 发现 clone() 占用 30% CPU
# 解决方案: 使用引用替代克隆

# 3. 优化后再次测试
cargo flamegraph --bin diff_tracker_bench

# 4. 验证改进: clone() 占用降到 5%
```

### 案例 2: 缓存优化

```bash
# 1. 检测缓存未命中
perf stat -e cache-references,cache-misses ./target/release/app
# 发现: 15% 缓存未命中率

# 2. 详细分析
perf record -e cache-misses -g ./target/release/app
perf report

# 3. 发现问题: 随机访问大数组
# 解决方案:
#   - 数据结构重新排列（结构体数组 -> 数组结构体）
#   - 添加预取指令
#   - 使用缓存行对齐

# 4. 验证改进
perf stat -e cache-references,cache-misses ./target/release/app
# 优化后: 3% 缓存未命中率
```

### 案例 3: 减少分支预测失败

```bash
# 1. 检测分支预测
perf stat -e branches,branch-misses ./target/release/app
# 发现: 5% 分支预测失败率

# 2. 详细分析
perf record -e branch-misses -g ./target/release/app
perf annotate

# 3. 发现问题: 热路径中的 if/else
# 解决方案:
#   - 使用 likely/unlikely 提示
#   - 重组代码减少分支
#   - 使用查找表替代条件判断

# 4. 验证改进
perf stat -e branches,branch-misses ./target/release/app
# 优化后: 1% 分支预测失败率
```

---

## macOS 替代方案

由于 perf 主要用于 Linux，macOS 用户可以使用：

### 1. Instruments (Apple 官方工具)

```bash
# 启动 Instruments
instruments -t "Time Profiler" ./target/release/your_app

# 或通过 Xcode 打开
open -a Instruments
```

### 2. DTrace

```bash
# CPU 采样
sudo dtrace -n 'profile-997 { @[ustack()] = count(); }'

# 函数调用追踪
sudo dtrace -n 'pid$target::*:entry { @[ustack()] = count(); }' \
  -p $(pgrep your_app)
```

### 3. cargo-instruments

```bash
# 安装
cargo install cargo-instruments

# 使用
cargo instruments -t time --bin your_app
```

---

## 快速参考

### 常用命令速查

```bash
# 1. 快速 CPU 热点分析
cargo flamegraph --bin your_app

# 2. 缓存性能检查
perf stat -e cache-references,cache-misses ./target/release/app

# 3. 分支预测检查
perf stat -e branches,branch-misses ./target/release/app

# 4. 实时监控
perf top -g

# 5. 详细统计
perf stat -d -d -d ./target/release/app

# 6. 记录完整性能数据
perf record -g --call-graph dwarf ./target/release/app
perf report
```

### 性能优化检查清单

- [ ] **CPU 热点**: 使用火焰图识别
- [ ] **缓存未命中**: < 3% L1, < 10% L3
- [ ] **分支预测**: < 1% 失败率
- [ ] **系统调用**: 热路径中为 0
- [ ] **上下文切换**: < 100/秒
- [ ] **CPU 迁移**: 0 次（如果使用 CPU 绑定）
- [ ] **TLB 未命中**: < 1%
- [ ] **内存带宽**: 未饱和

---

## 参考资源

- [Linux perf Examples](http://www.brendangregg.com/perf.html)
- [Flame Graphs](http://www.brendangregg.com/flamegraphs.html)
- [Rust Performance Book](https://nnethercote.github.io/perf-book/)
- [Intel VTune Profiler](https://software.intel.com/content/www/us/en/develop/tools/vtune-profiler.html)

---

**更新日期**: 2025-12-16
**适用于**: Linux perf, Rust 应用性能分析

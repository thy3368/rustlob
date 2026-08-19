## 大白话：单元测试 vs 集成测试

**一句话：单元测试是"单独考每个零件"，集成测试是"把零件拼起来整车路试"。**

---

### 用造车来类比

| | 单元测试 | 集成测试 |
|---|---|---|
| **测啥** | 每个零件单独测 | 零件拼起来测 |
| **例子** | 轮胎耐不耐压、发动机能不能转、刹车灵不灵 | 整辆车开起来，转向、刹车、加速配合顺不顺 |
| **环境** | 实验室里，隔离一切外部干扰 | 真实路况，有颠簸、有红绿灯、有其他车 |
| **发现啥问题** | 零件本身坏了 | 零件单独都没问题，但拼起来互相打架 |

---

### 用代码来类比（Rust 场景）

假设你写了个"用户注册"功能，拆成三个模块：

```
输入校验 → 密码加密 → 写入数据库
```

#### 单元测试：每个模块单独考

```rust
// 测"输入校验"模块——给它各种输入，看返回对不对
#[test]
fn test_validate_email() {
    assert!(validate_email("user@example.com").is_ok());
    assert!(validate_email("not-an-email").is_err());
}

// 测"密码加密"模块——给它密码，看加密结果对不对
#[test]
fn test_hash_password() {
    let hash = hash_password("123456");
    assert!(verify_password("123456", &hash));
}
```

**特点**：
- 不碰数据库、不碰网络、不碰文件系统
- 每个测试只测一个函数/一个模块
- 其他依赖用 mock（假的）代替
- 跑起来飞快（毫秒级）

#### 集成测试：三个模块串起来考

```rust
// tests/registration.rs
#[test]
fn test_full_registration_flow() {
    // 真实流程：输入 → 校验 → 加密 → 写数据库 → 查出来验证
    let db = Database::connect("test_db");  // 真的连数据库（测试库）
    let result = register_user(&db, "user@example.com", "123456");
    
    assert!(result.is_ok());
    let user = db.get_user("user@example.com").unwrap();
    assert_eq!(user.email, "user@example.com");
    assert!(user.password_hash != "123456");  // 确认密码确实加密了
}
```

**特点**：
- 真的连数据库、真的调多个模块
- 测的是"模块之间配合好不好"
- 跑起来慢（秒级甚至分钟级）
- 能发现单元测试发现不了的问题

---

### 集成测试能发现啥单元测试发现不了的问题？

举个经典例子：

- 输入校验模块：单元测试通过 ✅（能正确识别合法/非法邮箱）
- 密码加密模块：单元测试通过 ✅（加密解密都正常）
- 数据库模块：单元测试通过 ✅（读写正常）

**但三个拼起来可能崩**：
- 校验模块说邮箱合法，传给加密模块
- 加密模块返回的 hash 长度是 64 字节
- 数据库字段设计成了 `VARCHAR(32)`
- 写进去被截断，登录时验证失败

**这种"模块之间接口对不上"的问题，单元测试永远发现不了**——因为每个模块单独测的时候都假设对方传的是对的。只有拼起来跑，才能暴露。

---

### 关键区别速查表

| 维度 | 单元测试 | 集成测试 |
|---|---|---|
| **测试范围** | 一个函数/一个模块 | 多个模块/整个流程 |
| **外部依赖** | 全部 mock 掉（假的） | 用真实的（数据库、网络等） |
| **跑的速度** | 极快（毫秒级） | 慢（秒级~分钟级） |
| **发现问题类型** | 逻辑写错了、算法错了 | 模块配合错了、接口对不上、环境问题 |
| **写的人** | 开发自己写 | 开发或测试写 |
| **跑的频率** | 每次保存都跑 | CI 里跑，或手动跑 |
| **代码位置**（Rust） | `src/` 里 `#[cfg(test)]` 模块 | `tests/` 目录下独立文件 |
| **能访问私有代码** | 能（同一个 crate） | 不能（只能用 pub API） |

---

### Rust 里的具体区别（代码位置）

```
my-project/
├── src/
│   ├── lib.rs
│   ├── validator.rs      ← 单元测试写在这附近
│   └── hasher.rs
└── tests/                ← 集成测试放这里
    ├── registration.rs
    └── login_flow.rs
```

**单元测试**：跟源代码放一起，用 `#[cfg(test)]` 标记，编译时才编进去：
```rust
// src/validator.rs
pub fn validate_email(email: &str) -> bool { ... }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_valid_email() { ... }
}
```

**集成测试**：单独放 `tests/` 目录，每个文件是独立 crate，只能调用 `pub` 的东西：
```rust
// tests/registration.rs
use my_project::{validate_email, hash_password, Database};

#[test]
fn test_full_flow() { ... }
```

---

### 一句话总结

> **单元测试是"闭卷考试，每道题单独做"——测你每个知识点会不会；集成测试是"开卷实操，整套卷子一起做"——测你能不能把知识点串起来解决真问题。前者保证零件不坏，后者保证整车能开。**

> 本回答由AI生成，仅供参考，请仔细甄别，谨慎投资。
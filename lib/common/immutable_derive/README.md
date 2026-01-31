# immutable_derive

A procedural macro crate that provides the `#[immutable]` attribute for creating immutable value objects in Rust.

## Overview

The `#[immutable]` attribute macro automatically generates const getter methods for all private fields in a struct, enforcing immutability patterns commonly used in Clean Architecture and Domain-Driven Design.

## Features

- **Auto-generate const getters**: Creates a const getter method for each field that returns an immutable reference
- **Compile-time enforcement**: Fails compilation if any field is marked as `pub`, ensuring encapsulation
- **Zero runtime overhead**: All getters are inlined and const-evaluated where possible
- **Clean Architecture compliance**: Follows value object patterns from Clean Architecture principles

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
immutable_derive = { path = "path/to/immutable_derive" }
```

## Usage

```rust
use immutable_derive::immutable;

#[immutable]
pub struct AccountId {
    id: u64,          // ✅ Private field
    name: String,     // ✅ Private field
}

// Automatically generates:
// impl AccountId {
//     pub const fn id(&self) -> &u64 { &self.id }
//     pub const fn name(&self) -> &String { &self.name }
// }

// Usage
let account = AccountId { id: 1, name: "test".into() };
println!("ID: {:?}", account.id());
println!("Name: {}", account.name());
```

## Compilation Errors

The macro will fail compilation if any field is marked as `pub`:

```rust
#[immutable]
pub struct BadExample {
    pub id: u64,  // ❌ Compile error!
}
```

Error message:
```
#[immutable] 错误: 字段 'id' 不能使用 'pub' 修饰符。
不可变结构体的所有字段必须是私有的，只能通过自动生成的 getter 方法访问。
请移除 'pub' 关键字: pub id -> id
```

## Supported Structures

- ✅ Named field structs
- ❌ Tuple structs
- ❌ Unit structs
- ❌ Enums

## Design Philosophy

This macro enforces the principle that value objects should be immutable and accessed only through well-defined interfaces. By preventing direct field access and providing only const getters, it ensures that:

1. Fields cannot be modified after construction
2. Internal representation is hidden from consumers
3. Future refactoring is easier (can change internal structure without breaking API)

## Performance

All generated getters are marked with `#[inline]` and are `const fn` where possible, resulting in zero runtime overhead compared to direct field access.

## License

MIT

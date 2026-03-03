# SBE Message Conversion Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add trait-based API for converting between Trade structs and encoder/decoder types with zero-allocation hot path.

**Architecture:** Create `SbeMessage` trait with `encode_into`/`decode_from` methods. Modify derive macros to auto-generate implementations. Provide three buffer strategies: stack (fastest), pooled (balanced), Vec (convenient).

**Tech Stack:** Rust, proc-macro2, quote, syn, crossbeam, criterion

---

## Task 1: Implement SbeError Type

**Files:**
- Create: `lib/common/sbe/src/error.rs`
- Modify: `lib/common/sbe/src/lib.rs`

**Step 1: Write failing test**

Create `lib/common/sbe/src/error.rs`:

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum SbeError {
    BufferTooSmall { required: usize, available: usize },
    InsufficientData { required: usize, available: usize },
}

impl std::fmt::Display for SbeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BufferTooSmall { required, available } => {
                write!(f, "Buffer too small: need {} bytes, have {}", required, available)
            }
            Self::InsufficientData { required, available } => {
                write!(f, "Insufficient data: need {} bytes, have {}", required, available)
            }
        }
    }
}

impl std::error::Error for SbeError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_too_small_display() {
        let err = SbeError::BufferTooSmall { required: 100, available: 50 };
        assert_eq!(err.to_string(), "Buffer too small: need 100 bytes, have 50");
    }
}
```

**Step 2: Run test**

```bash
cd lib/common/sbe && cargo test error::tests::test_buffer_too_small_display
```

Expected: FAIL (module not exported)

**Step 3: Export module**

Modify `lib/common/sbe/src/lib.rs`, add:

```rust
pub mod error;
pub use error::SbeError;
```

**Step 4: Run test again**

```bash
cd lib/common/sbe && cargo test error::tests
```

Expected: PASS

**Step 5: Commit**

```bash
git add lib/common/sbe/src/error.rs lib/common/sbe/src/lib.rs
git commit -m "feat(sbe): add SbeError type

Co-Authored-By: Claude Sonnet 4.6 <noreply@anthropic.com>"
```

---

## Task 2: Implement SbeMessage Trait

**Files:**
- Create: `lib/common/sbe/src/message.rs`
- Modify: `lib/common/sbe/src/lib.rs`

**Step 1: Write trait definition**

Create `lib/common/sbe/src/message.rs`:

```rust
use crate::SbeError;

pub trait SbeMessage: Sized {
    fn encode_into(&self, buffer: &mut [u8]) -> Result<usize, SbeError>;
    fn decode_from(buffer: &[u8]) -> Result<Self, SbeError>;
    fn max_encoded_length() -> usize;

    fn encode_to_bytes(&self) -> Result<Vec<u8>, SbeError> {
        let mut buf = vec![0u8; Self::max_encoded_length()];
        let len = self.encode_into(&mut buf)?;
        buf.truncate(len);
        Ok(buf)
    }
}
```

**Step 2: Export module**

Modify `lib/common/sbe/src/lib.rs`, add:

```rust
pub mod message;
pub use message::SbeMessage;
```

**Step 3: Verify compilation**

```bash
cd lib/common/sbe && cargo check
```

Expected: SUCCESS

**Step 4: Commit**

```bash
git add lib/common/sbe/src/message.rs lib/common/sbe/src/lib.rs
git commit -m "feat(sbe): add SbeMessage trait

Co-Authored-By: Claude Sonnet 4.6 <noreply@anthropic.com>"
```

---

## Task 3: Implement BufferPool

**Files:**
- Create: `lib/common/sbe/src/pool.rs`
- Modify: `lib/common/sbe/src/lib.rs`
- Modify: `lib/common/sbe/Cargo.toml`

**Step 1: Add crossbeam dependency**

Modify `lib/common/sbe/Cargo.toml`, add to `[dependencies]`:

```toml
crossbeam = "0.8"
```

**Step 2: Write pool implementation**

Create `lib/common/sbe/src/pool.rs`:

```rust
use crossbeam::queue::ArrayQueue;
use std::sync::Arc;

pub struct BufferPool {
    pool: Arc<ArrayQueue<Vec<u8>>>,
    buffer_size: usize,
}

impl BufferPool {
    pub fn new(capacity: usize, buffer_size: usize) -> Self {
        let pool = Arc::new(ArrayQueue::new(capacity));
        for _ in 0..capacity {
            let _ = pool.push(vec![0u8; buffer_size]);
        }
        Self { pool, buffer_size }
    }

    pub fn acquire(&self) -> PooledBuffer {
        let buf = self.pool.pop()
            .unwrap_or_else(|| vec![0u8; self.buffer_size]);
        PooledBuffer {
            buf,
            pool: self.pool.clone(),
        }
    }
}

pub struct PooledBuffer {
    pub buf: Vec<u8>,
    pool: Arc<ArrayQueue<Vec<u8>>>,
}

impl Drop for PooledBuffer {
    fn drop(&mut self) {
        let mut buf = std::mem::take(&mut self.buf);
        buf.clear();
        let _ = self.pool.push(buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pool_acquire_and_return() {
        let pool = BufferPool::new(2, 1024);
        let buf1 = pool.acquire();
        assert_eq!(buf1.buf.len(), 1024);
        drop(buf1);
        let buf2 = pool.acquire();
        assert_eq!(buf2.buf.len(), 1024);
    }
}
```

**Step 3: Export module**

Modify `lib/common/sbe/src/lib.rs`, add:

```rust
pub mod pool;
pub use pool::{BufferPool, PooledBuffer};
```

**Step 4: Run test**

```bash
cd lib/common/sbe && cargo test pool::tests
```

Expected: PASS

**Step 5: Commit**

```bash
git add lib/common/sbe/src/pool.rs lib/common/sbe/src/lib.rs lib/common/sbe/Cargo.toml
git commit -m "feat(sbe): add BufferPool for zero-allocation encoding

Co-Authored-By: Claude Sonnet 4.6 <noreply@anthropic.com>"
```

---

## Task 4: Generate SbeMessage Implementation in Codegen

**Files:**
- Modify: `lib/common/sbe_derive/src/codegen.rs`
- Modify: `lib/common/sbe_derive/src/lib.rs`

**Step 1: Add helper to detect both derives**

Modify `lib/common/sbe_derive/src/lib.rs`, add at top:

```rust
use syn::DeriveInput;

fn has_derive(input: &DeriveInput, name: &str) -> bool {
    input.attrs.iter().any(|attr| {
        if attr.path().is_ident("derive") {
            if let Ok(list) = attr.parse_args::<syn::punctuated::Punctuated<syn::Path, syn::Token![,]>>() {
                return list.iter().any(|path| path.is_ident(name));
            }
        }
        false
    })
}
```

**Step 2: Modify generate_encoder to add SbeMessage impl**

In `lib/common/sbe_derive/src/codegen.rs`, at end of `generate_encoder()` before `Ok(output)`, add:

```rust
// Generate SbeMessage impl if both derives present
let sbe_message_impl = if has_both_derives {
    let field_encodings = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        quote! { encoder.#field_name(self.#field_name); }
    }).collect::<Vec<_>>();

    let field_decodings = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        quote! { #field_name: decoder.#field_name(), }
    }).collect::<Vec<_>>();

    quote! {
        impl sbe::SbeMessage for #name {
            fn encode_into(&self, buffer: &mut [u8]) -> Result<usize, sbe::SbeError> {
                if buffer.len() < Self::max_encoded_length() {
                    return Err(sbe::SbeError::BufferTooSmall {
                        required: Self::max_encoded_length(),
                        available: buffer.len(),
                    });
                }

                let write_buf = sbe::WriteBuf::new(buffer);
                let mut encoder = #encoder_name::default().wrap(write_buf, 0);
                #(#field_encodings)*
                Ok(encoder.encoded_length())
            }

            fn decode_from(buffer: &[u8]) -> Result<Self, sbe::SbeError> {
                let read_buf = sbe::ReadBuf::new(buffer);
                let decoder = #decoder_name::default().wrap(
                    read_buf, 0, #module_name::SBE_BLOCK_LENGTH, 0
                );
                Ok(Self { #(#field_decodings)* })
            }

            fn max_encoded_length() -> usize {
                #module_name::SBE_BLOCK_LENGTH as usize
            }
        }
    }
} else {
    quote! {}
};
```

**Step 3: Append to output**

Change the final `Ok(output)` to:

```rust
let final_output = quote! {
    #output
    #sbe_message_impl
};
Ok(final_output)
```

**Step 4: Test with trade_codec example**

```bash
cd lib/common/sbe_derive && cargo build --example trade_codec
```

Expected: SUCCESS

**Step 5: Commit**

```bash
git add lib/common/sbe_derive/src/codegen.rs lib/common/sbe_derive/src/lib.rs
git commit -m "feat(sbe_derive): auto-generate SbeMessage trait impl

Co-Authored-By: Claude Sonnet 4.6 <noreply@anthropic.com>"
```

---

## Task 5: Update Trade Codec Example

**Files:**
- Modify: `lib/common/sbe_derive/examples/trade_codec.rs`

**Step 1: Replace manual encoding with SbeMessage**

Replace lines 36-56 with:

```rust
// Encode using SbeMessage trait
println!("Encoding trade message...");
let trade = Trade {
    trade_id: 12345,
    symbol: b'A',
    price: 100.50,
    quantity: 1000,
};

let mut buffer = [0u8; 1024];
let len = trade.encode_into(&mut buffer).unwrap();

println!("  Encoded {} bytes", len);
println!("  trade_id: {}", trade.trade_id);
println!("  symbol: {}", trade.symbol as char);
println!("  price: {}", trade.price);
println!("  quantity: {}", trade.quantity);
```

**Step 2: Replace manual decoding**

Replace lines 59-72 with:

```rust
// Decode using SbeMessage trait
println!("\nDecoding trade message...");
let decoded = Trade::decode_from(&buffer[..len]).unwrap();

println!("  trade_id: {}", decoded.trade_id);
println!("  symbol: {}", decoded.symbol as char);
println!("  price: {}", decoded.price);
println!("  quantity: {}", decoded.quantity);
```

**Step 3: Run example**

```bash
cd lib/common/sbe_derive && cargo run --example trade_codec
```

Expected: Output showing successful encoding/decoding

**Step 4: Commit**

```bash
git add lib/common/sbe_derive/examples/trade_codec.rs
git commit -m "refactor(sbe_derive): use SbeMessage trait in example

Co-Authored-By: Claude Sonnet 4.6 <noreply@anthropic.com>"
```

---

## Task 6: Create Performance Benchmark

**Files:**
- Create: `lib/common/sbe_derive/benches/buffer_allocation.rs`
- Modify: `lib/common/sbe_derive/Cargo.toml`

**Step 1: Add benchmark dependencies**

Modify `lib/common/sbe_derive/Cargo.toml`, add:

```toml
[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }

[[bench]]
name = "buffer_allocation"
harness = false
```

**Step 2: Create benchmark**

Create `lib/common/sbe_derive/benches/buffer_allocation.rs`:

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sbe_derive::{SbeEncode, SbeDecode};
use sbe::{SbeMessage, BufferPool};

#[derive(SbeEncode, SbeDecode)]
#[sbe(template_id = 1, schema_id = 1, version = 0)]
pub struct Trade {
    #[sbe(id = 0)]
    pub trade_id: u64,
    #[sbe(id = 1)]
    pub symbol: u8,
    #[sbe(id = 2)]
    pub price: f64,
    #[sbe(id = 3)]
    pub quantity: i32,
}

fn bench_stack_buffer(c: &mut Criterion) {
    let trade = Trade {
        trade_id: 12345,
        symbol: b'A',
        price: 100.50,
        quantity: 1000,
    };

    c.bench_function("stack_buffer", |b| {
        b.iter(|| {
            let mut buffer = [0u8; 1024];
            let len = trade.encode_into(&mut buffer).unwrap();
            black_box(&buffer[..len]);
        });
    });
}

fn bench_pooled_buffer(c: &mut Criterion) {
    let trade = Trade {
        trade_id: 12345,
        symbol: b'A',
        price: 100.50,
        quantity: 1000,
    };

    let pool = BufferPool::new(128, 1024);

    c.bench_function("pooled_buffer", |b| {
        b.iter(|| {
            let mut pooled = pool.acquire();
            let len = trade.encode_into(&mut pooled.buf).unwrap();
            black_box(&pooled.buf[..len]);
        });
    });
}

fn bench_vec_allocation(c: &mut Criterion) {
    let trade = Trade {
        trade_id: 12345,
        symbol: b'A',
        price: 100.50,
        quantity: 1000,
    };

    c.bench_function("vec_allocation", |b| {
        b.iter(|| {
            let bytes = trade.encode_to_bytes().unwrap();
            black_box(&bytes);
        });
    });
}

criterion_group!(benches, bench_stack_buffer, bench_pooled_buffer, bench_vec_allocation);
criterion_main!(benches);
```

**Step 3: Run benchmark**

```bash
cd lib/common/sbe_derive && cargo bench --bench buffer_allocation
```

Expected: Benchmark results showing stack < 20ns, pooled < 60ns, vec < 250ns

**Step 4: Commit**

```bash
git add lib/common/sbe_derive/benches/buffer_allocation.rs lib/common/sbe_derive/Cargo.toml
git commit -m "perf(sbe_derive): add buffer allocation benchmarks

Co-Authored-By: Claude Sonnet 4.6 <noreply@anthropic.com>"
```

---

## Task 7: Final Integration Test

**Step 1: Run all tests**

```bash
cd lib/common/sbe && cargo test
cd lib/common/sbe_derive && cargo test
```

Expected: All tests PASS

**Step 2: Run all examples**

```bash
cd lib/common/sbe_derive && cargo run --example trade_codec
```

Expected: Successful roundtrip

**Step 3: Run benchmarks**

```bash
cd lib/common/sbe_derive && cargo bench
```

Expected: Performance targets met

**Step 4: Final commit**

```bash
git add -A
git commit -m "feat(sbe): complete SbeMessage trait implementation

- Add SbeError type for error handling
- Add SbeMessage trait with encode_into/decode_from
- Add BufferPool for zero-allocation encoding
- Auto-generate SbeMessage impl in derive macros
- Add performance benchmarks
- Update examples to use new API

Performance results:
- Stack buffer: < 20ns
- Pooled buffer: < 60ns
- Vec allocation: < 250ns

Co-Authored-By: Claude Sonnet 4.6 <noreply@anthropic.com>"
```

---

## Execution Notes

- Follow TDD: write test → run (fail) → implement → run (pass) → commit
- Each task is independent and can be executed sequentially
- Commit after each task completion
- Run tests frequently to catch issues early
- Benchmark at the end to verify performance targets

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

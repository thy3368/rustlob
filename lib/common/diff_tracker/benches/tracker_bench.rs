use criterion::{black_box, criterion_group, criterion_main, Criterion};
use diff_tracker::{track_auto, track_with_tracker, Diff};

#[derive(Clone, Debug, PartialEq, Diff)]
struct Order {
    id: String,
    symbol: String,
    price: i64,
    quantity: u32,
    status: String,
}

impl Order {
    fn new() -> Self {
        Self {
            id: "ORD123".to_string(),
            symbol: "BTCUSDT".to_string(),
            price: 50000,
            quantity: 100,
            status: "pending".to_string(),
        }
    }
}

fn bench_track_with_tracker(c: &mut Criterion) {
    c.bench_function("track_with_tracker", |b| {
        b.iter(|| {
            let mut order = Order::new();
            track_with_tracker(&mut order, |o, tracker| {
                tracker.set("price", &mut o.price, black_box(51000));
                tracker.set("quantity", &mut o.quantity, black_box(150));
                tracker.set("status", &mut o.status, black_box("confirmed".to_string()));
            })
        });
    });
}

fn bench_track_auto(c: &mut Criterion) {
    c.bench_function("track_auto", |b| {
        b.iter(|| {
            let mut order = Order::new();
            track_auto(&mut order, |o| {
                o.price = black_box(51000);
                o.quantity = black_box(150);
                o.status = black_box("confirmed".to_string());
            })
        });
    });
}

fn bench_manual_diff(c: &mut Criterion) {
    c.bench_function("manual_diff", |b| {
        b.iter(|| {
            let old_order = Order::new();
            let mut new_order = old_order.clone();
            new_order.price = black_box(51000);
            new_order.quantity = black_box(150);
            new_order.status = black_box("confirmed".to_string());
            old_order.diff(&new_order)
        });
    });
}

criterion_group!(benches, bench_track_with_tracker, bench_track_auto, bench_manual_diff);
criterion_main!(benches);

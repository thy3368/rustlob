use base_types::base_types::TraderId;
use base_types::cqrs::cqrs_types::CMetadata;
use base_types::exchange::spot::spot_types::{
    OrderSide, OrderType, SpotOrder, TimeInForce, TradingPair,
};
use base_types::{Price, Quantity};
use cmd_handler::pipe_line_handler::CmdPipeLineHandler;
use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use db_repo::adapter::v2::memdb_repo::MemdbRepo;
use lob_repo::adapter::embedded_lob_repo::EmbeddedLobRepo;
use lob_repo::adapter::local_lob_impl::LocalLob;
use lob_repo::core::symbol_lob_repo::MultiSymbolLobRepo;
use spot_behavior::proc::behavior::v2::spot_trade_behavior::NewOrderCmd;
use spot_behavior::proc::v2::trade_cmd_handlers::v3::cmd_handler::match_order_handler::MatchOrderCmdHandler;
use spot_behavior::proc::v2::trade_cmd_handlers::v3::cmd_handler::mock_repo::MockEventPublisher;
use spot_behavior::proc::v2::trade_cmd_handlers::v3::cmd_handler::place_order_handler::PlaceOrderCmdHandler;
use spot_behavior::proc::v2::trade_cmd_handlers::v3::cmd_handler::sett_order_handler::SettOrderCmdHandler;
use spot_behavior::proc::v2::trade_cmd_handlers::v3::pipe_line::place_order_pipeline_handler::PlaceOrderPipelineHandler;

/// 统计: branch miss rate; cache miss rate
fn create_test_cmd_with_quantity(client_order_id: &str, quantity: f64) -> NewOrderCmd {
    NewOrderCmd::new(
        CMetadata::default(),
        TradingPair::BtcUsdt,
        OrderSide::Buy,
        OrderType::Limit,
        Some(TimeInForce::GTC),
        Some(Quantity::from_f64(quantity)),
        None,
        Some(Price::from_f64(50000.0)),
        Some(client_order_id.to_string()),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    )
}

fn create_sell_order_with_quantity(
    order_id: u64,
    client_order_id: &str,
    quantity: f64,
) -> SpotOrder {
    SpotOrder::create_order(
        order_id,
        TraderId::new([1u8; 8]),
        TradingPair::BtcUsdt,
        OrderSide::Sell,
        Price::from_f64(50000.0),
        Quantity::from_f64(quantity),
        TimeInForce::GTC,
        Some(client_order_id.to_string()),
        Quantity::default(),
    )
}

fn create_embedded_lob(maker_orders: Vec<SpotOrder>) -> EmbeddedLobRepo<SpotOrder> {
    let mut lob = EmbeddedLobRepo::new(vec![LocalLob::new(TradingPair::BtcUsdt)]);
    for maker_order in maker_orders {
        lob.add_order(TradingPair::BtcUsdt, maker_order).expect("add maker order should succeed");
    }
    lob
}

fn build_pipeline_with_match()
-> PlaceOrderPipelineHandler<MemdbRepo, MockEventPublisher, EmbeddedLobRepo<SpotOrder>> {
    let maker_orders = vec![
        create_sell_order_with_quantity(31, "maker_sell_002", 1.0),
        create_sell_order_with_quantity(32, "maker_sell_003", 1.0),
    ];
    let repo = MemdbRepo::default();
    PlaceOrderPipelineHandler::new(
        PlaceOrderCmdHandler::new(repo.clone(), MockEventPublisher),
        MatchOrderCmdHandler::new(
            repo.clone(),
            MockEventPublisher,
            create_embedded_lob(maker_orders),
        ),
        SettOrderCmdHandler::new(repo, MockEventPublisher),
    )
}

fn build_pipeline_without_match()
-> PlaceOrderPipelineHandler<MemdbRepo, MockEventPublisher, EmbeddedLobRepo<SpotOrder>> {
    let repo = MemdbRepo::default();
    PlaceOrderPipelineHandler::new(
        PlaceOrderCmdHandler::new(repo.clone(), MockEventPublisher),
        MatchOrderCmdHandler::new(
            repo.clone(),
            MockEventPublisher,
            create_embedded_lob(Vec::new()),
        ),
        SettOrderCmdHandler::new(repo, MockEventPublisher),
    )
}

fn benchmark_place_order_pipeline_exec(c: &mut Criterion) {
    let mut group = c.benchmark_group("PlaceOrderPipelineHandler::exec");

    group.bench_function("no_match", |b| {
        b.iter_batched(
            || {
                (
                    build_pipeline_without_match(),
                    create_test_cmd_with_quantity("bench_no_match", 1.0),
                )
            },
            |(pipeline, cmd)| {
                let reply = pipeline.handle(black_box(&cmd)).unwrap();
                black_box(reply);
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("full_match_two_trades", |b| {
        b.iter_batched(
            || {
                (
                    build_pipeline_with_match(),
                    create_test_cmd_with_quantity("bench_full_match", 2.0),
                )
            },
            |(pipeline, cmd)| {
                let reply = pipeline.handle(black_box(&cmd)).unwrap();
                black_box(reply);
            },
            BatchSize::SmallInput,
        );
    });

    group.finish();
}

criterion_group!(benches, benchmark_place_order_pipeline_exec);
criterion_main!(benches);

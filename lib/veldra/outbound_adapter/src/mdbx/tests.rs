use example_core::Balance;
use veldra_core::entity::{BlockExecutionBody, NewBlock};
use veldra_core::use_case::BlockEntityChange;

use super::test_support::{TestDir, built_block, header_body_changes};
use crate::{VeldraMdbxBlockStore, VeldraMdbxStorageError};

#[test]
// 验证 crate 根对外 re-export 仍然可用，避免模块拆分后破坏公开入口。
fn crate_root_re_exports_are_accessible() {
    let _ = std::any::type_name::<VeldraMdbxBlockStore>();
    let _ = std::any::type_name::<VeldraMdbxStorageError>();
}

#[test]
// 验证完整写入一个 block 后，header / command / event / snapshot 都能按原语义读回。
fn append_and_read_back_block_data_and_snapshots() {
    let temp_dir = TestDir::new();
    let store = VeldraMdbxBlockStore::open(&temp_dir.path).expect("store should open");
    let built = built_block();
    let (header, body, changes) = header_body_changes(&built);

    store.append_block(&header, &body, &changes).expect("append should succeed");

    let stored_header = store
        .get_block_header(header.block_height)
        .expect("header read should succeed")
        .expect("header should exist");
    let header_by_hash = store
        .get_block_header_by_hash(&header.block_hash)
        .expect("hash lookup should succeed")
        .expect("header should exist by hash");
    let stored_commands =
        store.scan_block_commands(header.block_height).expect("command scan should succeed");
    let stored_events =
        store.scan_block_events(header.block_height).expect("event scan should succeed");
    let stored_order = store
        .load_current_spot_order("trader-1-BTCUSDT-7")
        .expect("order load should succeed")
        .expect("order should exist");
    let stored_balance = store
        .load_current_balance("trader-1", "USDT")
        .expect("balance load should succeed")
        .expect("balance should exist");

    assert_eq!(stored_header, header);
    assert_eq!(header_by_hash, header);
    assert_eq!(
        stored_commands.iter().map(|command| command.command_id.as_str()).collect::<Vec<_>>(),
        vec!["cmd-1", "cmd-2"]
    );
    assert_eq!(stored_events, body.replayable_events);
    assert_eq!(stored_order.order_id, "trader-1-BTCUSDT-7");
    assert_eq!((stored_order.reserved_quote, stored_order.status.as_str()), (200, "open"));
    assert_eq!(
        (stored_balance.available, stored_balance.frozen, stored_balance.version),
        (1_500, 0, 2)
    );
}

#[test]
// 验证同一个 block_height 不能重复写入，避免高度主键被覆盖。
fn duplicate_block_height_is_rejected() {
    let temp_dir = TestDir::new();
    let store = VeldraMdbxBlockStore::open(&temp_dir.path).expect("store should open");
    let built = built_block();
    let (header, body, changes) = header_body_changes(&built);

    store.append_block(&header, &body, &changes).expect("first append should succeed");
    let error =
        store.append_block(&header, &body, &changes).expect_err("second append should fail");

    assert!(matches!(error, VeldraMdbxStorageError::DuplicateBlockHeight(2)));
}

#[test]
// 验证不同高度也不能复用同一个 block_hash，避免 hash 索引出现歧义。
fn duplicate_block_hash_is_rejected() {
    let temp_dir = TestDir::new();
    let store = VeldraMdbxBlockStore::open(&temp_dir.path).expect("store should open");
    let built = built_block();
    let (header, body, changes) = header_body_changes(&built);
    store.append_block(&header, &body, &changes).expect("first append should succeed");

    let second_header = NewBlock {
        block_height: header.block_height + 1,
        parent_block_hash: header.parent_block_hash.clone(),
        commands_root: header.commands_root.clone(),
        events_root: header.events_root.clone(),
        post_state_root: header.post_state_root.clone(),
        block_hash: header.block_hash.clone(),
    };
    let second_body = BlockExecutionBody {
        block_hash: second_header.block_hash.clone(),
        block_height: second_header.block_height,
        commands: body.commands.clone(),
        replayable_events: body.replayable_events.clone(),
    };

    let error = store
        .append_block(&second_header, &second_body, &changes)
        .expect_err("duplicate hash should fail");

    assert!(
        matches!(error, VeldraMdbxStorageError::DuplicateBlockHash(hash) if hash == header.block_hash)
    );
}

#[test]
// 验证投影阶段一旦失败，整笔 append 会回滚，不留下部分 header 或 snapshot。
fn projection_failure_leaves_no_partial_block() {
    let temp_dir = TestDir::new();
    let store = VeldraMdbxBlockStore::open(&temp_dir.path).expect("store should open");
    let built = built_block();
    let (header, body, mut changes) = header_body_changes(&built);

    changes.push(BlockEntityChange::BalanceUpdated(
        cmd_handler::command_use_case_def2::UpdatedEntityPair {
            before: Balance::new("trader-1".to_string(), "USDT".to_string(), 10, 0, 1),
            after: Balance::new("trader-1".to_string(), "BTC".to_string(), 10, 0, 2),
        },
    ));

    let error = store
        .append_block(&header, &body, &changes)
        .expect_err("projection mismatch should abort append");
    assert!(matches!(error, VeldraMdbxStorageError::BalanceProjectionMismatch));
    assert!(
        store.get_block_header(header.block_height).expect("header read should succeed").is_none()
    );
    assert!(
        store
            .load_current_spot_order("trader-1-BTCUSDT-7")
            .expect("order read should succeed")
            .is_none()
    );
    assert!(
        store
            .load_current_balance("trader-1", "USDT")
            .expect("balance read should succeed")
            .is_none()
    );
}

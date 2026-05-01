use alloy_primitives::Address;
use web3::revm::{contracts, RevmExecutor};

fn main() {
    let mut executor = RevmExecutor::new();
    let request_id = "req-evm-1";
    let performer = "acct-evm-1";
    let settlement_id = contracts::settlement_id_from_seed(request_id);
    let beneficiary = contracts::address_from_performer(performer);
    let amount = 7u64;

    println!("settlement_id={settlement_id}");
    println!("beneficiary={beneficiary:?}");
    println!("amount={amount}");

    let deploy = executor.deploy_contract("settlement", contracts::get_settlement_escrow_bytecode());
    println!("deploy={deploy:?}");

    let create_calldata = contracts::encode_create_settlement(&settlement_id, beneficiary, amount);
    println!("create_calldata=0x{}", alloy_primitives::hex::encode(&create_calldata));
    let create = executor.call_contract("settlement", create_calldata);
    println!("create={create:?}");

    let get_amount = executor.view_contract("settlement", contracts::encode_get_amount(&settlement_id));
    println!("get_amount={get_amount:?}");

    let is_released = executor.view_contract("settlement", contracts::encode_is_released(&settlement_id));
    println!("is_released={is_released:?}");
}

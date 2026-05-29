use sha3::{Digest, Keccak256};

fn main() {
    let source = std::fs::read_to_string("src/revm/contracts/SettlementEscrow.sol")
        .expect("failed to read SettlementEscrow.sol");
    let hash = Keccak256::digest(source.as_bytes());
    println!("cargo:warning=SETTLEMENT_ESCROW_SOURCE_HASH={}", alloy_primitives::hex::encode(hash));
}

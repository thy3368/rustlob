use sha3::{Digest, Keccak256};

fn main() {
    let source = match std::fs::read_to_string("src/revm/contracts/SettlementEscrow.sol") {
        Ok(source) => source,
        Err(err) => {
            eprintln!("failed to read SettlementEscrow.sol: {err}");
            std::process::exit(1);
        }
    };
    let hash = Keccak256::digest(source.as_bytes());
    println!("cargo:rustc-env=SETTLEMENT_ESCROW_SOURCE_HASH={}", hex::encode(hash));
}

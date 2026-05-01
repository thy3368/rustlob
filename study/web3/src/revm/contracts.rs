use alloy_primitives::{hex, Address, FixedBytes};
use sha3::{Digest, Keccak256};

pub const COUNTER_BYTECODE: &str = "6080604052348015600e575f5ffd5b5060c580601a5f395ff3fe6080604052348015600e575f5ffd5b50600436106030575f3560e01c806306661abd146034578063d09de08a14604d575b5f5ffd5b603b5f5481565b60405190815260200160405180910390f35b60536055565b005b60015f5f82825460649190606b565b9091555050565b80820180821115608957634e487b7160e01b5f52601160045260245ffd5b9291505056fea2646970667358221220a48aa1ce6ad99f3a568df931a13cb1db6bbd0417ec8cf682182c794454b27c5664736f6c634300081e0033";
pub const SETTLEMENT_ESCROW_BYTECODE: &str = "6080604052348015600f57600080fd5b506103fa8061001f6000396000f3fe608060405234801561001057600080fd5b506004361061004c5760003560e01c80630e0a0d741461005157806383ef844814610087578063efdbe39f146100bd578063f4054221146100d2575b600080fd5b61007461005f366004610367565b60009081526020819052604090206001015490565b6040519081526020015b60405180910390f35b6100ad610095366004610367565b60009081526020819052604090206002015460ff1690565b604051901515815260200161007e565b6100d06100cb366004610367565b6100e5565b005b6100d06100e0366004610380565b6101e6565b600081815260208190526040902080546001600160a01b031661013b5760405162461bcd60e51b81526020600482015260096024820152681b9bdd08199bdd5b9960ba1b60448201526064015b60405180910390fd5b600281015460ff16156101835760405162461bcd60e51b815260206004820152601060248201526f185b1c9958591e481c995b19585cd95960821b6044820152606401610132565b60028101805460ff191660019081179091558154908201546040516001600160a01b039092169184917fc9ae16b27cf6ead0b905855ec7908be6df6660abd629f3ced1d69de3cab4e276916101da91815260200190565b60405180910390a35050565b6001600160a01b0382166102325760405162461bcd60e51b8152602060048201526013602482015272696e76616c69642062656e656669636961727960681b6044820152606401610132565b600081116102735760405162461bcd60e51b815260206004820152600e60248201526d1a5b9d985b1a5908185b5bdd5b9d60921b6044820152606401610132565b6000838152602081905260409020546001600160a01b0316156102c95760405162461bcd60e51b815260206004820152600e60248201526d616c72656164792065786973747360901b6044820152606401610132565b604080516060810182526001600160a01b03848116808352602080840186815260008587018181528a825281845290879020955186546001600160a01b031916951694909417855551600185015591516002909301805460ff1916931515939093179092559151838152909185917f46d64c17ba4e1987c50cf6a51d917cbdbf9dd1c578a39c2f7464fbd7b2bfc59e910160405180910390a3505050565b60006020828403121561037957600080fd5b5035919050565b60008060006060848603121561039557600080fd5b8335925060208401356001600160a01b03811681146103b357600080fd5b92959294505050604091909101359056fea2646970667358221220d5ee3726095284375bb374d3f1b15b14c5a822aec1c38996f2fb99768235c95664736f6c634300081e0033";

pub const SETTLEMENT_ESCROW_SOLIDITY_SOURCE: &str = include_str!("contracts/SettlementEscrow.sol");

pub fn get_counter_bytecode() -> Vec<u8> {
    hex::decode(COUNTER_BYTECODE).expect("invalid counter bytecode")
}

pub fn get_settlement_escrow_bytecode() -> Vec<u8> {
    hex::decode(SETTLEMENT_ESCROW_BYTECODE).expect("invalid settlement escrow bytecode")
}

pub mod selectors {
    use alloy_primitives::FixedBytes;

    pub const INCREMENT: FixedBytes<4> = FixedBytes([0xd0, 0x9d, 0xe0, 0x8a]);
    pub const COUNT: FixedBytes<4> = FixedBytes([0x06, 0x66, 0x1a, 0xbd]);
    pub const CREATE_SETTLEMENT: FixedBytes<4> = FixedBytes([0xf4, 0x05, 0x42, 0x21]);
    pub const RELEASE_SETTLEMENT: FixedBytes<4> = FixedBytes([0xef, 0xdb, 0xe3, 0x9f]);
    pub const GET_AMOUNT: FixedBytes<4> = FixedBytes([0x0e, 0x0a, 0x0d, 0x74]);
    pub const IS_RELEASED: FixedBytes<4> = FixedBytes([0x83, 0xef, 0x84, 0x48]);
}

pub fn encode_increment() -> Vec<u8> {
    selectors::INCREMENT.to_vec()
}

pub fn encode_get() -> Vec<u8> {
    selectors::COUNT.to_vec()
}

pub fn settlement_id_from_seed(seed: &str) -> String {
    let mut hasher = Keccak256::new();
    hasher.update(seed.as_bytes());
    hex::encode(hasher.finalize())
}

pub fn address_from_performer(performer: &str) -> Address {
    let mut hasher = Keccak256::new();
    hasher.update(performer.as_bytes());
    let result = hasher.finalize();
    Address::from_slice(&result[12..])
}

pub fn settlement_id_word(settlement_id: &str) -> Result<FixedBytes<32>, String> {
    let bytes = hex::decode(settlement_id).map_err(|err| format!("invalid settlement id hex: {err}"))?;
    if bytes.len() != 32 {
        return Err(format!("expected 32-byte settlement id, got {} bytes", bytes.len()));
    }
    let mut word = [0u8; 32];
    word.copy_from_slice(&bytes);
    Ok(FixedBytes(word))
}

fn encode_address_word(address: Address) -> [u8; 32] {
    let mut word = [0u8; 32];
    word[12..].copy_from_slice(address.as_slice());
    word
}

fn encode_u256_word(value: u64) -> [u8; 32] {
    let mut word = [0u8; 32];
    word[24..].copy_from_slice(&value.to_be_bytes());
    word
}

pub fn encode_create_settlement(settlement_id: &str, beneficiary: Address, amount: u64) -> Vec<u8> {
    let mut calldata = selectors::CREATE_SETTLEMENT.to_vec();
    calldata.extend_from_slice(settlement_id_word(settlement_id).expect("invalid settlement id").as_slice());
    calldata.extend_from_slice(&encode_address_word(beneficiary));
    calldata.extend_from_slice(&encode_u256_word(amount));
    calldata
}

pub fn encode_get_amount(settlement_id: &str) -> Vec<u8> {
    let mut calldata = selectors::GET_AMOUNT.to_vec();
    calldata.extend_from_slice(settlement_id_word(settlement_id).expect("invalid settlement id").as_slice());
    calldata
}

pub fn encode_release_settlement(settlement_id: &str) -> Vec<u8> {
    let mut calldata = selectors::RELEASE_SETTLEMENT.to_vec();
    calldata.extend_from_slice(settlement_id_word(settlement_id).expect("invalid settlement id").as_slice());
    calldata
}

pub fn encode_is_released(settlement_id: &str) -> Vec<u8> {
    let mut calldata = selectors::IS_RELEASED.to_vec();
    calldata.extend_from_slice(settlement_id_word(settlement_id).expect("invalid settlement id").as_slice());
    calldata
}

pub fn decode_u64_word(bytes: &[u8]) -> Result<u64, String> {
    if bytes.len() < 32 {
        return Err(format!("expected 32-byte word, got {} bytes", bytes.len()));
    }
    let mut word = [0u8; 8];
    word.copy_from_slice(&bytes[24..32]);
    Ok(u64::from_be_bytes(word))
}

pub fn decode_bool_word(bytes: &[u8]) -> Result<bool, String> {
    Ok(decode_u64_word(bytes)? > 0)
}

pub fn settlement_source_hash() -> String {
    let mut hasher = Keccak256::new();
    hasher.update(SETTLEMENT_ESCROW_SOLIDITY_SOURCE.as_bytes());
    hex::encode(hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn settlement_bytecode_decodes() {
        let bytecode = get_settlement_escrow_bytecode();
        assert!(!bytecode.is_empty());
    }

    #[test]
    fn counter_helpers_still_work() {
        assert_eq!(encode_increment(), vec![0xd0, 0x9d, 0xe0, 0x8a]);
        assert_eq!(encode_get(), vec![0x06, 0x66, 0x1a, 0xbd]);
    }

    #[test]
    fn settlement_seed_is_stable() {
        assert_eq!(settlement_id_from_seed("req-1").len(), 64);
    }

    #[test]
    fn settlement_source_hash_matches_build_env() {
        assert_eq!(settlement_source_hash(), env!("SETTLEMENT_ESCROW_SOURCE_HASH"));
    }

    #[test]
    fn create_settlement_calldata_has_selector_and_args() {
        let calldata = encode_create_settlement(
            &settlement_id_from_seed("req-1"),
            Address::repeat_byte(0x11),
            7,
        );
        assert_eq!(&calldata[..4], selectors::CREATE_SETTLEMENT.as_slice());
        assert_eq!(calldata.len(), 4 + 32 + 32 + 32);
    }
}

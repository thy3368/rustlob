const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
const FNV_PRIME: u64 = 0x100000001b3;

pub(crate) fn stable_hash_hex(parts: &[impl AsRef<str>]) -> String {
    let mut hash = FNV_OFFSET_BASIS;
    for part in parts {
        for byte in part.as_ref().as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(FNV_PRIME);
        }
        hash ^= u64::from(0xff_u8);
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    format!("{hash:016x}")
}

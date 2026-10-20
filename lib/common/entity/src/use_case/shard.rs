pub trait Shard {
    fn topic(&self) -> u8 {
        0
    }
    fn partition(&self) -> u8;

    fn party_id(&self) -> u64 {
        0
    }
}

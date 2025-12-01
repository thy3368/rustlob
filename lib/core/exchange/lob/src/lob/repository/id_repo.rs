pub struct IdRepo {
    /// 事件序列号计数器
    pub(crate) event_id_counter: u64,
    /// 事务ID计数器
    pub(crate) transaction_id_counter: u64,
}

impl IdRepo {
    pub(crate) fn allocate_event_id(&mut self) -> u64 {
        self.event_id_counter += 1;
        self.event_id_counter
    }
}

impl IdRepo {
    pub(crate) fn allocate_transaction_id(&mut self) -> u64 {
        self.transaction_id_counter += 1;
        self.transaction_id_counter
    }
}

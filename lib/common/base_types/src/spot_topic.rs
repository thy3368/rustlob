pub enum SpotTopic {
    OrderChangeLog,
    TradeChangeLog,
    BalanceChangeLog,
    KLineChangeLog,
}

impl SpotTopic {
    pub fn name(&self) -> &'static str {
        match self {
            SpotTopic::OrderChangeLog => "OrderChangeLog",
            SpotTopic::KLineChangeLog => "KLineChangeLog",
            SpotTopic::TradeChangeLog => "TradeChangeLog",
            SpotTopic::BalanceChangeLog => "BalanceChangeLog",
        }
    }
}

pub enum SpotTopic {
    OrderChangeLog,
    TradeChangeLog,
    BalanceChangeLog,
    KLineChangeLog,
    KUserDataChangeLog,
    KMarketChangeLog,
}

impl SpotTopic {
    pub fn name(&self) -> &'static str {
        match self {
            SpotTopic::OrderChangeLog => "OrderChangeLog",
            SpotTopic::KLineChangeLog => "KLineChangeLog",
            SpotTopic::TradeChangeLog => "TradeChangeLog",
            SpotTopic::BalanceChangeLog => "BalanceChangeLog",
            SpotTopic::KUserDataChangeLog => "KUserDataChangeLog",
            SpotTopic::KMarketChangeLog => "KMarketChangeLog",
        }
    }
}

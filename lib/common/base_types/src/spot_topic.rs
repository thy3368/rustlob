pub enum SpotTopic {
    EntityChangeLog,
    KLine,
    Other,
}

impl SpotTopic {
    pub fn name(&self) -> &'static str {
        match self {
            SpotTopic::EntityChangeLog => "entity_change_log",
            SpotTopic::KLine => "kline",
            SpotTopic::Other => "other",
        }
    }
}

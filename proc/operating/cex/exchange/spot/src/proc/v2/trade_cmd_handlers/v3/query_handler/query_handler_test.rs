#[cfg(test)]
mod tests {
    use crate::proc::behavior::v2::spot_trade_error::CMetadata;
    use crate::proc::behavior::v2::spot_user_data_behavior::QueryOrderCmd;

    #[test]
    fn test_query_cmd_fields() {
        let cmd = QueryOrderCmd {
            metadata: CMetadata::default(),
            symbol: "BTCUSDT".to_string(),
            order_id: Some(123),
            orig_client_order_id: None,
        };
        assert!(cmd.order_id.is_some());
        assert!(cmd.symbol == "BTCUSDT");
    }

    #[test]
    fn test_query_cmd_with_client_id() {
        let cmd = QueryOrderCmd {
            metadata: CMetadata::default(),
            symbol: "ETHUSDT".to_string(),
            order_id: None,
            orig_client_order_id: Some("client_abc".to_string()),
        };
        assert!(cmd.orig_client_order_id.is_some());
    }
}

use crate::proc::trading_prep_order_proc::{CancelAllOrdersCommand, CancelAllOrdersResult, CancelOrderCommand, CancelOrderResult, ClosePositionCommand, ClosePositionResult, ModifyOrderCommand, ModifyOrderResult, OpenPositionCommand, OpenPositionResult, OrderBookSnapshot, OrderQueryResult, PerpOrderExchangeProc, PositionInfo, PrepCommandError, QueryOrderBookCommand, QueryOrderCommand, QueryPositionCommand, QueryTradesCommand, TradesQueryResult};

pub struct MatchingService {
    
}

impl PerpOrderExchangeProc for MatchingService {
    fn handle_open_position(&mut self, cmd: OpenPositionCommand) -> Result<OpenPositionResult, PrepCommandError> {
        todo!()
    }

    fn handle_close_position(&mut self, cmd: ClosePositionCommand) -> Result<ClosePositionResult, PrepCommandError> {
        todo!()
    }

    fn cancel_order(&mut self, cmd: CancelOrderCommand) -> Result<CancelOrderResult, PrepCommandError> {
        todo!()
    }

    fn query_order(&self, cmd: QueryOrderCommand) -> Result<OrderQueryResult, PrepCommandError> {
        todo!()
    }

    fn query_position(&self, cmd: QueryPositionCommand) -> Result<PositionInfo, PrepCommandError> {
        todo!()
    }

    fn modify_order(&mut self, cmd: ModifyOrderCommand) -> Result<ModifyOrderResult, PrepCommandError> {
        todo!()
    }

    fn cancel_all_orders(&mut self, cmd: CancelAllOrdersCommand) -> Result<CancelAllOrdersResult, PrepCommandError> {
        todo!()
    }

    fn query_order_book(&self, cmd: QueryOrderBookCommand) -> Result<OrderBookSnapshot, PrepCommandError> {
        todo!()
    }

    fn query_trades(&self, cmd: QueryTradesCommand) -> Result<TradesQueryResult, PrepCommandError> {
        todo!()
    }
}
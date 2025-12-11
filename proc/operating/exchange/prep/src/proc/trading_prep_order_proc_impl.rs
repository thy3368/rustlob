use crate::proc::trading_prep_order_proc::{AccountBalance, AccountInfo, CancelAllOrdersCommand, CancelAllOrdersResult, CancelOrderCommand, CancelOrderResult, ClosePositionCommand, ClosePositionResult, FundingFeeRecord, FundingRateRecord, MarkPriceInfo, ModifyOrderCommand, ModifyOrderResult, OpenPositionCommand, OpenPositionResult, OrderBookSnapshot, OrderQueryResult, PerpOrderExchQueryProc, PerpOrderExchangeProc, PositionInfo, PrepCommandError, QueryAccountBalanceCommand, QueryAccountInfoCommand, QueryFundingFeeCommand, QueryFundingRateHistoryCommand, QueryMarkPriceCommand, QueryOrderBookCommand, QueryOrderCommand, QueryPositionCommand, QueryTradesCommand, SetLeverageCommand, SetLeverageResult, SetMarginTypeCommand, SetMarginTypeResult, SetPositionModeCommand, SetPositionModeResult, TradesQueryResult};

pub struct MatchingService {

}

impl PerpOrderExchangeProc for MatchingService {
    fn open_position(&mut self, cmd: OpenPositionCommand) -> Result<OpenPositionResult, PrepCommandError> {
        todo!()
    }

    fn close_position(&mut self, cmd: ClosePositionCommand) -> Result<ClosePositionResult, PrepCommandError> {
        todo!()
    }

    fn cancel_order(&mut self, cmd: CancelOrderCommand) -> Result<CancelOrderResult, PrepCommandError> {
        todo!()
    }


    fn modify_order(&mut self, cmd: ModifyOrderCommand) -> Result<ModifyOrderResult, PrepCommandError> {
        todo!()
    }

    fn cancel_all_orders(&mut self, cmd: CancelAllOrdersCommand) -> Result<CancelAllOrdersResult, PrepCommandError> {
        todo!()
    }


    fn set_leverage(&mut self, cmd: SetLeverageCommand) -> Result<SetLeverageResult, PrepCommandError> {
        todo!()
    }

    fn set_margin_type(&mut self, cmd: SetMarginTypeCommand) -> Result<SetMarginTypeResult, PrepCommandError> {
        todo!()
    }

    fn set_position_mode(&mut self, cmd: SetPositionModeCommand) -> Result<SetPositionModeResult, PrepCommandError> {
        todo!()
    }

}

impl PerpOrderExchQueryProc for MatchingService {
    fn query_order(&self, cmd: QueryOrderCommand) -> Result<OrderQueryResult, PrepCommandError> {
        todo!()
    }

    fn query_position(&self, cmd: QueryPositionCommand) -> Result<PositionInfo, PrepCommandError> {
        todo!()
    }

    fn query_order_book(&self, cmd: QueryOrderBookCommand) -> Result<OrderBookSnapshot, PrepCommandError> {
        todo!()
    }

    fn query_trades(&self, cmd: QueryTradesCommand) -> Result<TradesQueryResult, PrepCommandError> {
        todo!()
    }

    fn query_account_balance(&self, cmd: QueryAccountBalanceCommand) -> Result<Vec<AccountBalance>, PrepCommandError> {
        todo!()
    }

    fn query_account_info(&self, cmd: QueryAccountInfoCommand) -> Result<AccountInfo, PrepCommandError> {
        todo!()
    }

    fn query_mark_price(&self, cmd: QueryMarkPriceCommand) -> Result<Vec<MarkPriceInfo>, PrepCommandError> {
        todo!()
    }

    fn query_funding_rate_history(&self, cmd: QueryFundingRateHistoryCommand) -> Result<Vec<FundingRateRecord>, PrepCommandError> {
        todo!()
    }

    fn query_funding_fee(&self, cmd: QueryFundingFeeCommand) -> Result<Vec<FundingFeeRecord>, PrepCommandError> {
        todo!()
    }
}
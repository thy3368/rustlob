use base_types::handler::handler_update::ApplyCommandChanges;

use super::shared::{build_spot_order, match_spot_order};
use crate::cmd_handler::execute_trading_batch::context::{SpotCommandChangeSet, SpotCommandState};
use crate::cmd_handler::execute_trading_batch::{ExecuteTradingBatchError, RestingSpotOrder};
use crate::cmd_handler::execute_trading_batch_handler::{ExecutedBatchBlock, ExecutedOrder};
use crate::cmd_handler::{OrderType, SpotPlaceOrderCmd};

pub(super) struct PlaceOrderApplier;

impl<'a>
    ApplyCommandChanges<
        SpotPlaceOrderCmd,
        SpotCommandState<'a>,
        ExecutedBatchBlock,
        crate::cmd_handler::TradeExecutionLog,
        ExecuteTradingBatchError,
    > for PlaceOrderApplier
{
    //规则：分成两阶段，1 撮合，更新order,trade的状态 2 结算 通过trade计算 balance的状态。 3 返回变更的状态及changelog/domain<xxxx>

    fn apply_command_and_collect_changes(
        &self,
        command: &SpotPlaceOrderCmd,
        state: SpotCommandState<'a>,
    ) -> Result<SpotCommandChangeSet, ExecuteTradingBatchError> {
        let mut writes = ExecutedBatchBlock {
            summary: crate::cmd_handler::BatchExecutionSummary {
                accepted_commands: 1,
                orders_created: 1,
                ..Default::default()
            },
            ..Default::default()
        };
        let mut changelogs = Vec::new();

        let mut resting_order = RestingSpotOrder {
            order_id: state.envelope.command_id,
            trader_id: state.envelope.trader_id,
            market: command.market.clone(),
            side: command.side.clone(),
            price: command.price,
            original_quantity: command.quantity,
            remaining_quantity: command.quantity,
        };

        match_spot_order(
            state.handler,
            state.spot_order_book,
            &mut resting_order,
            &mut writes,
            &mut changelogs,
            command.order_type,
        )?;

        if resting_order.remaining_quantity > 0 {
            state
                .spot_order_book
                .entry(resting_order.market.clone())
                .or_default()
                .push(resting_order.clone());
            changelogs.push(crate::cmd_handler::TradeExecutionLog::SpotRestingOrderUpserted {
                market: resting_order.market.clone(),
                order: resting_order.clone(),
            });
        }

        let spot_order = build_spot_order(state.envelope, command, &resting_order)?;
        writes.orders.push(ExecutedOrder::SpotOrder(spot_order));
        writes.summary.balance_updates = writes.balance_deltas.len();

        Ok(SpotCommandChangeSet { writes, changelogs })
    }
}

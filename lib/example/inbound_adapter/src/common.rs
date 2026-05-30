use cmd_handler::EntityReplayableEvent;
use cmd_handler::use_case_def2::{
    CommandEnvelope, CommandUseCase2, CommandUseCaseExecutor2, CommandUseCaseOutbound,
    UseCaseReplyMapper,
};
use example_core::{
    DepositQuoteCmd, DepositQuoteError, DepositQuoteState, DepositQuoteUseCase, PlaceOrderCmd,
    PlaceOrderError, PlaceOrderState, PlaceOrderUseCase, WithdrawQuoteCmd, WithdrawQuoteError,
    WithdrawQuoteState, WithdrawQuoteUseCase,
};

pub(crate) fn execute_with_mapper<U, OB, M>(
    use_case: &U,
    envelope: CommandEnvelope<U::Command>,
    outbound: &OB,
    mapper: &M,
) -> Result<M::Reply, U::Error>
where
    U: CommandUseCase2,
    OB: ?Sized + Send + Sync + CommandUseCaseOutbound<U::Command, U::GivenState, U::Error>,
    M: UseCaseReplyMapper,
{
    let executor = CommandUseCaseExecutor2;
    executor.execute_and_map_reply(use_case, envelope, outbound, &(), mapper)
}

pub(crate) fn execute_place_order_with_mapper<OB, M>(
    envelope: CommandEnvelope<PlaceOrderCmd>,
    outbound: &OB,
    mapper: &M,
) -> Result<M::Reply, PlaceOrderError>
where
    OB: ?Sized
        + Send
        + Sync
        + CommandUseCaseOutbound<PlaceOrderCmd, PlaceOrderState, PlaceOrderError>,
    M: UseCaseReplyMapper,
{
    execute_with_mapper(&PlaceOrderUseCase, envelope, outbound, mapper)
}

pub(crate) fn execute_deposit_quote_with_mapper<OB, M>(
    envelope: CommandEnvelope<DepositQuoteCmd>,
    outbound: &OB,
    mapper: &M,
) -> Result<M::Reply, DepositQuoteError>
where
    OB: ?Sized
        + Send
        + Sync
        + CommandUseCaseOutbound<DepositQuoteCmd, DepositQuoteState, DepositQuoteError>,
    M: UseCaseReplyMapper,
{
    execute_with_mapper(&DepositQuoteUseCase, envelope, outbound, mapper)
}

pub(crate) fn execute_withdraw_quote_with_mapper<OB, M>(
    envelope: CommandEnvelope<WithdrawQuoteCmd>,
    outbound: &OB,
    mapper: &M,
) -> Result<M::Reply, WithdrawQuoteError>
where
    OB: ?Sized
        + Send
        + Sync
        + CommandUseCaseOutbound<WithdrawQuoteCmd, WithdrawQuoteState, WithdrawQuoteError>,
    M: UseCaseReplyMapper,
{
    execute_with_mapper(&WithdrawQuoteUseCase, envelope, outbound, mapper)
}

pub(crate) fn find_string_field(
    events: &[EntityReplayableEvent],
    field_name: &str,
) -> Option<String> {
    events.iter().find_map(|event| {
        event.field_changes.iter().find_map(|change| {
            if change.field_name_as_str().ok() != Some(field_name) {
                return None;
            }

            Some(String::from_utf8_lossy(change.new_value_bytes()).to_string())
        })
    })
}

pub(crate) fn find_u64_field(events: &[EntityReplayableEvent], field_name: &str) -> Option<u64> {
    events.iter().find_map(|event| {
        event.field_changes.iter().find_map(|change| {
            if change.field_name_as_str().ok() != Some(field_name) {
                return None;
            }

            std::str::from_utf8(change.new_value_bytes()).ok()?.parse::<u64>().ok()
        })
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use std::sync::{Mutex, MutexGuard};

    use cmd_handler::EntityReplayableEvent;
    use cmd_handler::use_case_def2::CommandUseCaseOutbound;
    use example_core::{
        DepositQuoteCmd, DepositQuoteError, DepositQuoteState, MarketRules, PlaceOrderCmd,
        PlaceOrderError, PlaceOrderState, TradingAccount, WithdrawQuoteCmd, WithdrawQuoteError,
        WithdrawQuoteState,
    };

    #[derive(Debug, Default)]
    pub(crate) struct TestOutbound {
        persisted_events: Mutex<Vec<EntityReplayableEvent>>,
        published_events: Mutex<Vec<EntityReplayableEvent>>,
    }

    impl TestOutbound {
        pub(crate) fn snapshot_event_counts(&self) -> Result<(usize, usize), PlaceOrderError> {
            let persisted =
                self.persisted_events.lock().map_err(|_| PlaceOrderError::StoreUnavailable)?.len();
            let published =
                self.published_events.lock().map_err(|_| PlaceOrderError::StoreUnavailable)?.len();
            Ok((persisted, published))
        }

        fn lock_events(
            mutex: &Mutex<Vec<EntityReplayableEvent>>,
        ) -> Result<MutexGuard<'_, Vec<EntityReplayableEvent>>, PlaceOrderError> {
            mutex.lock().map_err(|_| PlaceOrderError::StoreUnavailable)
        }
    }

    impl CommandUseCaseOutbound<PlaceOrderCmd, PlaceOrderState, PlaceOrderError> for TestOutbound {
        fn load_state(&self, _cmd: &PlaceOrderCmd) -> Result<PlaceOrderState, PlaceOrderError> {
            Ok(PlaceOrderState {
                trading_enabled: true,
                next_order_sequence: 11,
                account: TradingAccount {
                    account_id: "trader-1".to_string(),
                    available_quote: 1_000,
                    frozen_quote: 0,
                    version: 5,
                },
                market_rules: MarketRules { symbol: "BTCUSDT".to_string(), min_qty: 1 },
            })
        }

        fn persist(&self, events: &[EntityReplayableEvent]) -> Result<(), PlaceOrderError> {
            let mut persisted = Self::lock_events(&self.persisted_events)?;
            persisted.extend(events.iter().cloned());
            Ok(())
        }

        fn replay(&self, _events: &[EntityReplayableEvent]) -> Result<(), PlaceOrderError> {
            Ok(())
        }

        fn publish(&self, events: &[EntityReplayableEvent]) -> Result<(), PlaceOrderError> {
            let mut published = Self::lock_events(&self.published_events)?;
            published.extend(events.iter().cloned());
            Ok(())
        }
    }

    impl CommandUseCaseOutbound<DepositQuoteCmd, DepositQuoteState, DepositQuoteError>
        for TestOutbound
    {
        fn load_state(
            &self,
            _cmd: &DepositQuoteCmd,
        ) -> Result<DepositQuoteState, DepositQuoteError> {
            Ok(DepositQuoteState {
                account: TradingAccount {
                    account_id: "trader-1".to_string(),
                    available_quote: 1_000,
                    frozen_quote: 0,
                    version: 5,
                },
            })
        }

        fn persist(&self, events: &[EntityReplayableEvent]) -> Result<(), DepositQuoteError> {
            let mut persisted =
                self.persisted_events.lock().map_err(|_| DepositQuoteError::StoreUnavailable)?;
            persisted.extend(events.iter().cloned());
            Ok(())
        }

        fn replay(&self, _events: &[EntityReplayableEvent]) -> Result<(), DepositQuoteError> {
            Ok(())
        }

        fn publish(&self, events: &[EntityReplayableEvent]) -> Result<(), DepositQuoteError> {
            let mut published =
                self.published_events.lock().map_err(|_| DepositQuoteError::StoreUnavailable)?;
            published.extend(events.iter().cloned());
            Ok(())
        }
    }

    impl CommandUseCaseOutbound<WithdrawQuoteCmd, WithdrawQuoteState, WithdrawQuoteError>
        for TestOutbound
    {
        fn load_state(
            &self,
            _cmd: &WithdrawQuoteCmd,
        ) -> Result<WithdrawQuoteState, WithdrawQuoteError> {
            Ok(WithdrawQuoteState {
                account: TradingAccount {
                    account_id: "trader-1".to_string(),
                    available_quote: 1_000,
                    frozen_quote: 0,
                    version: 5,
                },
            })
        }

        fn persist(&self, events: &[EntityReplayableEvent]) -> Result<(), WithdrawQuoteError> {
            let mut persisted =
                self.persisted_events.lock().map_err(|_| WithdrawQuoteError::StoreUnavailable)?;
            persisted.extend(events.iter().cloned());
            Ok(())
        }

        fn replay(&self, _events: &[EntityReplayableEvent]) -> Result<(), WithdrawQuoteError> {
            Ok(())
        }

        fn publish(&self, events: &[EntityReplayableEvent]) -> Result<(), WithdrawQuoteError> {
            let mut published =
                self.published_events.lock().map_err(|_| WithdrawQuoteError::StoreUnavailable)?;
            published.extend(events.iter().cloned());
            Ok(())
        }
    }
}

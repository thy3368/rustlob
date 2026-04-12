use std::sync::Mutex;

use base_types::base_types::TraderId;
use base_types::exchange::spot::spot_types::{OrderSide, SpotOrder, SpotTrade, TimeInForce};
use cmd_handler::{CmdHandlerForUpdate3, CmdHandlerInternal, DomainEventSet};
use base_types::{Price, Quantity};
use db_repo::core::db_repo2::CmdRepo2;
use db_repo::core::event_publish::EventPublisher2;
use diff::diff_types::DomainEvent;
use diff::Entity;
use lob_repo::core::symbol_lob_repo::MultiSymbolLobRepo;

use crate::proc::behavior::spot_trade_behavior::{CommonError, SpotCmdErrorAny};

#[derive(Debug, Clone)]
pub struct MatchOrderStateSet {
    pub taker: SpotOrder,
    pub makers: Vec<SpotOrder>,
}

pub struct MatchOrderStateChangedSet {
    pub taker: DomainEvent<SpotOrder>,
    pub makers: Option<Vec<DomainEvent<SpotOrder>>>,
    pub trades: Option<Vec<DomainEvent<SpotTrade>>>,
}

impl DomainEventSet for MatchOrderStateChangedSet {
    #[inline]
    fn domain_event_count(&self) -> usize {
        self.trades.as_ref().map(|trades| trades.len()).unwrap_or(0)
    }
}

pub struct MatchCmd {
    pub taker_order: SpotOrder,
}

pub struct MatchOrderCmdHandler<
    R: CmdRepo2,
    P: EventPublisher2,
    L: MultiSymbolLobRepo<Order = SpotOrder> + Send,
> {
    pub repo: R,
    pub publisher: P,
    pub lob: Mutex<L>,
}

impl<R: CmdRepo2, P: EventPublisher2, L: MultiSymbolLobRepo<Order = SpotOrder> + Send>
    MatchOrderCmdHandler<R, P, L>
{
    pub fn new(repo: R, publisher: P, lob: L) -> Self {
        Self { repo, publisher, lob: Mutex::new(lob) }
    }
}

impl<R: CmdRepo2, P: EventPublisher2, L: MultiSymbolLobRepo<Order = SpotOrder> + Send> CmdHandlerInternal
    for MatchOrderCmdHandler<R, P, L>
{
    type Command = MatchCmd;
    type Reply = Option<Vec<DomainEvent<SpotTrade>>>;
    type GivenStateSet = MatchOrderStateSet;
    type ThenStateSet = MatchOrderStateChangedSet;
    type Error = SpotCmdErrorAny;

    type Repo = R;
    type Publisher = P;

    fn then(
        &self,
        _cmd: &Self::Command,
        state_set: Self::GivenStateSet,
    ) -> Result<Self::ThenStateSet, Self::Error> {
        if state_set.makers.is_empty() {
            return Ok(MatchOrderStateChangedSet {
                taker: DomainEvent::new(
                    state_set.taker.track_create().map_err(|e| {
                        SpotCmdErrorAny::Common(CommonError::Internal { message: e.to_string() })
                    })?,
                    state_set.taker,
                ),
                makers: None,
                trades: None,
            });
        }

        let mut remaining_qty = state_set.taker.unfilled_qty();
        let mut trade_events = Vec::new();

        for (index, maker) in state_set.makers.iter().enumerate() {
            if remaining_qty.is_zero() {
                break;
            }

            let fill_qty = remaining_qty.min(maker.unfilled_qty());
            if fill_qty.is_zero() {
                continue;
            }

            let trade = SpotTrade::new(
                state_set.taker.order_id + index as u64,
                state_set.taker.trading_pair,
                state_set.taker.order_id,
                maker.order_id,
                state_set.taker.timestamp,
                maker.price.expect("maker limit order should have price"),
                fill_qty,
                state_set.taker.side,
                Quantity::default(),
                Quantity::default(),
                state_set.taker.frozen_asset(),
                0,
                0,
            );
            let trade_event = DomainEvent::new(
                trade.track_create().map_err(|e| {
                    SpotCmdErrorAny::Common(CommonError::Internal { message: e.to_string() })
                })?,
                trade,
            );
            trade_events.push(trade_event);
            remaining_qty -= fill_qty;
        }

        let taker_event = DomainEvent::new(
            state_set.taker.track_create().map_err(|e| {
                SpotCmdErrorAny::Common(CommonError::Internal { message: e.to_string() })
            })?,
            state_set.taker,
        );
        let maker_events: Result<Vec<_>, Self::Error> = state_set
            .makers
            .into_iter()
            .map(|maker| {
                Ok(DomainEvent::new(
                    maker.track_create().map_err(|e| {
                        SpotCmdErrorAny::Common(CommonError::Internal { message: e.to_string() })
                    })?,
                    maker,
                ))
            })
            .collect();
        let maker_events = maker_events?;

        if trade_events.is_empty() {
            Ok(MatchOrderStateChangedSet {
                taker: taker_event,
                makers: Some(maker_events),
                trades: None,
            })
        } else {
            Ok(MatchOrderStateChangedSet {
                taker: taker_event,
                makers: Some(maker_events),
                trades: Some(trade_events),
            })
        }
    }

    fn state_changed_set_to_reply(&self, state_changed_set: Self::ThenStateSet) -> Self::Reply {
        state_changed_set.trades
    }
    fn pre_check_command(&self, _cmd: &Self::Command) -> Result<(), Self::Error> {
        Ok(())
    }

    fn give(
        &self,
        cmd: &Self::Command,
        _repo: &Self::Repo,
    ) -> Result<Self::GivenStateSet, Self::Error> {
        let taker = cmd.taker_order.clone();
        let taker_price = taker.price.unwrap_or_default();
        let mut lob = self.lob.lock().expect("match lob lock poisoned");
        let (matched, _) = lob.match_orders(
            taker.trading_pair,
            taker.side,
            taker_price,
            taker.unfilled_qty(),
        );
        let makers: Vec<SpotOrder> =
            matched.map(|v| v.into_iter().cloned().collect()).unwrap_or_default();

        Ok(MatchOrderStateSet { taker, makers })
    }

    fn validate_command_in_lock(
        &self,
        _cmd: &Self::Command,
        _state_set: &Self::GivenStateSet,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn persist_domain_events(
        &self,
        _domain_events: &Self::ThenStateSet,
        _repo: &Self::Repo,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn replay_domain_events_to_state(
        &self,
        domain_events: &Self::ThenStateSet,
        repo: &Self::Repo,
    ) -> Result<(), Self::Error> {
        if let Some(ref trades) = domain_events.trades {
            for trade_event in trades {
                repo.replay_event::<SpotTrade>(trade_event).map_err(|e| {
                    SpotCmdErrorAny::Common(CommonError::Internal { message: e.to_string() })
                })?;
            }
        }
        Ok(())
    }

    fn publish_domain_events(
        &self,
        domain_events: &Self::ThenStateSet,
        publisher: Self::Publisher,
    ) -> Result<(), Self::Error> {
        if let Some(ref trades) = domain_events.trades {
            publisher.publish_batch(trades).map_err(|_e| {
                SpotCmdErrorAny::Common(CommonError::Internal {
                    message: "publish match order events failed".to_string(),
                })
            })?;
        }
        Ok(())
    }
}

impl<R: CmdRepo2, P: EventPublisher2, L: MultiSymbolLobRepo<Order = SpotOrder> + Send>
    CmdHandlerForUpdate3 for MatchOrderCmdHandler<R, P, L>
{
}

#[cfg(test)]
mod tests {
    use base_types::exchange::spot::spot_types::TradingPair;
    use diff::diff_types::DomainEvent;
    use db_repo::adapter::v2::memdb_repo::MemdbRepo;
    use db_repo::core::db_repo2::QueryRepo2;
    use lob_repo::adapter::embedded_lob_repo::EmbeddedLobRepo;
    use lob_repo::core::symbol_lob_repo::MultiSymbolLobRepo;

    use super::*;

    fn create_order(order_id: u64, side: OrderSide, quantity: f64) -> SpotOrder {
        SpotOrder::create_order(
            order_id,
            TraderId::new([0u8; 8]),
            TradingPair::BtcUsdt,
            side,
            Price::from_f64(50000.0),
            Quantity::from_f64(quantity),
            TimeInForce::GTC,
            Some(format!("test_order_{order_id}")),
            Quantity::default(),
        )
    }

    struct MockEventPublisher;

    impl EventPublisher2 for MockEventPublisher {
        fn publish<E>(
            &self,
            _event: &DomainEvent<E>,
        ) -> Result<(), db_repo::core::event_publish::PublishError> {
            Ok(())
        }

        fn publish_batch<E>(
            &self,
            _events: &[DomainEvent<E>],
        ) -> Result<(), db_repo::core::event_publish::PublishError> {
            Ok(())
        }
    }

    #[test]
    fn test_match_order_apply_returns_no_trades_when_book_is_empty() {
        let repo = MemdbRepo::default();
        let handler = MatchOrderCmdHandler::new(
            repo.clone(),
            MockEventPublisher,
            EmbeddedLobRepo::<SpotOrder>::new(Vec::new()),
        );
        let cmd = MatchCmd { taker_order: create_order(1, OrderSide::Buy, 1.0) };

        let state = handler
            .give(&cmd, &repo)
            .expect("load_state_set_for_update should succeed");
        let changes =
            handler.then(&cmd, state).expect("apply should succeed");

        assert!(changes.trades.is_none());
        assert_eq!(changes.domain_event_count(), 0);
        assert_eq!(repo.count().unwrap(), 0);
    }

    #[test]
    fn test_match_order_apply_generates_two_trades_for_two_makers() {
        let repo = MemdbRepo::default();
        let mut lob = EmbeddedLobRepo::new(vec![lob_repo::adapter::local_lob_impl::LocalLob::new(
            TradingPair::BtcUsdt,
        )]);
        lob.add_order(TradingPair::BtcUsdt, create_order(11, OrderSide::Sell, 1.0))
            .expect("add first maker order should succeed");
        lob.add_order(TradingPair::BtcUsdt, create_order(12, OrderSide::Sell, 1.0))
            .expect("add second maker order should succeed");
        let handler = MatchOrderCmdHandler::new(repo.clone(), MockEventPublisher, lob);
        let cmd = MatchCmd { taker_order: create_order(21, OrderSide::Buy, 2.0) };

        let trades = handler
            .cmd_handle(cmd, repo.clone(), MockEventPublisher)
            .expect("match should succeed")
            .expect("trades should exist");

        assert_eq!(trades.len(), 2);
        assert_ne!(trades[0].object().trade_id, trades[1].object().trade_id);
        assert_eq!(trades[0].object().maker_order_id, 11);
        assert_eq!(trades[1].object().maker_order_id, 12);
        assert_eq!(trades[0].object().taker_order_id, 21);
        assert_eq!(trades[1].object().taker_order_id, 21);
        assert_eq!(trades[0].object().base_qty, Quantity::from_f64(1.0));
        assert_eq!(trades[1].object().base_qty, Quantity::from_f64(1.0));

        let stored_first = repo
            .find_by_id::<SpotTrade>(&trades[0].object().entity_id().to_string())
            .expect("query first trade should succeed")
            .expect("first trade should exist");
        let stored_second = repo
            .find_by_id::<SpotTrade>(&trades[1].object().entity_id().to_string())
            .expect("query second trade should succeed")
            .expect("second trade should exist");
        assert_eq!(stored_first.trade_id, trades[0].object().trade_id);
        assert_eq!(stored_second.trade_id, trades[1].object().trade_id);
    }

    #[test]
    /// bdd: 挂一笔卖单5，再挂一笔买单3，检查卖单应该只余2
    fn test_match_order_apply_generates_two_trades_for_two_makers2() {

    }
}

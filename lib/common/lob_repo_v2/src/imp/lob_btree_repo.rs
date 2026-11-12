use std::fs;
use std::io::Write;
use std::marker::PhantomData;
use std::path::{Path, PathBuf};

use tempfile::NamedTempFile;

use super::lob_btree::LocalLobBtree;
use crate::lob::LobOrder;
use crate::lob_repo::LobRepo;
use crate::LobError;

pub struct LobBtreeRepository<O: LobOrder>(PhantomData<O>);

impl<O: LobOrder + serde::Serialize + for<'de> serde::Deserialize<'de>> LobRepo
    for LobBtreeRepository<O>
{
    type Lob = LocalLobBtree<O>;
    type Order = O;

    fn load_from_snapshot(path: PathBuf) -> Result<Self::Lob, LobError> {
        let bytes = fs::read(path)?;
        bincode::deserialize(&bytes).map_err(|error| LobError::Codec(error.to_string()))
    }

    fn save_to_snapshot(path: PathBuf, lob: Self::Lob) -> Result<(), LobError> {
        let bytes = bincode::serialize(&lob).map_err(|error| LobError::Codec(error.to_string()))?;
        let parent = path
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
            .unwrap_or_else(|| Path::new("."));
        let mut temporary = NamedTempFile::new_in(parent)?;
        temporary.write_all(&bytes)?;
        temporary.as_file().sync_all()?;
        temporary.persist(path).map_err(|error| LobError::Io(error.error))?;
        Ok(())
    }

    fn load_from_orders(orders: Vec<Self::Order>) -> Result<Self::Lob, LobError> {
        ///todo 也可先在这对orders排序，再insert_order
        let symbol = orders
            .first()
            .ok_or_else(|| {
                LobError::Event("cannot infer order book symbol from empty orders".into())
            })?
            .symbol();
        let mut lob = LocalLobBtree::new(symbol);
        for order in orders {
            lob.insert_order(order)?;
        }
        Ok(lob)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use base_types_v2::{Coin, Symbol};
    use common_entity::EntityReplayableEvent;
    use rust_decimal::Decimal;
    use serde::{Deserialize, Serialize};
    use tempfile::tempdir;

    use super::LobBtreeRepository;
    use crate::lob::{AssetLobV2, LobOrder};
    use crate::{LobError, LobRepo};

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    enum Side {
        Buy,
        Sell,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    struct TestOrder {
        id: u64,
        symbol: Symbol,
        side: Side,
        price: Decimal,
        size: Decimal,
    }

    impl TestOrder {
        fn new(id: u64, symbol: Symbol, side: Side, price: Decimal) -> Self {
            Self { id, symbol, side, price, size: Decimal::ONE }
        }
    }

    impl LobOrder for TestOrder {
        type Side = Side;

        fn entity_type() -> u8 {
            1
        }
        fn entity_id(&self) -> u64 {
            self.id
        }
        fn symbol(&self) -> Symbol {
            self.symbol
        }
        fn side(&self) -> Self::Side {
            self.side
        }
        fn is_buy_side(side: Self::Side) -> bool {
            side == Side::Buy
        }
        fn px(&self) -> Decimal {
            self.price
        }
        fn orig_size(&self) -> Decimal {
            self.size
        }
        fn size(&self) -> Decimal {
            self.size
        }
        fn created_at(&self) -> u64 {
            self.id
        }
        fn entity_ver(&self) -> u64 {
            0
        }
        fn is_active(&self) -> bool {
            true
        }
        fn from_replay_event(_: &EntityReplayableEvent, _: u64) -> Result<Self, LobError> {
            unreachable!("repository tests insert orders directly")
        }
        fn apply_replay_event(self, _: &EntityReplayableEvent) -> Result<Self, LobError> {
            unreachable!("repository tests insert orders directly")
        }
    }

    type Repository = LobBtreeRepository<TestOrder>;

    fn symbol(id: u32) -> Symbol {
        Symbol::new(id, Coin::BTC, Coin::USDT)
    }

    fn orders(book_symbol: Symbol) -> Vec<TestOrder> {
        vec![
            TestOrder::new(1, book_symbol, Side::Buy, Decimal::new(100, 0)),
            TestOrder::new(2, book_symbol, Side::Buy, Decimal::new(101, 0)),
            TestOrder::new(3, book_symbol, Side::Sell, Decimal::new(103, 0)),
            TestOrder::new(4, book_symbol, Side::Sell, Decimal::new(102, 0)),
        ]
    }

    #[test]
    fn snapshot_round_trip_preserves_complete_lob_state() {
        let directory = tempdir().unwrap();
        let path = directory.path().join("lob.bin");
        let book_symbol = symbol(7);
        let lob = Repository::load_from_orders(orders(book_symbol)).unwrap();

        Repository::save_to_snapshot(path.clone(), lob).unwrap();
        let loaded = Repository::load_from_snapshot(path).unwrap();

        assert_eq!(loaded.symbol(), book_symbol);
        assert_eq!(loaded.best_bid(), Some(Decimal::new(101, 0)));
        assert_eq!(loaded.best_ask(), Some(Decimal::new(102, 0)));
        assert_eq!(loaded.order(1).map(|order| order.id), Some(1));
        assert_eq!(loaded.order(4).map(|order| order.id), Some(4));
        assert_eq!(loaded.sequence(), 0);
    }

    #[test]
    fn saving_to_existing_snapshot_atomically_replaces_complete_state() {
        let directory = tempdir().unwrap();
        let path = directory.path().join("lob.bin");
        let book_symbol = symbol(7);
        let old_lob = Repository::load_from_orders(vec![TestOrder::new(
            99,
            book_symbol,
            Side::Buy,
            Decimal::new(100, 0),
        )])
        .unwrap();
        Repository::save_to_snapshot(path.clone(), old_lob).unwrap();

        let new_lob = Repository::load_from_orders(orders(book_symbol)).unwrap();
        Repository::save_to_snapshot(path.clone(), new_lob).unwrap();
        let loaded = Repository::load_from_snapshot(path).unwrap();

        assert_eq!(loaded.best_bid(), Some(Decimal::new(101, 0)));
        assert_eq!(loaded.best_ask(), Some(Decimal::new(102, 0)));
        assert!(loaded.order(99).is_none());
        assert_eq!(loaded.order(4).map(|order| order.id), Some(4));
    }

    #[test]
    fn missing_snapshot_returns_io_error() {
        let directory = tempdir().unwrap();
        let error =
            Repository::load_from_snapshot(directory.path().join("missing.bin")).unwrap_err();

        assert!(
            matches!(error, LobError::Io(error) if error.kind() == std::io::ErrorKind::NotFound)
        );
    }

    #[test]
    fn corrupt_snapshot_returns_codec_error() {
        let directory = tempdir().unwrap();
        let path = directory.path().join("lob.bin");
        fs::write(&path, b"not a lob snapshot").unwrap();

        assert!(matches!(Repository::load_from_snapshot(path), Err(LobError::Codec(_))));
    }

    #[test]
    fn active_orders_rebuild_symbol_and_best_prices() {
        let book_symbol = symbol(7);
        let lob = Repository::load_from_orders(orders(book_symbol)).unwrap();

        assert_eq!(lob.symbol(), book_symbol);
        assert_eq!(lob.best_bid(), Some(Decimal::new(101, 0)));
        assert_eq!(lob.best_ask(), Some(Decimal::new(102, 0)));
    }

    #[test]
    fn empty_orders_return_event_error() {
        assert!(matches!(
            Repository::load_from_orders(vec![]),
            Err(LobError::Event(message)) if message.contains("empty orders")
        ));
    }

    #[test]
    fn mixed_symbols_return_event_error() {
        let mut active_orders = orders(symbol(7));
        active_orders.push(TestOrder::new(5, symbol(8), Side::Buy, Decimal::new(99, 0)));

        assert!(matches!(
            Repository::load_from_orders(active_orders),
            Err(LobError::Event(message)) if message == "wrong order book"
        ));
    }
}

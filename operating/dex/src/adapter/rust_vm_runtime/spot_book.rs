use std::collections::BTreeMap;
use std::path::Path;
use std::sync::Arc;

use db_repo::{KvStore, StorageError};
use mdbx::MdbxKvStore;

use crate::core::SpotSide;
use crate::core::use_case::execute_trading_batch::{RestingSpotOrder, SpotOrderBook};

const SPOT_ORDER_BOOK_KEY: &[u8] = b"spot_order_book";

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct RestingSpotOrderSnapshot {
    pub order_id: u64,
    pub trader_id: u64,
    pub market: String,
    pub side: SpotSide,
    pub price: u64,
    pub original_quantity: u64,
    pub remaining_quantity: u64,
}

impl From<RestingSpotOrder> for RestingSpotOrderSnapshot {
    fn from(value: RestingSpotOrder) -> Self {
        Self {
            order_id: value.order_id,
            trader_id: value.trader_id,
            market: value.market,
            side: value.side,
            price: value.price,
            original_quantity: value.original_quantity,
            remaining_quantity: value.remaining_quantity,
        }
    }
}

impl From<RestingSpotOrderSnapshot> for RestingSpotOrder {
    fn from(value: RestingSpotOrderSnapshot) -> Self {
        Self {
            order_id: value.order_id,
            trader_id: value.trader_id,
            market: value.market,
            side: value.side,
            price: value.price,
            original_quantity: value.original_quantity,
            remaining_quantity: value.remaining_quantity,
        }
    }
}

pub type SpotOrderBookSnapshot = BTreeMap<String, Vec<RestingSpotOrderSnapshot>>;

pub trait SpotOrderBookRepository: Send + Sync {
    fn load(&self) -> Result<SpotOrderBookSnapshot, String>;
    fn save(&self, snapshot: &SpotOrderBookSnapshot) -> Result<(), String>;
}

pub struct MdbxSpotOrderBookRepository {
    store: Arc<MdbxKvStore>,
}

impl MdbxSpotOrderBookRepository {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, StorageError> {
        Ok(Self { store: Arc::new(MdbxKvStore::open(path, "dex_spot_product_state")?) })
    }

    fn decode_snapshot(bytes: &[u8]) -> Result<SpotOrderBookSnapshot, String> {
        serde_json::from_slice(bytes).map_err(|err| err.to_string())
    }

    fn encode_snapshot(snapshot: &SpotOrderBookSnapshot) -> Result<Vec<u8>, String> {
        serde_json::to_vec(snapshot).map_err(|err| err.to_string())
    }
}

impl SpotOrderBookRepository for MdbxSpotOrderBookRepository {
    fn load(&self) -> Result<SpotOrderBookSnapshot, String> {
        self.store
            .get(SPOT_ORDER_BOOK_KEY)
            .map_err(|err| err.to_string())?
            .map(|bytes| Self::decode_snapshot(&bytes))
            .transpose()
            .map(|snapshot| snapshot.unwrap_or_default())
    }

    fn save(&self, snapshot: &SpotOrderBookSnapshot) -> Result<(), String> {
        let bytes = Self::encode_snapshot(snapshot)?;
        self.store.put(SPOT_ORDER_BOOK_KEY, &bytes).map_err(|err| err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicU64, Ordering};

    use super::*;

    static NEXT_TEST_ID: AtomicU64 = AtomicU64::new(1);

    fn next_state_path() -> std::path::PathBuf {
        let test_id = NEXT_TEST_ID.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir().join(format!("rustlob-dex-spot-book-{test_id}"))
    }

    #[test]
    fn reloads_saved_snapshot() {
        let state_path = next_state_path();
        let _ = std::fs::remove_dir_all(&state_path);
        std::fs::create_dir_all(&state_path).unwrap();
        let repository = MdbxSpotOrderBookRepository::open(&state_path).unwrap();
        let snapshot = BTreeMap::from([(
            "BTC-USDT".to_string(),
            vec![RestingSpotOrderSnapshot {
                order_id: 1,
                trader_id: 2,
                market: "BTC-USDT".to_string(),
                side: SpotSide::Buy,
                price: 100_000,
                original_quantity: 1,
                remaining_quantity: 1,
            }],
        )]);

        repository.save(&snapshot).unwrap();

        assert_eq!(repository.load().unwrap(), snapshot);
    }
}

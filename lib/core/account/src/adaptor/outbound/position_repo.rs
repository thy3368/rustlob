//! 内存实现的持仓仓储

use std::collections::HashMap;

use crate::domain::repo::{Position, PositionRepo};

/// 内存持仓仓储实现（泛型版本）
///
/// # 类型参数
/// - `P`: 持仓类型，必须实现 Position trait
pub struct MemoryPositionRepo<P: Position> {
    /// 持仓缓存 (Key -> Position)
    positions: HashMap<P::Key, P>
}

impl<P: Position> MemoryPositionRepo<P> {
    /// 创建新的内存持仓仓储
    pub fn new() -> Self {
        Self {
            positions: HashMap::new()
        }
    }
}

impl<P: Position> Default for MemoryPositionRepo<P> {
    fn default() -> Self { Self::new() }
}

impl<P: Position> PositionRepo<P> for MemoryPositionRepo<P> {
    fn get(&self, key: P::Key) -> Option<&P> { self.positions.get(&key) }

    fn get_mut(&mut self, key: P::Key) -> Option<&mut P> { self.positions.get_mut(&key) }

    fn save(&mut self, position: P) {
        let key = position.key();
        self.positions.insert(key, position);
    }

    fn remove(&mut self, key: P::Key) -> bool { self.positions.remove(&key).is_some() }

    fn get_all(&self) -> Vec<P> { self.positions.values().cloned().collect() }
}

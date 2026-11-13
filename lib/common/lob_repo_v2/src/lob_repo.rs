use std::path::PathBuf;

use crate::LobError;
use crate::lob::LobOrder;

pub trait LobRepo {
    type Lob;
    type Order: LobOrder;

    ///从path加载lob 从快照恢复
    fn load_from_snapshot(path: PathBuf) -> Result<Self::Lob, LobError>;

    ///lob持久化 保存快照
    fn save_to_snapshot(path: PathBuf, lob: Self::Lob) -> Result<(), LobError>;

    ///直接从active order重建lob，orders可能是无序
    fn load_from_orders(orders: Vec<Self::Order>) -> Result<Self::Lob, LobError>;
}

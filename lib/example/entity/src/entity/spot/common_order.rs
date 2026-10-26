use serde::{Deserialize, Serialize};

use crate::{OrderGroupRelation, OrderSide, OrderTif, OrderType, SpotOrderStatus};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommonOrder {
    /// Hyperliquid 原生现货订单 `oid`。
    ///
    /// 字段名保留 `order_id` 以兼容本仓库内实体接口语义，但业务含义等同
    /// Hyperliquid numeric spot `oid`。
    pub order_id: u64,
    /// Hyperliquid 现货资产编号，现货通常为 `10000 + spot index`。
    pub asset: u32,

    /// 交易对展示名，例如 `BTCUSDT`。业务身份以 `asset` 为准。
    pub pair: String,
    /// 买卖方向。
    pub side: OrderSide,
    /// Hyperliquid `p` 字段；market 意图同样保存其激进限价。
    pub limit_px: u64,
    /// 以 base asset 计价的下单数量。
    pub size: u64,
    /// 已成交数量。
    pub orig_size: u64,

    /// 统一的 Hyperliquid 订单类型事实。
    pub order_type: OrderType,

    pub tif: OrderTif,


    // "triggerCondition": "N/A",
    // "children": [],



    // order.isPositionTpsl*
    // boolean
    // Whether this is a position take-profit or stop-loss order.
    pub is_position_tpsl: bool,
    pub reduce_only: bool,
    pub is_trigger: bool,
    pub trigger_px: u64,

    /// `normalTpsl` 父子关系；历史订单反序列化时默认为独立订单。
    pub group_relation: OrderGroupRelation,
    /// 本地生命周期状态。
    pub status: SpotOrderStatus,

    // /// 是否仅允许减少既有敞口。实体保存意图，不在订单内模拟仓位。 perp特有
    // pub reduce_only: bool,
    /// 冻结费 spot 特有
    pub taker_max_fee: u64,
    /// Hyperliquid `cloid`，客户端自定义订单 ID。
    pub cloid: Option<String>,
    /// 订单创建时间，单位为 Unix 纳秒。
    pub created_at: u64,
    /// 订单最后更新时间，单位为 Unix 纳秒。
    pub updated_at: u64,
    /// 当前订单实体版本，用于生成可重放更新事件。
    pub version: u64,
}

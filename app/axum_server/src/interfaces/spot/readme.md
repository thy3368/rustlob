# spot的领域架构与布署架构


## 订单生命周期

- pre-parear  order_change_log,balance_change_log; user data; repo; Pending
- match order_change_logs;trade_change_logs; user data/market data; sub order_created_event ;     /// 已提交
  Submitted = 2,
  /// 部分成交
  PartiallyFilled = 3,
  /// 完全成交
  Filled = 4,
  /// 已取消
  Cancelled = 5,
  /// 已拒绝
  Rejected = 6,



- clear balance_change_logs; user data; sub trade_created_event


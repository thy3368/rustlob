// 业务指令

// parse & sort actions
// → freeze / pre-trade margin
// → match
// → settle fills (position/balance/margin)
// → risk check → liquidation / ADL actions（回灌队列）
// → time-driven settlement:
// if logical_time crosses funding boundary:
// settle funding
// → commit new state

// action-driven : place / cancel / modify
// state-driven  : liquidation / ADL
// logical-time-driven : funding / (可能还有 hourly settlement)

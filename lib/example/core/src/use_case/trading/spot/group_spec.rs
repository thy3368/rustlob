use cmd_handler::command_use_case_def2::{
    GroupBoundarySpec, MiCausalEdgeSpec, TruthCenterSpec, UseCaseGroupSpec, UseCaseInGroupSpec,
};

pub const SPOT_TRADING_GROUP_SPEC: UseCaseGroupSpec = UseCaseGroupSpec {
    group_name: "spot_trading",
    boundary: GroupBoundarySpec {
        scope: "现货订单从被交易账户发起，到订单成交、撤销或对应成交完成清结算为止",
        starts_when: "交易账户提交一个现货订单意图",
        ends_when: "订单进入 Filled 或 Canceled，且已产生的成交事实完成现货清结算",
        excludes: &["充值提现", "手续费计提", "行情推送", "adapter 请求映射", "事件持久化"],
    },
    truth_center: TruthCenterSpec {
        name: "SpotOrderFulfillment",
        main_mi: "SpotOrder",
        description: "一张现货订单在 group 边界内是否已经完成履约、撤销或仍处于可执行状态的业务真相",
    },
    mi_chain: &[
        MiCausalEdgeSpec {
            predecessor: "SpotOrderAccepted",
            predicate: "订单通过命令和状态校验，并冻结了可履约资产",
            successor: "SpotOrderWorking",
            caused_by: "PlaceOrderUseCase",
            due_to: "交易账户提交的现货订单意图",
        },
        MiCausalEdgeSpec {
            predecessor: "SpotOrderWorking",
            predicate: "taker 与 maker 订单价格、方向、资产和剩余数量满足撮合条件",
            successor: "SpotTradeMatched",
            caused_by: "MatchSpotOrderUseCase",
            due_to: "撮合引擎按价格时间优先选择的对手盘订单",
        },
        MiCausalEdgeSpec {
            predecessor: "SpotTradeMatched",
            predicate: "买卖双方冻结资产足以覆盖成交交割",
            successor: "SpotSettlementCreated",
            caused_by: "SettleSpotTradeUseCase",
            due_to: "已经生成且尚未清结算的现货成交事实",
        },
        MiCausalEdgeSpec {
            predecessor: "SpotOrderWorking",
            predicate: "订单仍可撤销且冻结余额足以释放剩余预留",
            successor: "SpotOrderCanceled",
            caused_by: "CancelSpotOrderUseCase",
            due_to: "交易账户提交的撤单意图",
        },
    ],
    terminal_facts: &["SpotOrderFilled", "SpotOrderCanceled", "SpotSettlementCreated"],
    use_cases: &[
        UseCaseInGroupSpec {
            name: "PlaceOrderUseCase",
            command: "PlaceOrderCmd",
            given_state: "PlaceOrderState",
            changes: "PlaceOrderExecution",
        },
        UseCaseInGroupSpec {
            name: "MatchSpotOrderUseCase",
            command: "MatchSpotOrderCmd",
            given_state: "MatchSpotOrderState",
            changes: "MatchSpotOrderChanges",
        },
        UseCaseInGroupSpec {
            name: "SettleSpotTradeUseCase",
            command: "SettleSpotTradeCmd",
            given_state: "SettleSpotTradeState",
            changes: "SettleSpotTradeChanges",
        },
        UseCaseInGroupSpec {
            name: "CancelSpotOrderUseCase",
            command: "CancelSpotOrderCmd",
            given_state: "CancelSpotOrderState",
            changes: "CancelSpotOrderChanges",
        },
    ],

    invariants: &[
        "订单成交数量不得超过原始数量",
        "已成交 trade 必须由方向相反且资产一致的订单产生",
        "清结算不得创建重复 settlement",
        "撤单只能释放订单剩余预留资产",
    ],
};

#[cfg(test)]
mod tests {
    use super::SPOT_TRADING_GROUP_SPEC;

    #[test]
    fn spot_group_spec_declares_required_modeling_fields() {
        let spec = SPOT_TRADING_GROUP_SPEC;

        assert!(!spec.group_name.is_empty());
        assert!(!spec.boundary.scope.is_empty());
        assert!(!spec.truth_center.name.is_empty());
        assert!(!spec.truth_center.main_mi.is_empty());
        assert!(!spec.mi_chain.is_empty());
        assert!(!spec.terminal_facts.is_empty());
        assert!(!spec.use_cases.is_empty());
        assert!(!spec.invariants.is_empty());
    }
}

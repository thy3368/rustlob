use cmd_handler::command_use_case_def2::{
    GroupBoundarySpec, MiCausalChainRootSpec, MiCausalChainSpec, MiCausalPointerSpec,
    MiInvariantSpec, MiPredicateSpec, MiProducedFactSpec, MiRootStateMachineSpec, MiRootStateSpec,
    MiRootStateTransitionSpec, TruthCenterSpec, UseCaseGroupSpec, UseCaseInGroupSpec,
};

pub const SPOT_TRADING_GROUP_SPEC: UseCaseGroupSpec = UseCaseGroupSpec {
    group_name: "spot_trading",
    boundary: GroupBoundarySpec {
        scope: "现货订单从被交易账户发起，到订单进入 Filled、Canceled 或 Rejected 终局状态为止",
        starts_when: "交易账户提交一个现货订单意图",
        ends_when: "订单进入 Filled、Canceled 或 Rejected",
        excludes: &[
            "充值提现",
            "成交清结算",
            "余额流水入账",
            "资金占用释放",
            "手续费计提",
            "行情推送",
            "adapter 请求映射",
            "事件持久化",
        ],
    },
    truth_center: TruthCenterSpec {
        name: "SpotOrderFulfillment",
        main_mi: "SpotOrder",
        description: "一张现货订单在 group 边界内是否已经完成履约、撤销或仍处于可执行状态的业务真相",
    },
    mi_chain: MiCausalChainSpec {
        root: MiCausalChainRootSpec {
            mi: "SpotOrder",
            identity: "order_id",
            created_by: "SpotOrderIntentSubmitted 通过受理校验后创建；未通过时以 SpotOrderRejected 收敛",
            starts_when: "交易账户提交现货订单意图，系统受理后生成 order_id",
            payload: &[
                "order_id",
                "account_id",
                "asset",
                "symbol",
                "side",
                "price",
                "qty",
                "filled_qty",
                "remaining_qty",
                "status",
                "time_in_force",
            ],
            why_root: "SpotOrder 是现货订单履约链的 main MI，承载 Open、PartiallyFilled、Filled、Canceled、Rejected 的合法演化",
            state_machine: MiRootStateMachineSpec {
                state_field: "SpotOrder.status",
                initial_state: "Open",
                states: &[
                    MiRootStateSpec {
                        name: "Open",
                        meaning: "订单已成立、仍有全部数量可撮合，或资金占用已证明其可履约",
                        terminal: false,
                    },
                    MiRootStateSpec {
                        name: "PartiallyFilled",
                        meaning: "订单已经产生至少一笔成交，但仍有剩余数量可继续撮合或撤销",
                        terminal: false,
                    },
                    MiRootStateSpec {
                        name: "Filled",
                        meaning: "订单累计成交数量等于原始数量，订单履约完成",
                        terminal: true,
                    },
                    MiRootStateSpec {
                        name: "Canceled",
                        meaning: "订单剩余未成交数量被合法撤销，后续不再进入撮合",
                        terminal: true,
                    },
                    MiRootStateSpec {
                        name: "Rejected",
                        meaning: "订单因参数、市场状态、资金能力或撮合限制被拒绝，未继续履约",
                        terminal: true,
                    },
                ],
                transitions: &[
                    MiRootStateTransitionSpec {
                        from: &["Open"],
                        to: &["Rejected"],
                        produced_fact: MiProducedFactSpec {
                            name: "SpotOrderRejected",
                            identity: "order_rejection_id",
                            kind: "append_only_fact",
                            produced_by: "PlaceOrderUseCase",
                            causal_pointer: MiCausalPointerSpec {
                                caused_by: "order_intent_id 或 order_id",
                                due_to: "订单意图或已创建订单未满足现货订单业务规则",
                            },
                        },
                        predicate: MiPredicateSpec {
                            reads: &[
                                "order_id",
                                "account_id",
                                "symbol",
                                "side",
                                "price",
                                "qty",
                                "reject_reason",
                            ],
                            rule: "订单参数、交易对状态、账户状态、余额能力或撮合限制任一拒绝规则成立",
                            when_true: "SpotOrder.status 进入 Rejected，并保留拒绝原因",
                            when_false: "订单继续处于可受理或可撮合路径",
                        },
                        root_change: "status = Rejected, status_reason = reject_reason",
                    },
                    MiRootStateTransitionSpec {
                        from: &["Open"],
                        to: &["Open"],
                        produced_fact: MiProducedFactSpec {
                            name: "SpotFundHoldCreated",
                            identity: "fund_hold_id",
                            kind: "fund_side_mi",
                            produced_by: "PlaceOrderUseCase",
                            causal_pointer: MiCausalPointerSpec {
                                caused_by: "order_id",
                                due_to: "现货订单成立后需要锁定买方报价资产或卖方基础资产",
                            },
                        },
                        predicate: MiPredicateSpec {
                            reads: &["fund_hold_id", "order_id", "asset", "held_qty"],
                            rule: "资金占用事实存在且覆盖订单剩余最大履约需求",
                            when_true: "SpotOrder 保持 Open，并具备进入撮合队列的履约担保",
                            when_false: "订单不得进入可撮合状态，等待拒绝或补偿事实",
                        },
                        root_change: "status 保持 Open，资金侧记录 fund_hold_id",
                    },
                    MiRootStateTransitionSpec {
                        from: &["Open", "PartiallyFilled"],
                        to: &["PartiallyFilled", "Filled"],
                        produced_fact: MiProducedFactSpec {
                            name: "SpotTradeExecuted",
                            identity: "trade_id",
                            kind: "secondary_mi",
                            produced_by: "MatchSpotOrderUseCase",
                            causal_pointer: MiCausalPointerSpec {
                                caused_by: "trade_id",
                                due_to: "该订单参与了一次价格交叉且数量可成交的现货成交",
                            },
                        },
                        predicate: MiPredicateSpec {
                            reads: &[
                                "trade_id",
                                "taker_order_id",
                                "maker_order_id",
                                "qty",
                                "previous_filled_qty",
                                "order_qty",
                            ],
                            rule: "成交数量应用到订单后，0 < filled_qty < order_qty 则部分成交，filled_qty == order_qty 则完全成交",
                            when_true: "更新 SpotOrder.filled_qty，并进入 PartiallyFilled 或 Filled",
                            when_false: "成交事实不应用到该订单，订单状态不变化",
                        },
                        root_change: "filled_qty += trade.qty, remaining_qty -= trade.qty, status = PartiallyFilled 或 Filled",
                    },
                    MiRootStateTransitionSpec {
                        from: &["Open", "PartiallyFilled"],
                        to: &["Canceled"],
                        produced_fact: MiProducedFactSpec {
                            name: "SpotOrderCanceled",
                            identity: "cancel_id",
                            kind: "append_only_fact",
                            produced_by: "CancelSpotOrderUseCase",
                            causal_pointer: MiCausalPointerSpec {
                                caused_by: "cancel_id",
                                due_to: "交易账户提交的合法撤单意图",
                            },
                        },
                        predicate: MiPredicateSpec {
                            reads: &["cancel_id", "order_id", "remaining_qty", "cancel_reason"],
                            rule: "订单仍有未成交数量、状态允许撤销，且撤单意图来自有权交易账户",
                            when_true: "SpotOrder.status 进入 Canceled，剩余数量停止后续撮合",
                            when_false: "拒绝撤单意图或等待订单进入可撤状态",
                        },
                        root_change: "status = Canceled, status_reason = cancel_reason",
                    },
                ],
            },
        },
        invariants: &[
            MiInvariantSpec {
                name: "OrderFillConservation",
                applies_to: &["SpotOrderAccepted", "SpotTradeExecuted", "SpotOrderFilled"],
                rule: "同一 order_id 的累计成交数量不得超过原始订单数量",
            },
            MiInvariantSpec {
                name: "TerminalStateExclusivity",
                applies_to: &["SpotOrderRejected", "SpotTradeExecuted", "SpotOrderCanceled"],
                rule: "同一 order_id 只能进入 Filled、Canceled、Rejected 三个终局状态之一",
            },
        ],
    },
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
            name: "CancelSpotOrderUseCase",
            command: "CancelSpotOrderCmd",
            given_state: "CancelSpotOrderState",
            changes: "CancelSpotOrderChanges",
        },
    ],
};

pub const SPOT_SETTLEMENT_GROUP_SPEC: UseCaseGroupSpec = UseCaseGroupSpec {
    group_name: "spot_settlement",
    boundary: GroupBoundarySpec {
        scope: "现货成交事实进入清结算后，到 settlement 事实和余额流水创建完成为止",
        starts_when: "一批已成交、未清结算的 SpotTrade 被清算方纳入待清结算批次",
        ends_when: "清结算批次进入 LedgerApplied 或 Rejected",
        excludes: &[
            "订单创建",
            "订单撮合",
            "撤单",
            "充值提现",
            "手续费计提",
            "行情推送",
            "adapter 请求映射",
            "事件持久化",
        ],
    },
    truth_center: TruthCenterSpec {
        name: "SpotSettlementClosure",
        main_mi: "SpotSettlementBatch",
        description: "一批现货成交在 group 边界内是否已经完成清结算、生成 settlement 并创建余额流水的业务真相",
    },
    mi_chain: MiCausalChainSpec {
        root: MiCausalChainRootSpec {
            mi: "SpotSettlementBatch",
            identity: "settlement_batch_id",
            created_by: "SettleSpotTradeCmd 将已成交且未结算的 SpotTrade 纳入待清结算批次",
            starts_when: "清算方提交一批 trade_id，系统准备为这些成交创建 settlement 和余额流水",
            payload: &[
                "settlement_batch_id",
                "trade_ids",
                "buyer_account_ids",
                "seller_account_ids",
                "base_asset",
                "quote_asset",
                "trade_qty",
                "trade_price",
                "status",
            ],
            why_root: "SpotSettlementBatch 是现货清结算链的 main MI，承载 Pending、SettlementsCreated、LedgerApplied、Rejected 的合法演化",
            state_machine: MiRootStateMachineSpec {
                state_field: "SpotSettlementBatch.status",
                initial_state: "Pending",
                states: &[
                    MiRootStateSpec {
                        name: "Pending",
                        meaning: "清结算批次已形成，等待从已成交事实推导 settlement 和余额流水",
                        terminal: false,
                    },
                    MiRootStateSpec {
                        name: "SettlementsCreated",
                        meaning: "批次内成交已经生成对应 SpotSettlement 事实，但余额流水尚未全部应用",
                        terminal: false,
                    },
                    MiRootStateSpec {
                        name: "LedgerApplied",
                        meaning: "批次内 settlement 对应的余额流水已经创建完成，清结算在本边界内闭合",
                        terminal: true,
                    },
                    MiRootStateSpec {
                        name: "Rejected",
                        meaning: "批次因成交缺失、重复清结算、资产不守恒或余额覆盖不足被拒绝",
                        terminal: true,
                    },
                ],
                transitions: &[
                    MiRootStateTransitionSpec {
                        from: &["Pending"],
                        to: &["Rejected"],
                        produced_fact: MiProducedFactSpec {
                            name: "SpotSettlementRejected",
                            identity: "settlement_rejection_id",
                            kind: "append_only_fact",
                            produced_by: "SettleSpotTradeUseCase",
                            causal_pointer: MiCausalPointerSpec {
                                caused_by: "settlement_batch_id",
                                due_to: "待清结算成交不满足幂等、资产守恒、流水完整性或冻结余额覆盖规则",
                            },
                        },
                        predicate: MiPredicateSpec {
                            reads: &[
                                "settlement_batch_id",
                                "trade_ids",
                                "existing_settlements",
                                "balances",
                                "reject_reason",
                            ],
                            rule: "任一 trade_id 不存在、已结算、资产借贷不守恒、缺少必要余额账户或冻结余额不足",
                            when_true: "SpotSettlementBatch.status 进入 Rejected，并保留拒绝原因",
                            when_false: "批次继续等待创建 settlement",
                        },
                        root_change: "status = Rejected, status_reason = reject_reason",
                    },
                    MiRootStateTransitionSpec {
                        from: &["Pending"],
                        to: &["SettlementsCreated"],
                        produced_fact: MiProducedFactSpec {
                            name: "SpotSettlementCreated",
                            identity: "settlement_id",
                            kind: "secondary_mi",
                            produced_by: "SettleSpotTradeUseCase",
                            causal_pointer: MiCausalPointerSpec {
                                caused_by: "trade_id",
                                due_to: "已成交事实需要生成可审计的现货清结算凭证",
                            },
                        },
                        predicate: MiPredicateSpec {
                            reads: &[
                                "trade_id",
                                "buyer_account_id",
                                "seller_account_id",
                                "base_asset",
                                "quote_asset",
                                "trade_qty",
                                "trade_price",
                            ],
                            rule: "成交未被结算且买卖双方、基础资产、报价资产、成交数量和成交价格可推出成对清结算分录",
                            when_true: "为每笔成交创建 SpotSettlement，批次进入 SettlementsCreated",
                            when_false: "不得创建 settlement，批次等待拒绝事实",
                        },
                        root_change: "status = SettlementsCreated, settlements += settlement_id",
                    },
                    MiRootStateTransitionSpec {
                        from: &["SettlementsCreated"],
                        to: &["LedgerApplied"],
                        produced_fact: MiProducedFactSpec {
                            name: "SpotBalanceLedgerApplied",
                            identity: "balance_ledger_entry_id",
                            kind: "fund_side_mi",
                            produced_by: "SettleSpotTradeUseCase",
                            causal_pointer: MiCausalPointerSpec {
                                caused_by: "settlement_id",
                                due_to: "现货 settlement 必须落成买卖双方基础资产和报价资产的余额流水",
                            },
                        },
                        predicate: MiPredicateSpec {
                            reads: &[
                                "settlement_id",
                                "buyer_base_delta",
                                "buyer_quote_delta",
                                "seller_base_delta",
                                "seller_quote_delta",
                                "available_balance",
                                "frozen_balance",
                            ],
                            rule: "每个 settlement 都产生完整买卖双方资产流水，且卖方冻结基础资产和买方冻结报价资产覆盖扣减",
                            when_true: "创建所有余额流水，批次进入 LedgerApplied",
                            when_false: "批次不能闭合，等待拒绝或补偿事实",
                        },
                        root_change: "status = LedgerApplied, ledger_entry_ids += balance_ledger_entry_id",
                    },
                ],
            },
        },
        invariants: &[
            MiInvariantSpec {
                name: "TradeSettlementIdempotency",
                applies_to: &["SpotTradeExecuted", "SpotSettlementCreated"],
                rule: "同一 trade_id 在清结算边界内最多只能生成一组 SpotSettlement",
            },
            MiInvariantSpec {
                name: "SpotAssetConservation",
                applies_to: &["SpotSettlementCreated", "SpotBalanceLedgerApplied"],
                rule: "不计手续费时，同一 trade_id 的基础资产和报价资产流水增减总和必须分别为零",
            },
            MiInvariantSpec {
                name: "SettlementLedgerCompleteness",
                applies_to: &["SpotSettlementCreated", "SpotBalanceLedgerApplied"],
                rule: "每个 settlement 必须生成买方基础资产、买方报价资产、卖方基础资产、卖方报价资产对应余额流水",
            },
            MiInvariantSpec {
                name: "FrozenBalanceCoverage",
                applies_to: &["SpotSettlementCreated", "SpotBalanceLedgerApplied"],
                rule: "卖方冻结基础资产必须覆盖交割数量，买方冻结报价资产必须覆盖成交价格乘以数量",
            },
        ],
    },
    use_cases: &[UseCaseInGroupSpec {
        name: "SettleSpotTradeUseCase",
        command: "SettleSpotTradeCmd",
        given_state: "SettleSpotTradeState",
        changes: "SettleSpotTradeChanges",
    }],
};

#[cfg(test)]
mod tests {
    use super::{SPOT_SETTLEMENT_GROUP_SPEC, SPOT_TRADING_GROUP_SPEC};

    #[test]
    fn spot_group_spec_declares_required_modeling_fields() {
        let spec = SPOT_TRADING_GROUP_SPEC;
        let chain = spec.mi_chain;

        assert!(!spec.group_name.is_empty());
        assert!(!spec.boundary.scope.is_empty());
        assert!(!spec.truth_center.name.is_empty());
        assert!(!spec.truth_center.main_mi.is_empty());
        assert!(!spec.use_cases.is_empty());
        assert!(spec.boundary.excludes.contains(&"成交清结算"));
        assert!(spec.boundary.excludes.contains(&"余额流水入账"));
        assert!(spec.boundary.excludes.contains(&"资金占用释放"));

        assert_eq!(chain.root.mi, "SpotOrder");
        assert_eq!(chain.root.identity, "order_id");
        assert!(!chain.root.created_by.is_empty());
        assert!(!chain.root.payload.is_empty());
        assert!(!chain.root.why_root.is_empty());

        let state_machine = chain.root.state_machine;
        assert_eq!(state_machine.state_field, "SpotOrder.status");
        assert_eq!(state_machine.initial_state, "Open");

        let state_names: Vec<&str> = state_machine.states.iter().map(|state| state.name).collect();
        for required_state in ["Open", "PartiallyFilled", "Filled", "Canceled", "Rejected"] {
            assert!(state_names.contains(&required_state));
        }

        let terminal_states: Vec<&str> = state_machine
            .states
            .iter()
            .filter(|state| state.terminal)
            .map(|state| state.name)
            .collect();
        assert_eq!(terminal_states, vec!["Filled", "Canceled", "Rejected"]);
        for state in state_machine.states {
            assert!(!state.name.is_empty(), "{state:?}");
            assert!(!state.meaning.is_empty(), "{state:?}");
        }

        let use_case_names: Vec<&str> =
            spec.use_cases.iter().map(|use_case| use_case.name).collect();
        assert!(!use_case_names.contains(&"SettleSpotTradeUseCase"));
        let mut transition_drivers = Vec::new();
        for transition in state_machine.transitions {
            assert!(!transition.from.is_empty(), "{transition:?}");
            assert!(!transition.to.is_empty(), "{transition:?}");
            assert!(!transition.produced_fact.name.is_empty(), "{transition:?}");
            assert!(!transition.produced_fact.identity.is_empty(), "{transition:?}");
            assert!(!transition.produced_fact.kind.is_empty(), "{transition:?}");
            assert!(!transition.predicate.reads.is_empty(), "{transition:?}");
            assert!(!transition.predicate.rule.is_empty(), "{transition:?}");
            assert!(!transition.predicate.when_true.is_empty(), "{transition:?}");
            assert!(!transition.predicate.when_false.is_empty(), "{transition:?}");
            assert!(
                use_case_names.contains(&transition.produced_fact.produced_by),
                "{transition:?}"
            );
            assert!(
                !use_case_names.contains(&transition.produced_fact.causal_pointer.caused_by),
                "{transition:?}"
            );
            assert!(!transition.produced_fact.causal_pointer.due_to.is_empty(), "{transition:?}");
            assert!(!transition.root_change.is_empty(), "{transition:?}");
            transition_drivers.push(transition.produced_fact.name);
        }
        assert!(transition_drivers.contains(&"SpotTradeExecuted"));
        assert!(transition_drivers.contains(&"SpotOrderCanceled"));
        assert!(transition_drivers.contains(&"SpotOrderRejected"));
        assert!(transition_drivers.contains(&"SpotFundHoldCreated"));

        let invariant_names: Vec<&str> =
            chain.invariants.iter().map(|invariant| invariant.name).collect();
        assert!(invariant_names.contains(&"OrderFillConservation"));
        assert!(invariant_names.contains(&"TerminalStateExclusivity"));
        for invariant in chain.invariants {
            assert!(!invariant.name.is_empty(), "{invariant:?}");
            assert!(!invariant.applies_to.is_empty(), "{invariant:?}");
            assert!(!invariant.rule.is_empty(), "{invariant:?}");
        }
    }

    #[test]
    fn spot_settlement_group_spec_declares_required_modeling_fields() {
        let spec = SPOT_SETTLEMENT_GROUP_SPEC;
        let chain = spec.mi_chain;

        assert_eq!(spec.group_name, "spot_settlement");
        assert!(!spec.boundary.scope.is_empty());
        assert!(!spec.boundary.starts_when.is_empty());
        assert!(!spec.boundary.ends_when.is_empty());
        assert!(spec.boundary.excludes.contains(&"订单创建"));
        assert!(spec.boundary.excludes.contains(&"订单撮合"));
        assert!(spec.boundary.excludes.contains(&"手续费计提"));
        assert_eq!(spec.truth_center.main_mi, "SpotSettlementBatch");
        assert!(!spec.truth_center.description.is_empty());

        assert_eq!(chain.root.mi, "SpotSettlementBatch");
        assert_eq!(chain.root.identity, "settlement_batch_id");
        assert!(!chain.root.created_by.is_empty());
        assert!(!chain.root.starts_when.is_empty());
        assert!(!chain.root.payload.is_empty());
        assert!(!chain.root.why_root.is_empty());

        let state_machine = chain.root.state_machine;
        assert_eq!(state_machine.state_field, "SpotSettlementBatch.status");
        assert_eq!(state_machine.initial_state, "Pending");

        let state_names: Vec<&str> = state_machine.states.iter().map(|state| state.name).collect();
        for required_state in ["Pending", "SettlementsCreated", "LedgerApplied", "Rejected"] {
            assert!(state_names.contains(&required_state));
        }

        let terminal_states: Vec<&str> = state_machine
            .states
            .iter()
            .filter(|state| state.terminal)
            .map(|state| state.name)
            .collect();
        assert_eq!(terminal_states, vec!["LedgerApplied", "Rejected"]);
        for state in state_machine.states {
            assert!(!state.name.is_empty(), "{state:?}");
            assert!(!state.meaning.is_empty(), "{state:?}");
        }

        let use_case_names: Vec<&str> =
            spec.use_cases.iter().map(|use_case| use_case.name).collect();
        assert_eq!(use_case_names, vec!["SettleSpotTradeUseCase"]);
        assert_eq!(spec.use_cases[0].command, "SettleSpotTradeCmd");
        assert_eq!(spec.use_cases[0].given_state, "SettleSpotTradeState");
        assert_eq!(spec.use_cases[0].changes, "SettleSpotTradeChanges");

        let mut transition_drivers = Vec::new();
        for transition in state_machine.transitions {
            assert!(!transition.from.is_empty(), "{transition:?}");
            assert!(!transition.to.is_empty(), "{transition:?}");
            assert!(!transition.produced_fact.name.is_empty(), "{transition:?}");
            assert!(!transition.produced_fact.identity.is_empty(), "{transition:?}");
            assert!(!transition.produced_fact.kind.is_empty(), "{transition:?}");
            assert_eq!(
                transition.produced_fact.produced_by, "SettleSpotTradeUseCase",
                "{transition:?}"
            );
            assert!(!transition.predicate.reads.is_empty(), "{transition:?}");
            assert!(!transition.predicate.rule.is_empty(), "{transition:?}");
            assert!(!transition.predicate.when_true.is_empty(), "{transition:?}");
            assert!(!transition.predicate.when_false.is_empty(), "{transition:?}");
            assert!(
                !transition.produced_fact.causal_pointer.caused_by.is_empty(),
                "{transition:?}"
            );
            assert!(!transition.produced_fact.causal_pointer.due_to.is_empty(), "{transition:?}");
            assert!(!transition.root_change.is_empty(), "{transition:?}");
            transition_drivers.push(transition.produced_fact.name);
        }
        assert_eq!(
            transition_drivers,
            vec!["SpotSettlementRejected", "SpotSettlementCreated", "SpotBalanceLedgerApplied",]
        );

        let invariant_names: Vec<&str> =
            chain.invariants.iter().map(|invariant| invariant.name).collect();
        assert!(invariant_names.contains(&"TradeSettlementIdempotency"));
        assert!(invariant_names.contains(&"SpotAssetConservation"));
        assert!(invariant_names.contains(&"SettlementLedgerCompleteness"));
        assert!(invariant_names.contains(&"FrozenBalanceCoverage"));
        for invariant in chain.invariants {
            assert!(!invariant.name.is_empty(), "{invariant:?}");
            assert!(!invariant.applies_to.is_empty(), "{invariant:?}");
            assert!(!invariant.rule.is_empty(), "{invariant:?}");
        }
    }
}

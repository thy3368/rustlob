use cmd_handler::command_use_case_def2::{
    GroupBoundarySpec, MiCausalChainSpec, MiCausalPointerSpec, MiInvariantSpec, MiPredicateSpec,
    MiSpec, MiStateMachineSpec, MiStateSpec, MiStateTransitionSpec, TruthCenterSpec,
    UseCaseGroupSpec, UseCaseInGroupSpec,
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
        root: MiSpec {
            name: "SpotOrder",
            identity: "order_id",
            created_by: Some(
                "SpotOrderIntentSubmitted 通过受理校验后创建；未通过时以 SpotOrderRejected 收敛",
            ),
            starts_when: Some("交易账户提交现货订单意图，系统受理后生成 order_id"),
            payload: Some(&[
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
            ]),
            why_root: Some(
                "SpotOrder 是现货订单履约链的 main MI，承载 Open、PartiallyFilled、Filled、Canceled、Rejected 的合法演化",
            ),
            state_machine: Some(MiStateMachineSpec {
                state_field: "SpotOrder.status",
                initial_state: "Open",
                states: &[
                    MiStateSpec {
                        name: "Open",
                        meaning: "订单已成立、仍有全部数量可撮合，或资金占用已证明其可履约",
                        terminal: false,
                    },
                    MiStateSpec {
                        name: "PartiallyFilled",
                        meaning: "订单已经产生至少一笔成交，但仍有剩余数量可继续撮合或撤销",
                        terminal: false,
                    },
                    MiStateSpec {
                        name: "Filled",
                        meaning: "订单累计成交数量等于原始数量，订单履约完成",
                        terminal: true,
                    },
                    MiStateSpec {
                        name: "Canceled",
                        meaning: "订单剩余未成交数量被合法撤销，后续不再进入撮合",
                        terminal: true,
                    },
                    MiStateSpec {
                        name: "Rejected",
                        meaning: "订单因参数、市场状态、资金能力或撮合限制被拒绝，未继续履约",
                        terminal: true,
                    },
                ],
                transitions: &[
                    MiStateTransitionSpec {
                        from: &["Open"],
                        to: &["Rejected"],
                        produced_fact: MiSpec {
                            name: "SpotOrderRejected",
                            identity: "order_rejection_id",
                            kind: Some("append_only_fact"),
                            produced_by: Some("PlaceOrderUseCase"),
                            causal_pointer: Some(MiCausalPointerSpec {
                                caused_by: "order_intent_id 或 order_id",
                                due_to: "订单意图或已创建订单未满足现货订单业务规则",
                            }),
                            created_by: None,
                            starts_when: None,
                            payload: None,
                            why_root: None,
                            state_machine: None,
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
                    MiStateTransitionSpec {
                        from: &["Open"],
                        to: &["Open"],
                        produced_fact: MiSpec {
                            name: "SpotFundHoldCreated",
                            identity: "fund_hold_id",
                            kind: Some("fund_side_mi"),
                            produced_by: Some("PlaceOrderUseCase"),
                            causal_pointer: Some(MiCausalPointerSpec {
                                caused_by: "order_id",
                                due_to: "现货订单成立后需要锁定买方报价资产或卖方基础资产",
                            }),
                            // Root MI fields - None for produced MI
                            created_by: None,
                            starts_when: None,
                            payload: None,
                            why_root: None,
                            state_machine: None,
                        },
                        predicate: MiPredicateSpec {
                            reads: &["fund_hold_id", "order_id", "asset", "held_qty"],
                            rule: "资金占用事实存在且覆盖订单剩余最大履约需求",
                            when_true: "SpotOrder 保持 Open，并具备进入撮合队列的履约担保",
                            when_false: "订单不得进入可撮合状态，等待拒绝或补偿事实",
                        },
                        root_change: "status 保持 Open，资金侧记录 fund_hold_id",
                    },
                    MiStateTransitionSpec {
                        from: &["Open", "PartiallyFilled"],
                        to: &["PartiallyFilled", "Filled"],
                        produced_fact: MiSpec {
                            name: "SpotTradeExecuted",
                            identity: "trade_id",
                            kind: Some("secondary_mi"),
                            produced_by: Some("MatchSpotOrderUseCase"),
                            causal_pointer: Some(MiCausalPointerSpec {
                                caused_by: "trade_id",
                                due_to: "该订单参与了一次价格交叉且数量可成交的现货成交",
                            }),
                            // Root MI fields - None for produced MI
                            created_by: None,
                            starts_when: None,
                            payload: None,
                            why_root: None,
                            state_machine: None,
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
                    MiStateTransitionSpec {
                        from: &["Open", "PartiallyFilled"],
                        to: &["Canceled"],
                        produced_fact: MiSpec {
                            name: "SpotOrderCanceled",
                            identity: "cancel_id",
                            kind: Some("append_only_fact"),
                            produced_by: Some("CancelSpotOrderUseCase"),
                            causal_pointer: Some(MiCausalPointerSpec {
                                caused_by: "cancel_id",
                                due_to: "交易账户提交的合法撤单意图",
                            }),
                            // Root MI fields - None for produced MI
                            created_by: None,
                            starts_when: None,
                            payload: None,
                            why_root: None,
                            state_machine: None,
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
            }),
            // Produced MI fields - None for root MI
            kind: None,
            produced_by: None,
            causal_pointer: None,
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

#[cfg(test)]
mod tests {
    use super::SPOT_TRADING_GROUP_SPEC;

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

        assert_eq!(chain.root.name, "SpotOrder");
        assert_eq!(chain.root.identity, "order_id");
        assert!(!chain.root.created_by.unwrap().is_empty());
        assert!(!chain.root.payload.unwrap().is_empty());
        assert!(!chain.root.why_root.unwrap().is_empty());

        let state_machine = chain.root.state_machine.unwrap();
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
            assert!(!transition.produced_fact.kind.unwrap().is_empty(), "{transition:?}");
            assert!(!transition.predicate.reads.is_empty(), "{transition:?}");
            assert!(!transition.predicate.rule.is_empty(), "{transition:?}");
            assert!(!transition.predicate.when_true.is_empty(), "{transition:?}");
            assert!(!transition.predicate.when_false.is_empty(), "{transition:?}");
            assert!(
                use_case_names.contains(&transition.produced_fact.produced_by.unwrap()),
                "{transition:?}"
            );
            assert!(
                !use_case_names
                    .contains(&transition.produced_fact.causal_pointer.unwrap().caused_by),
                "{transition:?}"
            );
            assert!(
                !transition.produced_fact.causal_pointer.unwrap().due_to.is_empty(),
                "{transition:?}"
            );
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
}

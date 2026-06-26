---
name: design-use-case-group
description: Define business use case groups with four-color modeling. Use when Codex should clarify a use case group before implementation, identify the `business_truth_center`, separate one group's main subject from adjacent actions, or output a business grouping spec for trading, clearing, settlement, or similar domains.
---

# Design Use Case Group

## Purpose

This skill is only for `use case group` discovery, boundary definition, and output organization.

It is not:
- the canonical definition of `Moment-Interval`
- the canonical reference for `MI` naming, audit, or `settled fact`
- the canonical reference for `use case / entity / aggregate` boundaries
- a code-writing skill

It produces a business grouping spec that later implementation, review, and modeling work can continue to use.

## Core Definition

A `use case group` is a set of business actions organized around the same `business_truth_center` and that center's legal evolution space inside a declared `group_boundary`.

Each `use_case` inside the group shares the same main business truth context, but has its own:
- `command`
- `given_state`
- `changes`

## Core Principle

Treat one `use case group` as one end-to-end `MI causal chain` / `MI 因果链`.

Local terminology:
- `business fact` / `业务事实`: a business fact that has already happened, can be recorded, and can be audited.
- `MI` / `Moment-Interval`: a business fact modeling form with time span, identity, traceability, and legal evolution semantics.
- `business truth` / `业务真相`: the business reality proven by one or more business facts / `MI` causal chains. Use the standard Chinese spelling `业务真相`.
- `MI causal chain` / `MI 因果链` is the preferred concept name.
- `end_to_end_mi_chain` is the output field for that concept.
- `business_truth_center` is the group's center business-truth context. It is usually the `main_mi`, not a separate object category.
- `business-truth chain` may be used as descriptive wording, but it must point back to `MI causal chain`.
- `final_settled_fact` is the template field for the terminal settled business fact inside `group_boundary`. Treat wording such as `closure fact` or `final settled fact` as referring to this field.

Recommended wording:
- Use `业务事实` / `business fact` for one concrete fact that has already become true.
- Use `MI` for a business fact object that satisfies the shared `Moment-Interval` standard.
- Use `业务真相` / `business truth` for the business reality or center context proven by facts and causal chains.
- Use `MI causal chain` / `MI 因果链` for the causal evidence chain between facts.

Business fact classification:
- `append-only fact`: one concrete happened fact. It is not updated in place; later corrections or progress are represented by appending new facts.
- `MI` / business fact object: an identity-bearing business object with lifecycle and legal evolution, proven and advanced by append-only facts.
- `current-state view`: mutable current fields, projections, cache rows, or query views derived from append-only facts. Treat these as state views, not fact bodies.

Examples:
- `Order` is usually an `MI` / business fact object, not one append-only fact and not `业务真相` itself.
- `OrderCreated`, `OrderAccepted`, `OrderPartiallyFilled`, `OrderFilled`, `OrderCancelled`, and `OrderExpired` are append-only facts.
- `Order.status`, `Order.remaining_qty`, `Account.available_balance`, and `Account.frozen_balance` are current-state views unless the domain explicitly models them as auditable facts.
- A business truth example is `订单 O1 在该业务边界内已经完成履约`, proven by the `Order` / `Trade` / fund-side fact chain.

The key question is not:
- how many related objects appear in the flow
- how many handlers or tables are touched

The key question is:
- whether the `MI causal chain` reaches the final settled fact inside the declared `group_boundary`

`end-to-end` here is business truth, not HTTP -> service -> DB -> MQ.
It is not a process flow, state transition list, or ordered object list.

All `MI` definition, `main_mi` / `secondary_mis`, `MI causal chain` judgment, append-only facts, and `settled fact` calibration must follow the shared references below. This file does not override them.

## Required References

Read the references in this order.

1. [`../shared/mi_chain.md`](../shared/mi_chain.md)
   Primary reference for `MI causal chain` semantics: causal links, `caused_by` / `due_to`, predicates, invariants, and terminal closure.
2. [`../shared/moment_interval_definition.md`](../shared/moment_interval_definition.md)
   Base definition for `Moment-Interval` and minimum judgment standard.
3. [`../shared/mi.md`](../shared/mi.md)
   Extended judgment for `main_mi` / `secondary_mis`, append-only facts, naming calibration, and `settled fact`.
4. [`references/review_checklist.md`](references/review_checklist.md)
   Completeness and end-to-end closure review for a proposed group.
5. [`../shared/use_case_entity_aggregate_boundary.md`](../shared/use_case_entity_aggregate_boundary.md)
   Boundary split for `use case`, `entity behavior method`, `aggregate`, and cross-aggregate coordination.

If a task extends to `MI -> entity` naming calibration, audit-voucher facts, or final-settlement closure, do not answer from examples in this file first. Route to the shared references, then use this skill only to organize the group output.

## Routing Rule

When there is any `MI` boundary dispute, first cite the shared reference, then apply this skill's grouping method.

Routing order:
1. `mi_chain.md` for causal links, `caused_by` / `due_to`, predicates, invariants, and terminal closure
2. `moment_interval_definition.md` for base definition and minimum threshold
3. `mi.md` for advanced `MI` judgment, naming, append-only facts, and `settled fact`
4. `review_checklist.md` for completeness and closure review
5. `use_case_entity_aggregate_boundary.md` for `use case / entity / aggregate` placement

Examples in this file are only calibration examples. They never override the shared references.

## Discovery Tool, Not Final Target

Four-color modeling is only a discovery aid.

Use the minimum vocabulary when it helps:
- `Moment-Interval`
- `Role`
- `Party/Place/Thing`
- `Description`

Do not use this skill to locally redefine those concepts.
The final output is still a `use case group` spec, not a color diagram.

## Method

Use this fixed sequence.

1. Infer the task boundary and candidate main subject from the user input and repository context first.
   Ask only when the business loop being claimed or its stopping point is missing or materially ambiguous.
2. Use [`../shared/moment_interval_definition.md`](../shared/moment_interval_definition.md) to judge whether the candidate can be the `business_truth_center`.
   If it is only a command, field, balance value, check step, executor step, or technical artifact, reject it.
3. Use [`../shared/mi_chain.md`](../shared/mi_chain.md) to trace the `MI causal chain`.
   The chain may branch through secondary MIs and fund-side facts, but each branch must still identify predecessor facts, predicates, `caused_by` / `due_to` links, invariants, and terminal closure.
4. Use [`../shared/mi.md`](../shared/mi.md) to identify:
   - `main_mi`
   - `secondary_mis`
   - append-only facts
   - `which_items_are_not_mi_and_why`
   - `final_settled_fact`
5. Use [`references/review_checklist.md`](references/review_checklist.md) to verify the chain truly closes inside the declared `group_boundary`.
   If the chain stops at an intermediate fact, either narrow the boundary or continue the chain.
6. Use [`../shared/use_case_entity_aggregate_boundary.md`](../shared/use_case_entity_aggregate_boundary.md) to split which actions are independent `use_case` values.
   Keep cross-aggregate coordination in `use case`; do not bury it in one `entity` or `aggregate` method.
7. Organize the final output.
   The output must clearly separate group center, closure fact, use case boundaries, and non-use-case items.

## Group Boundary Rules

The `business_truth_center` should satisfy the shared-reference expectations for a main `MI` by default.
Using a non-`MI` equivalent business truth center is an explicit justified exception, not a general fallback, and must be justified against the shared references.

Choose one main subject by default.

Split groups first when peer subjects both have:
- independent identity
- independent lifecycle
- independent audit meaning
- independent legal evolution space

Do not keep one group together only because:
- one API currently touches both
- one workflow currently orchestrates both
- one table or stream currently stores both

## Independent Use Case Rule

Promote an action into an independent `use_case` only when it has independent business meaning.

Strong signals:
- independent authorization
- independent failure semantics
- independent audit meaning
- independent state change

Usually not independent `use_case` values:
- validator steps
- persistence steps
- publish steps
- adapter mapping steps
- executor or scheduler sub-steps
- pure entity helper/query logic

Reusable business rules inside one main action may live in `entity` / `aggregate` behavior.
Cross-aggregate coordination must remain a `use case` concern.

## Trading Calibration Example

Use `Order` as a calibration object for group design, not as a replacement for the shared `MI` definitions.

Typical reading:
- `business_truth_center`: `Order`
- `main_mi`: `Order`
- append-only facts: `OrderCreated`, `OrderAccepted`, `OrderPartiallyFilled`, `OrderFilled`, `OrderCancelled`, `OrderExpired`
- current-state views, not fact bodies: `Order.status`, `Order.remaining_qty`, `Order.filled_qty`
- possible `secondary_mis`: `Trade`, `FundHold`
- possible lifecycle: `Created -> Working -> PartiallyFilled -> Filled | Cancelled | Rejected | Expired`

Boundary judgment examples:
- If the declared boundary is only order matching, `Order --[matching predicate, caused_by/due_to]--> Trade` may already close the group.
- If the declared boundary claims full fulfillment, `Order --[matching predicate, caused_by/due_to]--> Trade` is still incomplete until the boundary's real `final_settled_fact` is explicit.

Cross-aggregate coordination example:
- `cancel_order` may coordinate `Order` and `FundHold`
- this does not automatically change the main `business_truth_center`
- but it does mean the `use_case` may coordinate multiple entities or aggregates

`Trade` may be a secondary `MI` inside the same chain, or it may deserve another group, depending on the shared-reference criteria.

## Anti-Examples

These usually are not independent `use_case` values:
- `ValidateOrderInput`
- `CheckRisk`
- `ReserveMargin`
- `PersistOrder`
- `PublishOrderEvent`
- `LoadState`
- `MapReply`

These usually are not valid `business_truth_center` candidates:
- `available_balance`
- `frozen_amount` as only a number field
- `matching_step`
- `db_row`

Whether any of them is an `MI` in a special domain still depends on the shared references, not on this file's examples.

## Output Template

### Use Case Group
- `group_name`:
- `group_boundary`:
- `business_truth_center`:
- `main_mi`:
- `secondary_mis`: confirmed chain-internal secondary MIs
- `append_only_facts`:
- `four_color_archetype`:
- `end_to_end_mi_chain`: causal edges, preferably `predecessor_mi --[predicate, caused_by/due_to]--> successor_mi`
- `final_settled_fact`: terminal settled business fact inside the declared boundary
- `which_facts_require_independent_mi`: facts that may need independent MI treatment or a group split, with reason
- `which_items_are_not_mi_and_why`:
- `lifecycle_or_state_machine`:
- `recommended_business_names`:
- `use_cases`:
- `non_use_case_items`:

### Use Case
#### `<use_case_name>`
- `command`:
- `given_state`:
- `changes`:
- `primary_entity_or_aggregate`:
- `coordinated_entities_or_aggregates`:

## Output Rules

Do not group by API shape, handler shape, database table shape, or technical step order.

The output must explicitly separate:
- which business-truth context / `main_mi` is the group's `business_truth_center`
- where the group's final settled fact is
- which actions are independent `use_case` values
- which items are only entity rules, helpers, adapters, or executor steps
- which `use_case` values coordinate multiple entities or aggregates

`group_boundary` must answer:
- which main subject this group is centered on
- which `MI causal chain` / business-truth chain this group is responsible for
- which legal evolutions are covered
- where the boundary-internal `final_settled_fact` sits
- which adjacent actions do not belong here, and why

When `MI` judgment is involved, the output must show:
- `main_mi`
- `secondary_mis`
- `append_only_facts`
- `end_to_end_mi_chain`
- `final_settled_fact`
- `which_facts_require_independent_mi`
- `which_items_are_not_mi_and_why`

Each `secondary_mi` included in the chain must name its predecessor fact and derivation reason. Prefer writing `end_to_end_mi_chain` as causal edges in the form `predecessor_mi --[predicate, caused_by/due_to]--> successor_mi`.

When recommending a business-object name for an `MI` that should land as an `entity`, aggregate-core object, or audit fact, also provide:
- `recommended_business_names`
- naming reason
- why the name is not a command name, step name, or technical action name

Do not assume one `use_case` maps to exactly one `entity`.
Keep the output short, explicit, and implementation-oriented.

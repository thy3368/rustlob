---
name: design-use-case-group
description: Define business use case groups with four-color modeling. Use when Codex should clarify a use case group before implementation, identify the `business_truth_center`, separate one group's main subject from adjacent actions, or output each use case as `command / given_state / changes / entity` for trading, clearing, settlement, or similar domains.
---

# Design Use Case Group

## Purpose

This skill is for defining a `use case` group before implementation.

It is not a code-writing skill. Use it to decide what belongs in one group, what the group's main subject is, and which business actions deserve independent `use_case` boundaries.

## Core Definition

A `use case` group is the set of business actions organized around the same `business_truth_center` and that center's legal evolution space.

Each `use_case` inside the group shares the same main subject, but has its own:
- `command`
- `given_state`
- `changes`

## Core Principle: End-to-End MI Chain

Treat a `use case` group as the design of one end-to-end `Moment-Interval` business-truth chain.

Unified definition:
- `端到端 MI 链 = 从主 MI 成立开始，经过合法推进与派生，一直到该 group 业务边界内最终落定事实的业务真相链`

The main design rule is:
- do not ask whether the chain contains enough related objects
- ask whether it reaches the final settled fact inside the declared `group_boundary`

`end-to-end` here is about business truth, not technical calls:
- start: the main `Moment-Interval` becomes valid
- middle: legal progression, branching, and secondary `Moment-Interval` derivation
- end: the final settled fact inside this group's business boundary

It is not HTTP -> service -> DB -> MQ.

## Business Truth Center Rule

Only choose an object as `business_truth_center` when it is worthy of being remembered as an independent business truth.

It should satisfy most of these tests:
- It has an identity that matters to the business.
- It has a lifecycle or state machine the business cares about.
- Its changes are auditable as business facts.
- Multiple business actions exist only because this object can evolve in distinct legal ways.

If the object is only a validation step, technical artifact, persistence concern, or transient calculation, it is usually not a valid group center.

## One Group, One Main Subject

By default, one group should have one main subject.

If two peer subjects both have independent identity, independent lifecycle, and independent audit meaning, split them into separate groups first.

Do not keep one group together only because the API, handler, database table, or workflow currently touches both.

## Four-Color Modeling Basics

Use the minimum four-color vocabulary:
- `Moment-Interval`: a business happening, commitment, transaction, or lifecycle step worth remembering
- `Role`: the part a party plays in the business game
- `Party/Place/Thing`: a business actor or object
- `Description`: policy, classification, rule, capability, or descriptive data

Read the relationships as:
- `Party/Place/Thing -> plays Role -> participates in Moment-Interval`
- `Party/Place/Thing -> uses Description`

## Required Reference

If the task asks about `时标对象`, `Moment-Interval`, or how to distinguish `business_truth_center` from commands, fields, balances, or technical steps, read [`references/moment_interval_definition.md`](references/moment_interval_definition.md) before answering.

If the task asks about any of these topics, read [`../shared/mi.md`](../shared/mi.md) before answering:
- `MI 识别`
- `主 MI / 次级 MI`
- `资金侧 MI`
- `审计凭证型 MI`
- `端到端 MI 链`
- `settled fact`
- `业务事实留痕`
- 哪些事实必须 append-only
- 哪些对象不是 `MI` 以及原因

If the task asks how to review whether a group is complete, whether an MI chain is truly end-to-end, or whether the modeled boundary reaches the final settled fact, read [`references/review_checklist.md`](references/review_checklist.md) before answering.

If the task asks about any of these topics, read [`../shared/use_case_entity_aggregate_boundary.md`](../shared/use_case_entity_aggregate_boundary.md) before answering:
- `use case` vs `entity`
- `entity behavior method`
- `aggregate root` 该管什么
- `helper/query method`
- `state machine` 与实体方法关系
- “什么该升格成 use case”

If the task extends to whether a core object inside the group should become an `entity`, or what its archetype is, read [`../shared/entity_four_color_classification.md`](../shared/entity_four_color_classification.md) before answering.

## Four-Color Is A Discovery Tool

Four-color modeling is a discovery tool, not the final output target.

Use it in this order:
1. Find the `business_truth_center`.
2. Judge its main four-color archetype.
3. Trace the end-to-end `Moment-Interval` chain inside the declared boundary.
4. Find its legal evolution space.
5. Find the boundary between independent business actions.

The final output is still a use-case-group definition, not a color diagram.

## Methodology

1. Find the `business_truth_center`.
2. Judge its main archetype.
3. Write the end-to-end `Moment-Interval` chain inside the declared group boundary.
4. Define its `lifecycle_or_state_machine`.
5. Identify independent `use_case` boundaries.
6. For each `use_case`, write:
   - `command`
   - `given_state`
   - `changes`
   - `entity`

## Independent Use Case Rule

Split an action into an independent `use_case` only when it has independent business meaning.

A good independent `use_case` usually has:
- independent authorization
- independent failure semantics
- independent audit meaning
- independent business state change

If an action is only a sub-step inside another business action, keep it inside that use case or inside an entity rule instead of promoting it into a separate `use_case`.

主业务动作内部的可复用规则可以落在 `entity` / `aggregate` rule。
但跨聚合协调不得下沉成单个 `entity` / `aggregate method`。

## Moment-Interval Routing Rule

Do not answer `Moment-Interval` boundary questions from memory or by extrapolating from examples in this file. Use the required reference above as the canonical definition.

Routing order:
1. If the task is about `MI 识别`、`主次 MI`、`审计链`、`settled fact`、`业务事实留痕`, read `../shared/mi.md` first.
2. If the task is about `use case group` 划分、`business_truth_center`、`Moment-Interval` 定义, also read `references/moment_interval_definition.md`.
3. If the task extends to `entity / aggregate / state machine` 边界, continue to `../shared/use_case_entity_aggregate_boundary.md`.

Do not rely on the examples in this file alone to judge whether something should become an independent `MI`, whether it is only a state/command/field, or where the final settled fact sits.

## Trading Example

Use `Order` as the calibration example:
- `business_truth_center`: `Order`
- main archetype: `Moment-Interval`
- state machine: `Created -> Working -> PartiallyFilled -> Filled | Cancelled | Rejected | Expired`

Inside the `Order` group, different `use_case` values may include submit, cancel, replace, expire, or reject because they are distinct legal evolutions of the same main subject.

`Trade` or `Fill` is often a related or secondary `Moment-Interval`, not automatically the same group center as `Order`.

`ConditionalOrder` is not just a variant label if it has its own activation logic and lifecycle meaning. When it carries a separate business truth and legal evolution space, treat it as a different `Moment-Interval` and usually a different group.

Use `Order` to calibrate boundary judgment:
- if the group boundary is only the matching loop, `Order -> Trade` may already be complete
- if the group boundary claims to cover the full trading fulfillment loop, `Order + Trade` is still incomplete without the boundary's final settled fact such as `Settlement`

Do not hard-code `Settlement` as a universal final noun for every domain. The real standard is always:
- what is the final settled fact inside this group's declared business boundary?

## Anti-Examples

These are usually not independent `use_case` values:
- `ValidateOrderInput`
- `CheckRisk`
- `ReserveMargin`
- `PersistOrder`
- `PublishOrderEvent`
- `MapReply`
- `LoadState`

They are commonly:
- entity rules
- policy checks
- adapter work
- executor steps
- infra steps

Do not elevate them into business `use_case` values unless the domain truly treats them as independent business actions with separate audit meaning.

## Output Template

### Use Case Group
- `business_truth_center`:
- `main_mi`:
- `secondary_mis`:
- `four_color_archetype`:
- `end_to_end_mi_chain`:
- `final_settled_fact`:
- `which_facts_require_independent_mi`:
- `which_items_are_not_mi_and_why`:
- `lifecycle_or_state_machine`:
- `group_boundary`:
- `use_cases`:

### Use Case
#### `<use_case_name>`
- `command`:
- `given_state`:
- `changes`:
- `entity`:

## Output Rules

Do not group by API shape, handler shape, database tables, or process steps.

`group_boundary` must answer:
- which main subject this group is centered on
- which end-to-end `Moment-Interval` chain this group is responsible for
- which legal evolutions of that subject are covered
- where the final settled fact inside that boundary is
- which adjacent actions do not belong to this group, and why

When the task involves `MI` judgment, the output should explicitly separate:
- `main_mi`
- `secondary_mis`
- `end_to_end_mi_chain`
- `final_settled_fact`
- `which_facts_require_independent_mi`
- `which_items_are_not_mi_and_why`

Keep the output short, rule-driven, and implementation-oriented.

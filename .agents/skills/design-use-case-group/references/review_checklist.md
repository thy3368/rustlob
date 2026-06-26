# Use Case Group Review Checklist

## Purpose

Use this checklist to review whether a `use case` group definition is complete, boundary-consistent, and truly end-to-end in business-truth terms.

Do not review completeness by counting objects or steps.

Use this rule instead:
- `不要问链上对象够不够多，要问它是否追到了边界内最终结果`

## Core Review Definition

Unified definition:
- `端到端 MI 因果链 = 从主 MI 成立开始，经过合法推进与派生，一直到该 group 业务边界内最终落定事实的业务真相链`
- `end_to_end_mi_chain` is the output field for that `MI causal chain`.

Unified anti-example expression:
- `只到中间事实而未到最终落定事实的链，不算完整端到端 MI 因果链`

`end-to-end` here refers only to business truth boundaries.
It means causal evidence closure, not a process flow, state transition list, or ordered object list.

It does not mean:
- HTTP chain
- service orchestration chain
- DB write chain
- MQ publish chain

## Checklist

1. Is the chosen `business_truth_center` truly the main `Moment-Interval` or an explicitly justified equivalent business truth center, instead of a command, field, balance value, or technical step?
2. Has the main `MI causal chain` been traced from its creation through legal progression and derivation?
3. Has that chain been traced all the way to the final settled fact inside the declared `group_boundary`?
4. Are secondary `Moment-Interval` values modeled as legal progression or derivation from the main chain, with predecessor facts and `caused_by` / `due_to` reasons, instead of being mixed in without boundary logic?
5. Do the proposed `use_case` boundaries correspond to independent business meaning, rather than validator, persistence, publish, or executor steps?
6. Is the declared `group_boundary` consistent with where the modeled chain actually stops?
7. Does the model avoid treating `业务真相` / `business truth` as a directly persisted object? A persistence candidate should usually first be judged as a concrete `业务事实` / `business fact` or `MI`.
8. Does the model separate append-only facts, identity-bearing `MI` / business fact objects, and mutable current-state views instead of calling all of them the same kind of fact?

## High-Priority Failure Rule

If a group claims to cover a complete business loop, but its `MI causal chain` stops at an intermediate fact and never reaches the final settled fact inside that boundary, the group is incomplete.

In that case, do one of these:
- narrow the `group_boundary` honestly
- continue the chain until the final settled fact is explicit

## Boundary Calibration Examples

### `Order` Matching Group

If the boundary is only the matching loop, then:
- `Order --[matching predicate, caused_by/due_to]--> Trade` may already be a complete end-to-end `MI causal chain`

### `Order` Fulfillment Group

If the boundary claims to cover the full trading fulfillment loop, then:
- `Order + Trade` is not enough
- without a final settled fact such as `Settlement`, the chain is incomplete

`Settlement` is only a domain example, not a universal mandatory final noun.

The actual standard is:
- the final settled fact inside the declared business boundary

### `Reservation` Group

If the boundary covers fulfillment from booking through checkout, then:
- modeling only `Reservation` is not enough
- if there is no actual check-in, room change, checkout, or equivalent final settled stay fact, the chain is likely cut off mid-way

### `Wallet` / `FundHold` Group

If the boundary claims to cover fund fulfillment, then:
- showing only resulting balances is not enough
- without hold, release, consume, settle, or equivalent auditable fund facts, the chain is incomplete

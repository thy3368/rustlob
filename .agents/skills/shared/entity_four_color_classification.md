## Purpose

This file is the canonical rule for deciding what a candidate domain object is in four-color modeling,
how that classification affects whether it should become an `entity`, and how that choice should shape
modeling and layer boundaries.

It answers only these questions:

- what the object is in four-color terms
- whether it deserves to be an `entity`
- what kind of `entity` it is if promoted
- how that classification should influence methods and boundaries

## Four-Color Mapping For Entity

- `Moment-Interval`
  - A business truth center worth remembering, auditing, and evolving through a lifecycle or state machine.
  - This is the most common archetype for a main `entity` or `aggregate root`.
- `Party/Place/Thing`
  - A business participant or object that matters to the domain, but does not itself drive the main lifecycle progression.
  - This is commonly a `supporting entity`, owned object, or referenced object.
- `Role`
  - Not the default home for an independent `entity`.
  - Promote a role into an `entity` only when the role itself has independent identity, lifecycle, constraints, and audit meaning.
- `Description`
  - Classification, rule, capability, configuration, or product definition.
  - This usually belongs as `description`, `value object`, or `policy`, not as a behavior-carrying `entity`.

## Classification Procedure

1. Decide whether the object is worth being remembered as an independent business truth.
2. Decide whether it has independent identity.
3. Decide whether it has an independent lifecycle or state machine.
4. Decide whether its changes have independent audit meaning.
5. Classify it as `Moment-Interval`, `Party/Place/Thing`, `Role`, or `Description`, then decide whether it should really become an `entity`.

## Entity Output Contract

Any skill output that classifies a candidate `entity` should include at least:

- `business_object`
- `four_color_archetype`
- `why_it_is_an_entity`
- `entity_kind`: `truth_center` / `supporting_entity` / `role_entity` / `description_backed_entity`
- `allowed_methods_bias`: `behavior` or `helper/query`

## Boundary Implications

- `Moment-Interval` entities should more naturally carry lifecycle and state-transition `behavior methods`.
- `Party/Place/Thing` entities more often carry eligibility checks, ownership checks, and derived business calculations.
- `Role` should stay as relationship semantics unless it truly qualifies for promotion into an `entity`.
- `Description` should usually not carry business state transitions.

## Anti-Patterns

- Turning pure description or policy data into a rich `entity`
- Mistaking a temporary technical object for `Party/Place/Thing`
- Creating an `entity` directly from a role name without identity or lifecycle
- Hiding cross-aggregate process actions inside a fake `Moment-Interval entity` method

## Trading Calibration Examples

- `Order` -> `Moment-Interval`
- `TradeFill` -> secondary `Moment-Interval` or derived business truth
- `Account` / `Wallet` / `Instrument` -> common `Party/Place/Thing`
- `Maker` / `Taker` -> default `Role`
- `OrderType` / `RiskRule` / `FeeSchedule` -> `Description`

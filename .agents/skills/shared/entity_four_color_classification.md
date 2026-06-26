## Purpose

`lib/common/entity/src/entity.rs` is the source of truth for entity classification metadata.
This file explains how to use that Rust metadata when deciding what a candidate domain object is in
four-color modeling, whether it should become an `entity`, and how that choice should shape modeling
and layer boundaries.

Do not maintain a second enum, default-value, or string-label table here. Enum variants, default
implementations, and `as_str()` labels must follow the Rust source, especially:

- `FourColorArchetype`
- `EntityMethodBias`
- `EntityMutationModel`
- `EntityLifecycleModel`
- the `Entity` trait metadata methods

It answers only these questions:

- what the object is in four-color terms
- whether it deserves to be an `entity`
- which existing `Entity` metadata should describe it if promoted
- how that classification should influence methods and boundaries

## Four-Color Mapping For Entity

- `FourColorArchetype::MomentInterval`
  - A business truth center worth remembering, auditing, and evolving through a lifecycle or state machine.
  - This is the most common archetype for a main `entity` or `aggregate root`.
- `FourColorArchetype::PartyPlaceThing`
  - A business participant or object that matters to the domain, but does not itself drive the main lifecycle progression.
  - This is commonly a `supporting entity`, owned object, or referenced object.
- `FourColorArchetype::Role`
  - Not the default home for an independent `entity`.
  - Promote a role into an `entity` only when the role itself has independent identity, lifecycle, constraints, and audit meaning.
- `FourColorArchetype::Description`
  - Classification, rule, capability, configuration, or product definition.
  - This usually belongs as `description`, `value object`, or `policy`, not as a behavior-carrying `entity`.

Use `FourColorArchetype::Unclassified` only as the source-level default for legacy or unguided entities,
not as a modeling conclusion.

## Classification Procedure

1. Decide whether the object is worth being remembered as an independent business truth.
2. Decide whether it has independent identity.
3. Decide whether it has an independent lifecycle or state machine.
4. Decide whether its changes have independent audit meaning.
5. Classify it with `FourColorArchetype`, then decide whether it should really become an `entity`.
6. If promoted, choose the source-defined `Entity` metadata methods that explain its modeling role:
   `four_color_archetype()`, `allowed_methods_bias()`, `mutation_model()`, and `lifecycle_model()`.

## Entity Output Contract

Any skill output that classifies a candidate `entity` should include at least:

- `business_object`
- `why_it_is_an_entity`
- `four_color_archetype`: the intended `FourColorArchetype` variant or its `as_str()` label
- `allowed_methods_bias`: the intended `EntityMethodBias` variant or its `as_str()` label
- `mutation_model`: the intended `EntityMutationModel` variant or its `as_str()` label
- `lifecycle_model`: the intended `EntityLifecycleModel` variant or its `as_str()` label

Structured metadata fields should map directly to existing `Entity` trait methods. If a string label is
needed, use the enum's `as_str()` semantics from `entity.rs`; do not invent labels in this document or
in downstream skill output.

## Boundary Implications

- `FourColorArchetype::MomentInterval` entities should more naturally carry lifecycle and state-transition
  behavior methods.
- `FourColorArchetype::PartyPlaceThing` entities more often carry eligibility checks, ownership checks,
  and derived business calculations.
- `FourColorArchetype::Role` should stay as relationship semantics unless it truly qualifies for promotion
  into an `entity`.
- `FourColorArchetype::Description` should usually not carry business state transitions.
- `EntityMethodBias` is a governance signal, not an allowlist. It should guide review of method placement
  without replacing the four-color archetype.
- `EntityMutationModel` and `EntityLifecycleModel` explain whether the entity is current mutable state,
  append-only fact, snapshot, derived read model, stateless fact, stateful lifecycle, or dependent lifecycle.
  Use the Rust source for the exact variants and defaults.

## Anti-Patterns

- Turning pure description or policy data into a rich `entity`
- Mistaking a temporary technical object for `Party/Place/Thing`
- Creating an `entity` directly from a role name without identity or lifecycle
- Hiding cross-aggregate process actions inside a fake `Moment-Interval entity` method
- Adding new canonical classification fields in skill output before adding the corresponding metadata to
  `lib/common/entity/src/entity.rs`
- Copying enum labels into this document instead of reading the source-level `as_str()` semantics

## Trading Calibration Examples

- `Order` -> `FourColorArchetype::MomentInterval`, usually stateful and behavior-oriented.
- `TradeFill` -> secondary `FourColorArchetype::MomentInterval` or derived business truth, commonly an
  append-only fact.
- `Account` / `Wallet` / `Instrument` -> common `FourColorArchetype::PartyPlaceThing`.
- `Maker` / `Taker` -> default `FourColorArchetype::Role`; promote only with independent identity and lifecycle.
- `OrderType` / `RiskRule` / `FeeSchedule` -> `FourColorArchetype::Description`.

# Multi-sig

HyperCore supports native multi-sig actions, enabling multiple private keys to control a single account. Unlike other chains, this is a built-in primitive rather than a smart contract feature.

## Workflow

- To convert a user, send a `ConvertToMultiSigUser` action with authorized users and a minimum signature threshold. Authorized users must already exist on Hyperliquid. After conversion, all actions must go through multi-sig.
- Each authorized user signs a payload to produce a signature. A `MultiSig` action wraps any normal action and includes the collected signatures.
- The `MultiSig` payload includes the target multi-sig user and the authorized user sending the transaction — known as the `leader`.
  - Only the leader's nonce is validated and updated.
  - The leader can also be an API wallet of an authorized user; that wallet's nonce is then checked.
- To update authorized users or threshold, send a `MultiSig` wrapping a new `ConvertToMultiSigUser` action.
- To revert to a normal user, send a `ConvertToMultiSigUser` via multi-sig with an empty authorized users set.

## Notes

- The leader must be an authorized user, not the multi-sig account itself.
- "Each signature must use the same information, e.g., same nonce, transaction lead address."
- The leader must collect all signatures before submitting.
- A user can simultaneously be a multi-sig user and an authorized user for other multi-sig accounts. Max authorized users per account: 10.

## HyperEVM Warning

Converting to multi-sig still leaves the HyperEVM user controllable by the original wallet. CoreWriter does not work for multi-sig users. Multi-sig users should avoid HyperEVM interactions before or after conversion.

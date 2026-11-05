//! Hyperliquid node deployment assets and replica RPC access.
//!
//! The RPC URL is always supplied by the caller. For example, block zero can
//! be fetched from a self-hosted node's `replica_cmds` endpoint:
//!
//! ```no_run
//! # async fn example() -> Result<(), hyperliquid_node::replica_rpc::RpcError> {
//! let fetched = hyperliquid_node::replica_rpc::get_block(
//!     "http://127.0.0.1:3000/replica_cmds",
//!     0,
//! )
//! .await?;
//! assert_eq!(fetched.block().height, Some(0));
//! # Ok(())
//! # }
//! ```

pub mod hl_node_outputs;
pub mod replica_rpc;

/// Merkle Patricia Trie (MPT) module
///
/// Clean Architecture implementation of MPT
///
/// Layer structure:
/// - entities: Core domain entities (Node, Path, etc.)
/// - usecases: Use case interfaces (Insert, Get, Delete, Prove)
/// - trie: MPT core implementation
/// - storage: Storage abstraction layer
/// - persistent_storage: File-based persistent storage
/// - block_data: Ethereum block data structures
/// - block_persistence_example: Block persistence demo
/// - example: Usage examples

pub mod entities;
pub mod usecases;
pub mod trie;
pub mod storage;
pub mod example;
pub mod persistent_storage;
pub mod block_data;
pub mod block_persistence_example;

pub use entities::{Node, MptError, MptResult};
pub use usecases::{MptUseCases, InsertUseCase, GetUseCase, DeleteUseCase, ProveUseCase};
pub use trie::MerklePatriciaTrie;
pub use storage::{Storage, InMemoryStorage};
pub use persistent_storage::PersistentStorage;
pub use block_data::{Block, BlockHeader, Transaction, Receipt};
pub use block_persistence_example::run_block_persistence_example;

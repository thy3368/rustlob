/// Merkle Patricia Trie (MPT) module
///
/// Clean Architecture implementation of MPT
///
/// Layer structure:
/// - entities: Core domain entities (Node, Path, etc.)
/// - usecases: Use case interfaces (Insert, Get, Delete, Prove)
/// - trie: MPT core implementation
/// - storage: Storage abstraction layer
/// - example: Usage examples

pub mod entities;
pub mod usecases;
pub mod trie;
pub mod storage;
pub mod example;

pub use entities::{Node, MptError, MptResult};
pub use usecases::{MptUseCases, InsertUseCase, GetUseCase, DeleteUseCase, ProveUseCase};
pub use trie::MerklePatriciaTrie;
pub use storage::{Storage, InMemoryStorage};

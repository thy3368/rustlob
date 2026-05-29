use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use dex::adapter::rust_vm_runtime::RustVmRuntimeAdapter;
use l1_adapter::{EvmRuntimeAdapter, InMemoryMempool, MdbxStateStore};
use l1_core::{PendingRequest, VmKind, VmRegistry};

use crate::service::{AppState, L1E2eService};

pub fn build_app_state() -> Result<AppState, db_repo::StorageError> {
    let state_path = std::env::var("L1_E2E_MDBX_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| std::env::current_dir().unwrap().join("tmp/l1_e2e_mdbx"));

    std::fs::create_dir_all(&state_path)
        .map_err(|err| db_repo::StorageError::Open { source: Box::new(err) })?;

    let mempool = InMemoryMempool::new();
    let state_store = Arc::new(Mutex::new(MdbxStateStore::open(&state_path)?));
    let spot_book_path = state_path.join("dex_spot_book");
    let rust_vm_runtime =
        Arc::new(RustVmRuntimeAdapter::with_spot_order_book_path(&spot_book_path)?);

    let mut vm_registry = VmRegistry::<PendingRequest>::new();
    vm_registry.register_runtime(VmKind::RustVm, rust_vm_runtime.clone());
    vm_registry.register_runtime(VmKind::Evm, Arc::new(EvmRuntimeAdapter::new()));

    let service = L1E2eService::new(mempool, state_store, vm_registry, rust_vm_runtime, 100);

    Ok(AppState { service: Arc::new(service) })
}

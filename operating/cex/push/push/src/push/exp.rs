use std::sync::atomic::Ordering;
use std::thread;

use actix_rt::{Arbiter, ArbiterHandle, System};
use tokio::sync::mpsc;

#[cfg(all(target_os = "linux", feature = "io-uring"))]
#[allow(clippy::new_without_default)]
pub fn new() -> Arbiter {
    let sys = System::current();
    let system_id = sys.id();
    let arb_id = COUNT.fetch_add(1, Ordering::Relaxed);

    let name = format!("actix-rt|system:{}|arbiter:{}", system_id, arb_id);
    let (tx, rx) = mpsc::unbounded_channel();

    let (ready_tx, ready_rx) = std::sync::mpsc::channel::<()>();

    let thread_handle = thread::Builder::new()
        .name(name.clone())
        .spawn({
            let tx = tx.clone();
            move || {
                let hnd = ArbiterHandle::new(tx);

                System::set_current(sys);

                HANDLE.with(|cell| *cell.borrow_mut() = Some(hnd.clone()));

                // register arbiter
                let _ = System::current().tx().send(SystemCommand::RegisterArbiter(arb_id, hnd));

                ready_tx.send(()).unwrap();

                // run arbiter event processing loop
                tokio_uring::start(ArbiterRunner { rx });

                // deregister arbiter
                let _ = System::current().tx().send(SystemCommand::DeregisterArbiter(arb_id));
            }
        })
        .unwrap_or_else(|err| panic!("Cannot spawn Arbiter's thread: {name:?}: {err:?}"));

    ready_rx.recv().unwrap();

    Arbiter { tx, thread_handle }
}

use kameo::actor::ActorRef;
/// Kameo Framework Simple Example
///
/// Kameo is a lightweight actor framework for Rust
/// This example demonstrates a simple Counter actor
///
/// Run with: cargo run --bin kameo_simple
use kameo::actor::Spawn;
use kameo::message::{Context, Message};
use kameo::Actor;

/// Counter actor that maintains a simple count
#[derive(Actor)]
pub struct Counter {
    count: i32,
}

impl Counter {
    pub fn new() -> Self {
        Self { count: 0 }
    }
}

/// Message to increment the counter
pub struct Increment;

impl Message<Increment> for Counter {
    type Reply = i32;

    async fn handle(
        &mut self,
        _msg: Increment,
        _ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        //todo 通常在这里引用领域服务，重要！！！！
        //todo service.handle(command)/service.async_handle(command)
        self.count += 1;
        self.count
    }
}

/// Message to get current count
pub struct GetCount;

impl Message<GetCount> for Counter {
    type Reply = i32;

    async fn handle(
        &mut self,
        _msg: GetCount,
        _ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        self.count
    }
}

/// Message to reset counter
pub struct Reset;

impl Message<Reset> for Counter {
    type Reply = ();

    async fn handle(&mut self, _msg: Reset, _ctx: &mut Context<Self, Self::Reply>) -> Self::Reply {
        self.count = 0;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Kameo Counter Example ===\n");

    // Spawn the counter actor
    let counter_ref: ActorRef<Counter> = Counter::spawn(Counter::new());
    println!("✓ Counter actor spawned");

    // Increment counter
    let count = counter_ref.ask(Increment).send().await?;
    println!("After increment: {}", count);

    let count = counter_ref.ask(Increment).send().await?;
    println!("After increment: {}", count);

    let count = counter_ref.ask(Increment).send().await?;
    println!("After increment: {}", count);

    // Get current count
    let count = counter_ref.ask(GetCount).send().await?;
    println!("\nCurrent count: {}", count);

    counter_ref.tell(GetCount).send().await?;

    // Reset counter
    counter_ref.ask(Reset).send().await?;
    println!("✓ Counter reset");

    // Get count after reset
    let count = counter_ref.ask(GetCount).send().await?;
    println!("Count after reset: {}", count);

    println!("\n=== Example completed ===");
    Ok(())
}

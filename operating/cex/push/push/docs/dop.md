面向对象与并行计算：冲突与协同

面向对象编程（OOP）和并行计算不冲突，但需要谨慎设计。它们可以很好地协同工作，但传统的 OOP 设计模式在某些方面与并行计算存在矛盾。

1. 潜在冲突点

1.1 共享可变状态

// 传统 OOP：对象封装状态
class BankAccount {
private double balance;  // 共享状态

    public void deposit(double amount) {
        balance += amount;  // 并发修改危险
    }
}

// 并行问题：多个线程同时修改同一个账户
let account = BankAccount::new(100.0);
parallel_for i in 0..100 {
account.deposit(1.0);  // 数据竞争！
}


1.2 继承与多态的并发问题

// 基类
class Shape {
public virtual void draw() { /* ... */ }
protected Color color;  // 被所有子类共享
}

// 派生类
class Circle : public Shape {
public override void draw() {
// 访问共享的 color，可能需要同步
}
}

// 并发绘制多个形状
parallel_for shape in shapes {
shape.draw();  // 多个线程访问共享的 color
}


1.3 对象生命周期管理

class ObjectPool {
private List<Connection> connections;

    public Connection acquire() {
        // 线程安全问题：多个线程同时获取连接
        return connections.pop();
    }
}

// 多个线程同时使用连接池
parallel_for i in 0..10 {
let conn = pool.acquire();  // 数据竞争
// 使用连接...
}


2. 解决方案：面向数据的设计

2.1 从 OOP 到 DOP

// 传统 OOP：对象为中心
struct Particle {
position: Vec3,
velocity: Vec3,
mass: f32,
color: Color,

    fn update(&mut self, dt: f32) {
        self.position += self.velocity * dt;
    }
}

// 问题：多个 Particle 对象在内存中分散
let particles = vec![Particle::new(); 10000];
parallel_for particle in &mut particles {
particle.update(dt);  // 缓存不友好
}

// 解决方案：面向数据的设计（DOP）
struct ParticleSystem {
// SoA（Structure of Arrays）布局
positions: Vec<Vec3>,    // 连续存储
velocities: Vec<Vec3>,   // 连续存储
masses: Vec<f32>,        // 连续存储
colors: Vec<Color>,      // 连续存储
}

impl ParticleSystem {
fn update_all(&mut self, dt: f32) {
// 缓存友好，适合 SIMD
parallel_for i in 0..positions.len() {
self.positions[i] += self.velocities[i] * dt;
}
}
}


2.2 实体组件系统（ECS）

// ECS：更好的并行化支持
use specs::{Component, System, World, RunNow};
use rayon::prelude::*;

// 组件：纯数据
#[derive(Component)]
struct Position(Vec3);

#[derive(Component)]
struct Velocity(Vec3);

#[derive(Component)]
struct Mass(f32);

// 系统：纯逻辑
struct PhysicsSystem;

impl<'a> System<'a> for PhysicsSystem {
type SystemData = (WriteStorage<'a, Position>,
ReadStorage<'a, Velocity>);

    fn run(&mut self, (mut positions, velocities): Self::SystemData) {
        // 自动并行化查询
        (&mut positions, &velocities)
            .par_join()
            .for_each(|(pos, vel)| {
                *pos += *vel * dt;
            });
    }
}


3. Rust 中的安全并行 OOP

3.1 使用 Rust 的所有权系统

use std::sync::{Arc, Mutex};
use rayon::prelude::*;

// 安全的 OOP 并行设计
struct Account {
balance: Mutex<f64>,
}

impl Account {
fn new(initial: f64) -> Self {
Self {
balance: Mutex::new(initial),
}
}

    fn deposit(&self, amount: f64) {
        let mut balance = self.balance.lock().unwrap();
        *balance += amount;
    }
    
    // 并行安全的转账
    fn transfer(from: &Account, to: &Account, amount: f64) -> bool {
        // 按确定顺序获取锁，避免死锁
        let (first, second) = if from as *const _ < to as *const _ {
            (from, to)
        } else {
            (to, from)
        };
        
        let lock1 = first.balance.lock().unwrap();
        if *lock1 < amount {
            return false;
        }
        
        let lock2 = second.balance.lock().unwrap();
        // 在 Rust 中，编译器防止了这种情况
        // 需要手动管理锁的生命周期
        false // 简化实现
    }
}

fn parallel_deposits() {
let account = Arc::new(Account::new(100.0));

    // 并行存款
    (0..100).into_par_iter().for_each(|i| {
        account.deposit(i as f64);
    });
}


3.2 消息传递代替共享内存

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// Actor 模式：并行的 OOP
struct BankAccount {
balance: f64,
receiver: mpsc::Receiver<Message>,
}

enum Message {
Deposit(f64),
Withdraw(f64, mpsc::Sender<bool>),
GetBalance(mpsc::Sender<f64>),
}

impl BankAccount {
fn new() -> (Self, mpsc::Sender<Message>) {
let (tx, rx) = mpsc::channel();

        let account = Self {
            balance: 0.0,
            receiver: rx,
        };
        
        (account, tx)
    }
    
    fn run(mut self) {
        thread::spawn(move || {
            while let Ok(msg) = self.receiver.recv() {
                match msg {
                    Message::Deposit(amount) => {
                        self.balance += amount;
                    }
                    Message::Withdraw(amount, responder) => {
                        let success = self.balance >= amount;
                        if success {
                            self.balance -= amount;
                        }
                        responder.send(success).unwrap();
                    }
                    Message::GetBalance(responder) => {
                        responder.send(self.balance).unwrap();
                    }
                }
            }
        });
    }
}

// 安全地并行访问
fn main() {
let (account, sender) = BankAccount::new();
account.run();

    // 多个线程发送消息
    let senders: Vec<_> = (0..10)
        .map(|_| sender.clone())
        .collect();
    
    for (i, sender) in senders.into_iter().enumerate() {
        thread::spawn(move || {
            sender.send(Message::Deposit(i as f64 * 10.0)).unwrap();
        });
    }
}


4. 现代并行 OOP 设计模式

4.1 不可变对象模式

```rust
// 不可变对象：天然线程安全
[derive(Clone)]

struct ImmutableAccount {
    balance: f64,
    transactions: Vec<Transaction>,
}

impl ImmutableAccount {
    fn new() -> Self {
        Self {
            balance: 0.0,
            transactions: Vec::new(),
        }
    }
    
    fn deposit(&self, amount: f64) -> Self {
        let mut new = self.clone();
        new.balance += amount;
        new.transactions.push(Transaction::Deposit(amount));
        new
    }
use actix::prelude::*;
use core_affinity;
use std::thread;
use std::time::Duration;

use crate::push::thread_binding::{
    ThreadBoundActorSystem, SystemConfig, BindingRules, BindingRule, ThreadAffinity,
};
use crate::push::push_service_actor::PushService;
use crate::push::connection_types::ConnectionRepo;
use rust_queue::queue::queue::Queue;
use rust_queue::queue::queue_impl::mpmc_queue::MPMCQueue;

// Arbiter 是独立的执行线程
fn arbiter_example() {
    // 创建新的 Arbiter
    let arbiter = Arbiter::new();

    // 在 Arbiter 线程中执行代码
    arbiter.spawn(async move {
        println!("在 Arbiter 线程中执行");

        // 启动 Actor
        let addr = MyActor.start();

        // 发送消息
        let _ = addr.send(MyMessage).await;
    });

    // 获取当前 Arbiter
    if let Some(current) = Arbiter::try_current() {
        println!("当前 Arbiter: {:?}", current);
    }

    // 停止 Arbiter
    arbiter.stop();
}

// 系统默认 Arbiter
fn system_arbiters() {
    let system = System::new();

    // 系统有一个主 Arbiter
    // 注意：actix-rt 2.x 中没有直接的 arbiter() 方法，需要通过其他方式获取

    // 创建额外的 Arbiter
    let extra_arbiters: Vec<_> = (0..3)
        .map(|i| {
            let arbiter = Arbiter::new();
            println!("创建 Arbiter {}: {:?}", i, arbiter);
            arbiter
        })
        .collect();
}

// 简单的测试 Actor
struct MyActor;

impl Actor for MyActor {
    type Context = Context<Self>;
}

struct MyMessage;

impl Message for MyMessage {
    type Result = ();
}

impl Handler<MyMessage> for MyActor {
    type Result = ();

    fn handle(&mut self, _msg: MyMessage, _ctx: &mut Context<Self>) -> Self::Result {
        println!("MyActor 在线程 {:?} 上处理消息", thread::current().id());
    }
}

// 线程绑定系统测试
#[actix::test]
async fn test_thread_bound_actor_system() {
    // 创建系统配置
    let config = SystemConfig {
        thread_count: 4,
        binding_rules: None,
    };

    // 创建线程绑定的 Actor 系统
    let system = ThreadBoundActorSystem::new(config);

    // 启动系统
    system.start().await.unwrap();

    // 创建测试 Actor
    let actor_addr = system.create_actor(MyActor).await.unwrap();

    // 发送消息
    actor_addr.send(MyMessage).await.unwrap();

    println!("测试 Actor 启动成功");
}

// PushService 线程绑定测试
#[actix::test]
async fn test_push_service_actor_binding() {
    // 获取可用的核心 ID
    if let Some(core_ids) = core_affinity::get_core_ids() {
        println!("可用核心: {:?}", core_ids);

        // 创建依赖
        let connection_repo = std::sync::Arc::new(ConnectionRepo::default());
        let change_log_repo = std::sync::Arc::new(MPMCQueue::new());

        // 创建带有 CPU 亲和性的 PushService
        let push_service = PushService::with_affinity(
            connection_repo,
            change_log_repo,
            core_ids[0],
        );

        // 启动 Actor
        let addr = push_service.start();

        assert!(addr.connected());

        println!("PushService 成功绑定到 CPU 核心 0");
    } else {
        println!("无法获取核心 ID，可能是不支持的平台");
    }
}

// 绑定规则配置测试
#[actix::test]
async fn test_binding_rules() {
    // 创建绑定规则
    let binding_rules = BindingRules::new()
        .bind_actor::<MyActor>(BindingRule::Fixed(0))
        .bind_actor::<PushService>(BindingRule::Affinity(
            ThreadAffinity::new(vec![1, 2])
                .with_fallback(vec![3])
                .with_strict(true),
        ));

    // 创建系统配置
    let config = SystemConfig {
        thread_count: 4,
        binding_rules: Some(binding_rules),
    };

    // 创建线程绑定的 Actor 系统
    let system = ThreadBoundActorSystem::new(config);

    // 启动系统
    system.start().await.unwrap();

    println!("系统绑定规则配置成功");
}

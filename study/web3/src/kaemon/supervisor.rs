use kameo::actor::{ActorRef, Spawn};
use kameo::message::{Context, Message};
use kameo::Actor;
use std::collections::VecDeque;
use tokio::time::{sleep, Duration};

// 定义消息类型
#[derive(Debug, Clone)]
struct ProcessImage {
    image_id: String,
    target_size: (u32, u32),
}

// 定义 Worker Actor
#[derive(Actor)]
struct ImageProcessor {
    worker_id: usize,
    failure_count: usize, // 记录失败次数，用于模拟故障
}

impl Message<ProcessImage> for ImageProcessor {
    type Reply = String;

    async fn handle(
        &mut self,
        msg: ProcessImage,
        _ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        println!("Worker {} 开始处理图片: {}", self.worker_id, msg.image_id);

        // 模拟随机故障：每处理5个任务就有1个失败
        self.failure_count += 1;
        if self.failure_count % 5 == 0 {
            // 模拟故障，返回错误信息
            return format!("错误: Worker {} 处理图片时发生内部错误", self.worker_id);
        }

        // 模拟图片处理耗时
        sleep(Duration::from_millis(100)).await;

        let result = format!(
            "图片 {} 已调整为 {}x{}",
            msg.image_id, msg.target_size.0, msg.target_size.1
        );
        println!("Worker {} 完成: {}", self.worker_id, result);

        result
    }
}

// 任务提交消息
#[derive(Debug, Clone)]
struct SubmitTask(ProcessImage);

// 获取统计信息消息
#[derive(Debug, Clone)]
struct GetStats;

// Supervisor Actor - 管理整个 Worker Pool
#[derive(Actor)]
struct ImageProcessingSupervisor {
    workers: Vec<ActorRef<ImageProcessor>>,
    pending_tasks: VecDeque<ProcessImage>,
    max_workers: usize,
    current_worker_index: usize, // 用于轮询负载均衡
}

impl Message<SubmitTask> for ImageProcessingSupervisor {
    type Reply = String;

    async fn handle(
        &mut self,
        msg: SubmitTask,
        _ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        let task = msg.0;
        println!("收到新图片处理任务: {}", task.image_id);

        // 寻找可用的 Worker（轮询负载均衡）
        if !self.workers.is_empty() {
            let worker = &self.workers[self.current_worker_index];
            self.current_worker_index = (self.current_worker_index + 1) % self.workers.len();

            // 发送任务到 Worker
            match worker.ask(task.clone()).await {
                Ok(result) => result,
                Err(e) => {
                    println!("发送任务失败: {:?}", e);
                    self.pending_tasks.push_back(task);
                    format!("发送失败: {:?}", e)
                }
            }
        } else {
            println!("无可用 Worker，任务进入等待队列");
            self.pending_tasks.push_back(task);
            "无可用 Worker".to_string()
        }
    }
}

impl Message<GetStats> for ImageProcessingSupervisor {
    type Reply = String;

    async fn handle(
        &mut self,
        _msg: GetStats,
        _ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        format!(
            "Workers: {}, Pending: {}, Active: {}",
            self.workers.len(),
            self.pending_tasks.len(),
            self.workers.len()
        )
    }
}

impl ImageProcessingSupervisor {
    fn new(max_workers: usize) -> Self {
        Self {
            workers: Vec::new(),
            pending_tasks: VecDeque::new(),
            max_workers,
            current_worker_index: 0,
        }
    }

    // 初始化 Worker Pool
    fn initialize_pool(&mut self) {
        println!("初始化 Worker Pool，创建 {} 个 Worker", self.max_workers);

        for i in 0..self.max_workers {
            let worker = ImageProcessor {
                worker_id: i,
                failure_count: 0,
            };

            let worker_ref = ImageProcessor::spawn(worker);
            self.workers.push(worker_ref);
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建 Supervisor
    let mut supervisor = ImageProcessingSupervisor::new(3);
    supervisor.initialize_pool();

    let supervisor_ref = ImageProcessingSupervisor::spawn(supervisor);

    // 等待 Supervisor 初始化完成
    sleep(Duration::from_millis(100)).await;

    // 提交多个图片处理任务
    let tasks = vec![
        ProcessImage {
            image_id: "img1.jpg".to_string(),
            target_size: (800, 600),
        },
        ProcessImage {
            image_id: "img2.jpg".to_string(),
            target_size: (1024, 768),
        },
        ProcessImage {
            image_id: "img3.jpg".to_string(),
            target_size: (640, 480),
        },
        ProcessImage {
            image_id: "img4.jpg".to_string(),
            target_size: (1920, 1080),
        },
        ProcessImage {
            image_id: "img5.jpg".to_string(),
            target_size: (1280, 720),
        },
        ProcessImage {
            image_id: "img6.jpg".to_string(),
            target_size: (800, 600),
        },
    ];

    // 提交任务
    let mut handles = Vec::new();
    for task in tasks {
        let supervisor_clone = supervisor_ref.clone();
        let handle = tokio::spawn(async move {
            match supervisor_clone.ask(SubmitTask(task)).await {
                Ok(result) => println!("最终结果: {:?}", result),
                Err(e) => eprintln!("提交任务失败: {:?}", e),
            }
        });
        handles.push(handle);
    }

    // 等待所有任务完成
    for handle in handles {
        handle.await?;
    }

    // 查看统计信息
    sleep(Duration::from_millis(500)).await;
    match supervisor_ref.ask(GetStats).await {
        Ok(stats) => {
            println!("\nWorker Pool 状态: {}", stats);
        },
        Err(e) => eprintln!("发送统计请求失败: {:?}", e),
    }

    // 保持运行以便观察
    sleep(Duration::from_secs(2)).await;

    Ok(())
}

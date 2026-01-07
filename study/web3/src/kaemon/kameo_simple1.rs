use kameo::actor::Spawn;
use kameo::message::{Context, Message};
use kameo::Actor;
use std::path::PathBuf;

// 1. 定义转码消息
pub struct Transcode {
    pub input_path: PathBuf,
    pub output_format: String,
}

// 2. 定义Actor状态
#[derive(Actor)]
pub struct VideoProcessor {
    // 可以在这里保存处理器ID、负载状态等
    worker_id: usize,
}

// 3. 为Actor实现消息处理
impl Message<Transcode> for VideoProcessor {
    type Reply = Result<String, String>; // 返回转码结果或错误

    async fn handle(
        &mut self,
        msg: Transcode,
        _ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        // 模拟转码工作，例如耗时计算
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        println!(
            "Worker {} 成功处理: {:?} -> .{}",
            self.worker_id, msg.input_path, msg.output_format
        );
        Ok(format!("转码成功，由Worker {}完成", self.worker_id))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建一组Actor
    let worker_count = 4;
    let mut workers = Vec::with_capacity(worker_count);

    for i in 0..worker_count {
        let worker_ref = VideoProcessor::spawn(VideoProcessor { worker_id: i });
        workers.push(worker_ref);
    }

    // 模拟10个转码任务
    let tasks = (0..10).map(|i| Transcode {
        input_path: PathBuf::from(format!("video_{}.mp4", i)),
        output_format: "mp4".to_string(),
    });

    // 轮询分发任务
    let mut handles = Vec::new();
    for (task_index, task) in tasks.enumerate() {
        // 通过取模运算决定当前任务分配给哪个Worker
        let target_worker_index = task_index % workers.len();
        let worker_ref = workers[target_worker_index].clone();

        // 使用ask等待响应，收集所有future
        let handle = tokio::spawn(async move {
            match worker_ref.ask(task).await {
                Ok(result) => println!("任务 {} 结果: {:?}", task_index, result),
                Err(e) => eprintln!("任务 {} 失败: {:?}", task_index, e),
            }
        });
        handles.push(handle);
    }

    // 等待所有任务完成
    for handle in handles {
        handle.await?;
    }

    // 给Actor一些时间完成打印
    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

    Ok(())
}

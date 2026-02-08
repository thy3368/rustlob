//! Actix Actor adapter for Agent (Interface/Adapter layer)
//! This module provides Actor capabilities for the domain Agent

use actix::{Actor, Context, Handler, Message, Running};
use actix::fut::wrap_future;
use serde::{Deserialize, Serialize};
use crate::agent::domain::agent::{Agent, AgentResult};
use crate::error;

/// Message for executing a task asynchronously via Actor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunTaskCmd(pub String);

impl Message for RunTaskCmd {
    type Result = error::Result<AgentResult>;
}

/// Legacy synchronous task command (kept for backwards compatibility)
#[derive(Debug, Clone)]
pub struct TaskCmd(pub String);

impl Message for TaskCmd {
    type Result = String;
}

/// Actix Actor implementation for Agent
impl Actor for Agent {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        tracing::info!("Agent actor started");
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> Running {
        tracing::info!("Agent actor stopping");
        Running::Stop
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        tracing::info!("Agent actor stopped");
    }
}

/// Handler for synchronous task commands
impl Handler<TaskCmd> for Agent {
    type Result = String;

    fn handle(&mut self, msg: TaskCmd, _ctx: &mut Self::Context) -> Self::Result {
        tracing::info!("Agent received task command: {}", msg.0);
        format!("Task processed: {}", msg.0)
    }
}

/// Handler for asynchronous task execution
impl Handler<RunTaskCmd> for Agent {
    type Result = actix::ResponseFuture<error::Result<AgentResult>>;

    fn handle(&mut self, msg: RunTaskCmd, _ctx: &mut Self::Context) -> Self::Result {
        tracing::info!("Agent received run task command: {}", msg.0);

        let task = msg.0;

        // Call the async run method and wrap the future
        let future = self.execute_task(task);

        Box::pin(wrap_future(future))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{ModelInfo, ModelResponse, TokenUsage};
    use crate::tools::ToolRegistry;
    use crate::config::Config;
    use crate::permissions::PermissionManager;
    use async_trait::async_trait;
    use std::sync::{Arc, Mutex};

    struct MockModel {
        responses: Vec<String>,
        call_count: Arc<Mutex<usize>>,
    }

    #[async_trait]
    impl crate::model::LanguageModel for MockModel {
        async fn complete(&self, _: &str, _: Option<&str>) -> error::Result<ModelResponse> {
            unimplemented!()
        }

        async fn chat(&self, _: &[crate::model::MMessage]) -> error::Result<ModelResponse> {
            let mut count = self.call_count.lock().unwrap();
            let response = self.responses[*count].clone();
            *count += 1;

            Ok(ModelResponse {
                content: response,
                model: "mock".to_string(),
                usage: TokenUsage::default(),
                tool_calls: None,
                finish_reason: Some("stop".to_string()),
            })
        }

        async fn chat_with_tools(
            &self,
            messages: &[crate::model::MMessage],
            _: &[crate::model::ToolDefinition],
        ) -> error::Result<ModelResponse> {
            self.chat(messages).await
        }

        fn model_info(&self) -> ModelInfo {
            ModelInfo {
                provider: "mock".to_string(),
                model: "test".to_string(),
                max_tokens: 4096,
                supports_tools: false,
                supports_streaming: false,
            }
        }
    }

    #[actix_rt::test]
    async fn test_agent_with_actor() {
        let model = Box::new(MockModel {
            responses: vec!["FINISH".to_string()],
            call_count: Arc::new(Mutex::new(0)),
        });

        let tools = ToolRegistry::new();
        let mut config = Config::default();
        config.safety.require_approval = false;

        let permission_manager = Arc::new(Mutex::new(
            PermissionManager::new().unwrap(),
        ));

        let agent = Agent::new(model, tools, config, Vec::new(), permission_manager)
            .await
            .unwrap();

        let addr = agent.start();

        let result = addr.send(TaskCmd("test task".to_string())).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response, "Task processed: test task");
    }

    #[actix_rt::test]
    async fn test_agent_actor_multiple_messages() {
        let model = Box::new(MockModel {
            responses: vec!["FINISH".to_string()],
            call_count: Arc::new(Mutex::new(0)),
        });

        let tools = ToolRegistry::new();
        let mut config = Config::default();
        config.safety.require_approval = false;

        let permission_manager = Arc::new(Mutex::new(
            PermissionManager::new().unwrap(),
        ));

        let agent = Agent::new(model, tools, config, Vec::new(), permission_manager)
            .await
            .unwrap();

        let addr = agent.start();

        let msg1 = addr.send(TaskCmd("task 1".to_string()));
        let msg2 = addr.send(TaskCmd("task 2".to_string()));
        let msg3 = addr.send(TaskCmd("task 3".to_string()));

        let (result1, result2, result3) = tokio::join!(msg1, msg2, msg3);

        assert!(result1.is_ok());
        assert!(result2.is_ok());
        assert!(result3.is_ok());

        assert_eq!(result1.unwrap(), "Task processed: task 1");
        assert_eq!(result2.unwrap(), "Task processed: task 2");
        assert_eq!(result3.unwrap(), "Task processed: task 3");
    }
}

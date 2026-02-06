use crate::agent::AgentResult;

pub trait AgentInt {
    /// Create a new agent

    /// Run the agent on a task
    async fn run(&mut self, task: &str) -> crate::Result<AgentResult>;
    /// Format a response using the formatter (strip model identity, clean up)
    fn format_response(&self, content: &str) -> String;
}

use minstant::Instant;

///   观测：分支预测; cache 命中率; 时延; qps
pub trait CmdPipeLineHandler: Send + Sync {
    type Command;
    type Reply;

    type Error;

    fn pipeline_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }

    fn do_handle(&self, cmd: &Self::Command) -> Result<Self::Reply, Self::Error>;

    fn handle(&self, cmd: &Self::Command) -> Result<Self::Reply, Self::Error> {
        let start = Instant::now();
        let result = self.do_handle(cmd);
        let elapsed = start.elapsed();
        let elapsed_ns = elapsed.as_nanos();
        let estimated_qps = if elapsed_ns > 0 { 1_000_000_000u128 / elapsed_ns } else { u128::MAX };

        match &result {
            Ok(_) => {
                tracing::info!(
                    pipeline = self.pipeline_name(),
                    elapsed_ns,
                    estimated_qps,
                    success = true,
                    "pipeline handled command"
                );
            }
            Err(_) => {
                tracing::error!(
                    pipeline = self.pipeline_name(),
                    elapsed_ns,
                    estimated_qps,
                    success = false,
                    "pipeline failed"
                );
            }
        }

        result
    }
}

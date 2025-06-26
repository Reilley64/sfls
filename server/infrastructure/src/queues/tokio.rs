use application::ports::queue::JobQueue;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
use tokio::sync::Semaphore;
use application::ports::job::Job;

pub struct TokioQueue {
    sender: UnboundedSender<Box<dyn Job + Send + Sync>>,
}

impl TokioQueue {
    pub fn new() -> Self {
        let (tx, mut rx) = unbounded_channel::<Box<dyn Job + Send + Sync>>();

        tokio::spawn(async move {
            let semaphore = Arc::new(Semaphore::new(num_cpus::get()));

            while let Some(job) = rx.recv().await {
                let semaphore = semaphore.clone();

                tokio::spawn(async move {
                    let _permit = match semaphore.acquire().await {
                        Ok(p) => p,
                        Err(e) => {
                            tracing::error!("failed to acquire semaphore: {}", e);
                            return;
                        }
                    };

                    if let Err(e) = job.run().await {
                        tracing::error!("failed to run job: {}", e);
                    }
                });
            }

            tracing::info!("job queue shutdown");
        });

        tracing::info!("started job queue");
        Self { sender: tx }
    }
}

#[async_trait]
impl JobQueue for TokioQueue {
    async fn enqueue(&self, job: Box<dyn Job + Send + Sync>) -> Result<(), anyhow::Error> {
        self.sender
            .send(job)
            .map_err(|_| anyhow::anyhow!("failed to enqueue job"))?;
        Ok(())
    }
}

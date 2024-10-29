mod bucket;
mod error;

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use anyhow::Result;
pub use bucket::Bucket;
pub use error::RateLimitError;

#[derive(Clone)]
pub struct RateLimiter {
    buckets: Arc<Mutex<HashMap<String, Bucket>>>,
}

impl RateLimiter {
    pub fn new() -> Self {
        Self {
            buckets: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn try_acquire(&self, key: &str) -> Result<()> {
        let mut buckets = self.buckets.lock().await;
        let bucket = buckets
            .entry(key.to_string())
            .or_insert_with(|| Bucket::new());
        let result = if bucket.try_acquire() {
            Ok(())
        } else {
            Err(RateLimitError::LimitExceeded.into())
        };
        result
    }
}

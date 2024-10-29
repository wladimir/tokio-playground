mod error;
mod bucket;

use std::time::Duration;
use anyhow::Result;
use rate_limiter::RateLimiter;

#[tokio::main]
async fn main() -> Result<()> {
    let limiter = RateLimiter::new();
    let key = "test";

    for i in 1..=8 {
        match limiter.try_acquire(key).await {
            Ok(()) => println!("Allowed {}", i),
            Err(e) => println!("Denied {} {}", i, e)
        }

        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    tokio::time::sleep(Duration::from_secs(1)).await;

    for i in 9..=12 {
        match limiter.try_acquire(key).await {
            Ok(()) => println!("Allowed {}", i),
            Err(e) => println!("Denied {} {}", i, e)
        }
    }

    Ok(())
}
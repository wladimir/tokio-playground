use tokio::time::sleep;
use std::time::Duration;
use anyhow::Result;
use rate_limiter::RateLimiter;

#[tokio::test]
async fn test_basic_rate_limiting() -> Result<()> {
    let limiter = RateLimiter::new();
    let key = "test_user";

    for _ in 0..3 {
        limiter.try_acquire(key).await?;
    }

    assert!(limiter.try_acquire(key).await.is_err());

    sleep(Duration::from_secs(1)).await;

    limiter.try_acquire(key).await?;
    Ok(())
}

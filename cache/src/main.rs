use std::sync::Arc;
use anyhow::Result;
use tokio::net::TcpListener;
use simple_cache::Cache;

#[tokio::main]
async fn main() -> Result<()> {
    let cache = Arc::new(Cache::new());
    let listener = TcpListener::bind("127.0.0.1:8081").await?;

    loop {
        let (s, _) = listener.accept().await?;
        let cache = Arc::clone(&cache);

        tokio::spawn(async move {
            if let Err(e) = simple_cache::handle(s, cache).await {
                eprintln!("Error {}", e)
            }
        });
    }
}
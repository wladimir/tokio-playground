use crate::Cache;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

#[derive(Debug, Serialize, Deserialize)]
struct Request {
    value: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Response<T> {
    status: &'static str,
    data: Option<T>,
    error: Option<T>,
}

pub async fn handle(mut stream: TcpStream, cache: Arc<Cache<String, String>>) -> Result<()> {
    let (reader, mut writer) = stream.split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    reader.read_line(&mut line).await?;
    let parts: Vec<&str> = line.trim().split_whitespace().collect();
    if parts.len() < 2 {
        return Err(anyhow::anyhow!("Invalid request format"));
    }

    let method = parts[0];
    let path = parts[1];

    let response = match (method, path) {
        ("GET", path) if path.starts_with("/cache/") => {
            let key = path.trim_start_matches("/cache/").to_string();
            match cache.get(key).await {
                Ok(value) => Response {
                    status: "success",
                    data: Some(value),
                    error: None,
                },
                Err(e) => Response {
                    status: "error",
                    data: None,
                    error: Some(e.to_string()),
                },
            }
        }
        ("POST", path) if path.starts_with("/cache/") => {
            let key = path.trim_start_matches("/cache/").to_string();
            let mut body = String::new();
            reader.read_line(&mut body).await?;

            let req: Request = serde_json::from_str(&body)?;
            match cache.set(key, req.value).await {
                Ok(_) => Response {
                    status: "success",
                    data: None,
                    error: None,
                },
                Err(e) => Response {
                    status: "error",
                    data: None,
                    error: Some(e.to_string()),
                },
            }
        }
        ("DELETE", path) if path.starts_with("/cache/") => {
            let key = path.trim_start_matches("/cache/").to_string();
            match cache.delete(key).await {
                Ok(_) => Response {
                    status: "success",
                    data: None,
                    error: None,
                },
                Err(e) => Response {
                    status: "error",
                    data: None,
                    error: Some(e.to_string()),
                },
            }
        }
        ("DELETE", "/cache") => match cache.clear().await {
            Ok(_) => Response {
                status: "success",
                data: None,
                error: None,
            },
            Err(e) => Response {
                status: "error",
                data: None,
                error: Some(e.to_string()),
            },
        },
        _ => Response {
            status: "error",
            data: None,
            error: Some("Invalid request".to_string()),
        },
    };

    let response_json = serde_json::to_string(&response)?;
    let response_str = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        response_json.len(),
        response_json
    );

    writer.write_all(response_str.as_bytes()).await?;
    Ok(())
}

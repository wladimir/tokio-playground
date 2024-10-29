use crate::error::Result;
use crate::error::ServerError::{ConnectionClosed, IoError};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: impl Into<String>) -> Self {
        Self {
            address: address.into(),
        }
    }

    pub async fn run(&self) -> Result<()> {
        let listener = TcpListener::bind(&self.address).await?;
        println!("Server listening on {}", self.address);

        loop {
            let (socket, address) = listener.accept().await?;
            println!("New connection from: {}", address);

            tokio::spawn(async move {
                if let Err(e) = handle(socket).await {
                    eprintln!("Error {}", e);
                }
            });
        }
    }
}

async fn handle(mut socket: TcpStream) -> Result<()> {
    let mut buffer = [0; 1024];

    loop {
        match socket.read(&mut buffer).await {
            Ok(n) if n == 0 => return Err(ConnectionClosed),
            Ok(n) => {
                socket.write_all(&buffer[..n]).await?;
                socket.flush().await.expect("Failed to flush");
            }
            Err(e) => return Err(IoError(e)),
        };
    }
}

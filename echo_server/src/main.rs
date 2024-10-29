mod error;
mod server;

use server::Server;

#[tokio::main]
async fn main() -> error::Result<()> {
    let server = Server::new("127.0.0.1:8080");
    server.run().await
}

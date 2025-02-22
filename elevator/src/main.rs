#![cfg(windows)]

use crate::elevator::elevator_proto::elevator_server::ElevatorServer;
use crate::elevator::Server;

mod elevator;
mod symlink;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let address = "[::1]:50051".parse()?;
    let server = Server::default();

    tonic::transport::Server::builder()
        .add_service(ElevatorServer::new(server))
        .serve(address)
        .await?;

    Ok(())
}

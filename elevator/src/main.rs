use tokio::sync::mpsc;
use crate::elevator::elevator_proto::elevator_server::ElevatorServer;
use crate::elevator::Server;

mod elevator;
mod symlink;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (tx, mut rx) = mpsc::channel(1);

    let address = "[::1]:50051".parse()?;
    let server = Server::init(tx);

    tonic::transport::Server::builder()
        .add_service(ElevatorServer::new(server))
        .serve_with_shutdown(address, async { let _ = rx.recv().await; })
        .await?;

    Ok(())
}

use std::path::PathBuf;
use tokio::sync::mpsc::Sender;
use tonic::{Request, Response, Status};
use elevator_proto::elevator_server::Elevator;
use crate::elevator::elevator_proto::{SymlinkReply, SymlinkRequest, ShutdownRequest, ShutdownResponse};
use crate::symlink;

pub mod elevator_proto {
    tonic::include_proto!("elevator");
}

#[derive(Debug)]
pub struct Server {
    shutdown_sender: Sender<()>,
}

impl Server {
    pub fn init(shutdown_sender: Sender<()>) -> Server {
        Server { shutdown_sender }
    }
}

#[tonic::async_trait]
impl Elevator for Server {
    async fn create_symlink(
        &self,
        request: Request<SymlinkRequest>
    ) -> Result<Response<SymlinkReply>, Status> {
        let request = request.get_ref();

        let target_path = PathBuf::from(&request.target);
        let sym_path = PathBuf::from(&request.symlink);

        match symlink::create(target_path, sym_path).await {
            Ok(_) => Ok(Response::new(SymlinkReply{})),
            Err(e) => Err(Status::unknown(e.to_string()))
        }
    }

    async fn shutdown_tool(
        &self,
        _request: Request<ShutdownRequest>,
    ) -> Result<Response<ShutdownResponse>, Status> {
        match &self.shutdown_sender.send(()).await {
            Ok(_) => Ok(Response::new(ShutdownResponse {})),
            Err(_) => Err(Status::internal("Error sending shutdown signal")),
        }
    }
}

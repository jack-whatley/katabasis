use std::path::PathBuf;
use tonic::{Request, Response, Status};
use elevator_proto::elevator_server::Elevator;
use crate::elevator::elevator_proto::{SymlinkReply, SymlinkRequest};
use crate::symlink;

pub mod elevator_proto {
    tonic::include_proto!("elevator");
}

#[derive(Debug, Default)]
pub struct Server {}

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
            Err(e) => Err(Status::invalid_argument(e.to_string()))
        }
    }
}

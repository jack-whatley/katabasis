use tokio::process::{Child, Command};
use crate::storage::dir::Directories;
use crate::utils::fs::elevator_proto::elevator_client::ElevatorClient;
use crate::utils::fs::elevator_proto::{ShutdownRequest, SymlinkRequest};

pub mod elevator_proto {
    tonic::include_proto!("elevator");
}

pub struct SymlinkTool {
    process_handle: Child,
    elevator_client: ElevatorClient<tonic::transport::Channel>,
}

impl SymlinkTool {
    pub async fn init() -> crate::Result<Self> {
        let executable = Directories::executable_dir()?.join("elevator.exe");
        let runas = format!("Start-Process {:?} -Verb \"runas\"", executable);

        let mut child = Command::new("powershell.exe")
            .args(&["-NoProfile", "-NonInteractive", "-NoLogo", "-ExecutionPolicy", "Bypass"])
            .args(&["-Command", &runas])
            .spawn()?;

        let mut client: Option<ElevatorClient<tonic::transport::Channel>> = None;

        for _ in 0..30 {
            match ElevatorClient::connect("http://[::1]:50051").await {
                Ok(elevator_client) => {
                    client = Some(elevator_client);
                    break;
                },
                Err(_) => {
                    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                    continue;
                }
            }
        }

        if client.is_none() {
            child.kill().await?;

            return Err(crate::Error::SymlinkToolError(
                "Failed to initialise Symlink Tool in time".to_string()
            ))
        }

        Ok(Self { process_handle: child, elevator_client: client.unwrap() })
    }

    pub async fn create_symlink(
        &mut self,
        target_path: impl Into<String>,
        symlink_path: impl Into<String>,
    ) -> crate::Result<()> {
        let request = tonic::Request::new(SymlinkRequest {
            target: target_path.into(),
            symlink: symlink_path.into()
        });

        let response = self.elevator_client.create_symlink(request).await;

        match response {
            Ok(_) => Ok(()),
            Err(status) => Err(crate::error::Error::SymlinkToolError(status.message().to_owned())),
        }
    }

    pub async fn terminate(mut self) -> crate::Result<()> {
        let response = self.elevator_client
            .shutdown_tool(ShutdownRequest {}).await;

        if response.is_err() {
            return Err(crate::Error::SymlinkToolError("Failed to shutdown elevator".to_string()));
        }

        let process_result = self.process_handle.wait().await?;

        if !process_result.success() {
            return Err(crate::Error::SymlinkToolError("Failed to shutdown elevator".to_string()));
        }

        Ok(())
    }
}

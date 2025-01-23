use tokio::process::{Command, Child};
use crate::storage::dir::Directories;

pub(crate) mod download;

pub struct SymlinkTool {
    elevator_process: Child,
}

impl SymlinkTool {
    pub async fn init() -> crate::Result<Self> {
        let current_path = Directories::executable_dir()?.join("elevator.exe");
        let output = Command::new(current_path)
            .arg("symlink-listener")
            .spawn()?;

        Ok(Self {
            elevator_process: output,
        })
    }
}

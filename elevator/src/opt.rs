use clap::Parser;

#[derive(Parser, Debug)]
pub struct Opt {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Parser, Debug)]
pub enum Command {
    Symlink {
        target: String,

        symlink: String,

        #[arg(long)]
        is_dir: Option<bool>,
    },
    SymlinkListener {
        #[arg(short, long)]
        port: Option<u16>
    }
}

impl Opt {
    pub async fn run(opt: Opt) -> anyhow::Result<()> {
        match opt.command {
            Command::Symlink {
                target,
                symlink,
                is_dir } => {
                if is_dir.is_some() {
                    tokio::task::spawn_blocking(move || {
                        std::os::windows::fs::symlink_dir(&target, &symlink)
                    }).await??;
                }
                else {
                    tokio::task::spawn_blocking(move || {
                        std::os::windows::fs::symlink_file(&target, &symlink)
                    }).await??;
                }
            }
            // Need to open interprocess named pipe "server" here, caller can
            // act as the client in this case
            Command::SymlinkListener { port } => {

            }
        }

        Ok(())
    }
}

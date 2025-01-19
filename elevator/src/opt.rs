use clap::Parser;
use crate::utils;

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
    pub fn run(opt: Opt) -> anyhow::Result<()> {
        match opt.command {
            Command::Symlink {
                target,
                symlink,
                is_dir } => {
                if is_dir.is_some() {
                    std::os::windows::fs::symlink_dir(&target, &symlink)?;
                }
                else {
                    std::os::windows::fs::symlink_file(&target, &symlink)?;
                }
            }
            Command::SymlinkListener { port } => {
                let listener = utils::open_listener(port)?;

                for connection in listener.incoming() {
                    // The idea here is to just accept one connection at a time i.e.
                    // the handling of the connection is blocking and then returns the
                    // next action that should be taken (either keep listening or close)

                    if utils::handle_connection(connection?)? {
                        break;
                    }
                }
            }
        }

        Ok(())
    }
}

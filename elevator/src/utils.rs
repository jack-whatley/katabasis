use interprocess::local_socket::tokio::Stream;
use interprocess::local_socket::traits::tokio::Listener;
use interprocess::local_socket::{GenericNamespaced, ListenerOptions, ToNsName};
use std::path::PathBuf;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::mpsc;

#[derive(Debug, PartialEq)]
pub enum CommandType {
    SYML(String, String),
    EXIT,
    INVALID
}

impl CommandType {
    fn parse(command: &str) -> CommandType {
        if command.len() < 4 { return CommandType::INVALID }
        let command_str = &command[..4];

        match command_str {
            "SYML" => {
                if command.len() < 5 { return CommandType::INVALID }
                let split = command[5..].split(" ").collect::<Vec<&str>>();

                if split.len() != 2 { return CommandType::EXIT }

                CommandType::SYML(split[0].to_string(), split[1].to_string())
            },
            "EXIT" => CommandType::EXIT,
            _ => CommandType::INVALID
        }
    }
}

pub async fn open_listener() -> anyhow::Result<()> {
    let print_name = "kb.elevator.sock";
    let name = print_name.to_ns_name::<GenericNamespaced>()?;

    let ln_opts = ListenerOptions::new().name(name);

    let listener = match ln_opts.create_tokio() {
        Err(err) if err.kind() == std::io::ErrorKind::AddrInUse => {
            // In future need to clean this error up for user (luckily having admin)
            // however for now it will be preferential to log and encourage the user
            // to restart.
            eprintln!("\
                Error: Could not start the elevator server due to the socket file
                being already occupied. Please check if {print_name} is in use, and
                try again; if that doesn't work restarting may help.
            ");

            return Err(err.into());
        }
        ln => ln?,
    };

    // Channel for ending loop
    let (tx, mut rx) = mpsc::channel::<()>(100);

    loop {
        let conn = match listener.accept().await {
            Ok(conn) => conn,
            Err(e) => {
                eprintln!("There was an error with an incoming connection: {e}");
                continue;
            }
        };

        // TODO: use a channel here to communicate if shutdown is requested
        // then can send command via channel and break loop here. For now, it's fine to
        // just call shutdown on process management.

        let tx_clone = tx.clone();

        tokio::spawn(async move {
            if let Err(e) = handle_connection(conn, tx_clone).await {
                eprintln!("Error while handling connection: {e}");
            }
        });
    }

    Ok(())
}

async fn handle_connection(conn: Stream, sig_sender: mpsc::Sender<()>) -> anyhow::Result<()> {
    let mut recver = BufReader::new(&conn);
    let mut sender = &conn;

    // A command length of 1024 characters should be safe due to path
    // size limitations on Windows
    let mut buffer = String::with_capacity(1024);
    let _ = recver.read_line(&mut buffer).await?;

    match CommandType::parse(&buffer.trim()) {
        CommandType::SYML(target, symlink) => {
            let target_path = PathBuf::from(&target);

            if !target_path.exists() {
                sender.write_all(b"File does not exist...\n").await?;
            }
            else {
                if target_path.is_file() {
                    tokio::task::spawn_blocking(move || {
                        std::os::windows::fs::symlink_file(&target, &symlink)
                    }).await??;
                }
                else if target_path.is_dir() {
                    tokio::task::spawn_blocking(move || {
                        std::os::windows::fs::symlink_dir(&target, &symlink)
                    }).await??;
                }
                else {
                    sender.write_all(
                        b"Unable to create symlink for unknown file type.\n"
                    ).await?;
                }
            }
        }
        CommandType::EXIT => {
            sig_sender.send(()).await?;
        }
        CommandType::INVALID => {
            sender.write_all(
                b"Provided invalid command to elevator, valid commands are: 'SYML', 'EXIT'\n"
            ).await?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::CommandType;

    #[test]
    fn test_exit_command_parse() {
        let valid_exit = "EXIT";

        assert_eq!(CommandType::parse(valid_exit), CommandType::EXIT);
    }

    #[test]
    fn test_sym_command_parse() {
        let valid_sym = r#"SYML X:\example\dir X:\target\dir"#;

        assert_eq!(CommandType::parse(valid_sym), CommandType::SYML(r#"X:\example\dir"#.to_string(), r#"X:\target\dir"#.to_string()));
    }

    #[test]
    fn test_invalid_command_parse() {
        let invalid_exit = "@@INVALID_MADE>UP>>>COMMAND";

        assert_eq!(CommandType::parse(invalid_exit), CommandType::INVALID);
    }

    #[test]
    fn test_invalid_command_len() {
        let invalid_len = "ABC";

        assert_eq!(CommandType::parse(invalid_len), CommandType::INVALID);
    }
}

use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use tungstenite::Message;

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

pub fn open_listener(port: Option<u16>) -> anyhow::Result<TcpListener> {
    let port_name = port.unwrap_or(1800);
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port_name))?;

    Ok(listener)
}

pub fn handle_connection(stream: TcpStream) -> anyhow::Result<bool> {
    let mut command_type;
    let mut websocket = tungstenite::accept(stream)?;

    loop {
        let msg = websocket.read()?;

        if msg.is_text() {
            let str_msg = msg.to_string();

            match CommandType::parse(&str_msg) {
                CommandType::EXIT => {
                    command_type = CommandType::EXIT;
                    break;
                }
                CommandType::INVALID => {
                    command_type = CommandType::INVALID;

                    println!("Received invalid command: '{}'", str_msg);

                    websocket.write(
                        Message::text("Message should use a valid command: SYML, EXIT")
                    )?;
                    websocket.flush()?;
                }
                CommandType::SYML(path, symlink) => {
                    let path = PathBuf::from(path);
                    let symlink = PathBuf::from(symlink);

                    // Prefer to raise error here, instead of catch it, so exit code is non-zero
                    if path.is_dir() {
                        std::os::windows::fs::symlink_dir(&path, &symlink)?;
                    }
                    else {
                        std::os::windows::fs::symlink_file(&path, &symlink)?;
                    }
                }
            }
        }
        else {
            websocket.write(Message::text("Message format should be text..."))?;
            websocket.flush()?;
        }

        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    Ok(command_type == CommandType::EXIT)
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

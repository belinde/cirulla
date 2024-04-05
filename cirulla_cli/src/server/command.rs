use super::response::ServiceError;

#[derive(Debug)]
pub enum Command {
    Hello(String),
    Scream(String),
    Error(ServiceError),
    Quit,
    TableNew((String, u8, u8)),
    TableList,
    TableJoin(u8),
    TableLeave,
    Status,
}

impl Command {
    pub fn from_string(input: &str) -> Command {
        let mut parts = input.trim().split_whitespace();

        match parts.next() {
            Some(command) => match command.to_lowercase().as_str() {
                "hello" => {
                    let name = parts.collect::<Vec<&str>>().join(" ");
                    Command::Hello(name)
                }
                "scream" => {
                    let message = parts.collect::<Vec<&str>>().join(" ");
                    Command::Scream(message)
                }
                "status" => Command::Status,
                "quit" => Command::Quit,
                "table" => match parts.next() {
                    Some(sub_command) => match sub_command.to_lowercase().as_str() {
                        "new" => {
                            let mut name = parts.next().unwrap_or_default().to_string();
                            if !name.starts_with('"') {
                                return Command::Error(ServiceError::TableNameNotQuoted);
                            }
                            if !name.ends_with('"') {
                                while let Some(part) = parts.next() {
                                    name.push_str(" ");
                                    name.push_str(part);
                                    if part.ends_with('"') {
                                        break;
                                    }
                                }
                            }
                            let table_name = name.trim_matches('"');

                            let player_max =
                                parts.next().unwrap_or_default().parse::<u8>().unwrap_or(2);

                            let win_at =
                                parts.next().unwrap_or_default().parse::<u8>().unwrap_or(51);

                            Command::TableNew((table_name.to_string(), player_max, win_at))
                        }
                        "list" => Command::TableList,
                        "leave" => Command::TableLeave,
                        "join" => Command::TableJoin(
                            parts.next().unwrap_or_default().parse::<u8>().unwrap_or(0),
                        ),
                        _ => Command::Error(ServiceError::InvalidCommand),
                    },
                    _ => Command::Error(ServiceError::InvalidCommand),
                },
                _ => Command::Error(ServiceError::InvalidCommand),
            },
            _ => Command::Error(ServiceError::InvalidCommand),
        }
    }
}

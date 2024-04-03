use std::str::FromStr;

#[derive(Debug)]
pub enum Command {
    Hello(String),
    Scream(String),
    Error(String),
}

impl FromStr for Command {
    type Err = String;

    fn from_str(input: &str) -> Result<Command, Self::Err> {
        let mut parts = input.split_whitespace();

        match parts.next() {
            Some(command) => match command.to_lowercase().as_str() {
                "hello" => {
                    let name = parts.collect::<Vec<&str>>().join(" ");
                    Ok(Command::Hello(name))
                }
                "scream" => {
                    let message = parts.collect::<Vec<&str>>().join(" ");
                    Ok(Command::Scream(message))
                }
                _ => Ok(Command::Error("unknown command".to_string())),
            },
            _ => Ok(Command::Error("Empty command".to_string())),
        }
    }
}

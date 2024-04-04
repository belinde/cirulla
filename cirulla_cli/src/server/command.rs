#[derive(Debug)]
pub enum Command {
    Hello(String),
    Scream(String),
    Error(String),
    Quit
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
                "quit" => Command::Quit,
                _ => Command::Error("unknown command".to_string()),
            },
            _ => Command::Error("Empty command".to_string()),
        }
    }
}

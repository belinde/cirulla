
#[derive(Debug)]
pub enum Command {
    Hello(String),
    Scream(String),
}

impl Command {
    pub fn from_string(input: String) -> Option<Command> {
        let mut parts = input.split_whitespace();

        match parts.next() {
            Some(command) => match command.to_lowercase().as_str() {
                "hello" => {
                    let name = parts.collect::<Vec<&str>>().join(" ");
                    Some(Command::Hello(name))
                }
                "scream" => {
                    let message = parts.collect::<Vec<&str>>().join(" ");
                    Some(Command::Scream(message))
                }
                _ => None,
            },
            _ => None,
        }
    }
}

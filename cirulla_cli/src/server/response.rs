#[derive(Clone)]
pub enum Response {
    Hi(String),
    Scream((String, String)),
    Error(String),
}

impl ToString for Response {
    fn to_string(&self) -> String {
        match self {
            Response::Hi(name) => format!("HI {}!\n", name),
            Response::Scream((name, message)) => format!("SCREAM FROM {}: {}\n", name, message),
            Response::Error(message) => format!("ERROR: {}\n", message),
        }
    }
}

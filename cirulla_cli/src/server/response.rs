use super::table::TableInfo;
use cirulla_lib::{GameError, GameForPlayer, HandResult};
use std::fmt::Display;

#[derive(Clone, Debug)]
pub enum ServiceError {
    NameInUse,
    NotHello,
    TableNotFound,
    TableNameNotQuoted,
    TableAlreadyJoined,
    InvalidCommand,
    GameError(GameError),
    NotYourTurn,
}

impl Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceError::NameInUse => write!(f, "name already in use"),
            ServiceError::NotHello => write!(f, "you need to say hello first"),
            ServiceError::TableNotFound => write!(f, "table not found"),
            ServiceError::TableNameNotQuoted => write!(f, "table name must be quoted"),
            ServiceError::TableAlreadyJoined => write!(f, "already joined a table"),
            ServiceError::InvalidCommand => write!(f, "invalid command"),
            ServiceError::GameError(err) => write!(f, "game error: {}", err),
            ServiceError::NotYourTurn => write!(f, "not your turn"),
        }
    }
}

#[derive(Clone)]
pub enum Response {
    Hi(String),
    Scream((String, String)),
    Error(ServiceError),
    TableCreated(TableInfo),
    TableJoined(u8),
    TableLeaved(u8),
    TableRemoved(u8),
    TableList(Vec<TableInfo>),
    GameStart(u8),
    GameStatus(GameForPlayer),
    GameEnd,
    HandResult(HandResult),
    Play,
    Wait,
    Status((String, u8)),
}

impl ToString for Response {
    fn to_string(&self) -> String {
        let formatted_msg;
        match self {
            Response::Hi(name) => format!("HI {}\n", name),
            Response::Scream((name, message)) => format!("SCREAM FROM {}: {}\n", name, message),
            Response::Error(code) => format!(
                "ERROR: {}\n",
                match code {
                    ServiceError::NameInUse => "name already in use",
                    ServiceError::NotHello => "you need to say hello first",
                    ServiceError::TableNotFound => "table not found",
                    ServiceError::TableAlreadyJoined => "already joined a table",
                    ServiceError::TableNameNotQuoted => "table name must be quoted",
                    ServiceError::InvalidCommand => "invalid command",
                    ServiceError::GameError(err) => {
                        formatted_msg = format!("game error: {}", err);
                        &formatted_msg
                    }
                    ServiceError::NotYourTurn => "not your turn",
                }
            ),
            Response::TableCreated(info) => format!(
                "TABLE CREATED {} \"{}\" {}/{} {}\n",
                info.id, info.name, info.player_count, info.player_max, info.win_at
            ),
            Response::TableJoined(id) => format!("TABLE JOINED {}\n", id),
            Response::TableLeaved(id) => format!("TABLE LEAVED {}\n", id),
            Response::TableRemoved(id) => format!("TABLE REMOVED {}\n", id),
            Response::Wait => "WAIT\n".to_string(),
            Response::Play => "PLAY\n".to_string(),
            Response::GameStart(id) => format!("GAME START {}\n", id),
            Response::GameStatus(game) => format!(
                "GAME STATUS START\n{}\nGAME STATUS END\n",
                serde_json::to_string_pretty(game).expect("Should serialize")
            ),
            Response::HandResult(result) => format!(
                "HAND RESULT START\n{}\nHAND RESULT END\n",
                serde_json::to_string_pretty(result).expect("Should serialize")
            ),
            Response::TableList(list) => {
                let mut response = "TABLE LIST START\n".to_string();
                for table in list {
                    response.push_str(&format!(
                        "{} \"{}\" {}/{} {}\n",
                        table.id, table.name, table.player_count, table.player_max, table.win_at
                    ));
                }
                response.push_str("TABLE LIST END\n");

                response
            }
            Response::Status((name, table_id)) => format!(
                "STATUS START\nNAME: {}\nJOINED TABLE: {}\nSTATUS END\n",
                name, table_id
            ),
            Response::GameEnd => "GAME END\n".to_string(),
        }
    }
}

impl Response  {
    pub fn from_string(msg: &str) -> Result<Response, ServiceError> {
        let mut parts = msg.trim().splitn(2, ' ');
        match parts.next() {
            Some("HI") => Ok(Response::Hi(parts.next().unwrap_or_default().to_string())),
            Some("SCREAM") => {
                let mut parts = parts.next().unwrap_or_default().splitn(2, ' ');
                Ok(Response::Scream((
                    parts.next().unwrap_or_default().to_string(),
                    parts.next().unwrap_or_default().to_string(),
                )))
            }
            Some("ERROR:") => {
                let error = parts.next().unwrap_or_default();
                match error {
                    "name already in use" => Ok(Response::Error(ServiceError::NameInUse)),
                    "you need to say hello first" => Ok(Response::Error(ServiceError::NotHello)),
                    "table not found" => Ok(Response::Error(ServiceError::TableNotFound)),
                    "table name must be quoted" => Ok(Response::Error(ServiceError::TableNameNotQuoted)),
                    "already joined a table" => Ok(Response::Error(ServiceError::TableAlreadyJoined)),
                    "invalid command" => Ok(Response::Error(ServiceError::InvalidCommand)),
                    "not your turn" => Ok(Response::Error(ServiceError::NotYourTurn)),
                    _ => Ok(Response::Error(ServiceError::InvalidCommand)),
                }
            }
            Some("TABLE") => match parts.next() {
                Some("CREATED") => {
                    let mut parts = parts.next().unwrap_or_default().splitn(4, ' ');
                    let id = parts.next().unwrap_or_default().parse::<u8>().unwrap_or(0);
                    let name = parts.next().unwrap_or_default().trim_matches('"').to_string();
                    let mut parts = parts.next().unwrap_or_default().splitn(2, '/');
                    let player_count = parts.next().unwrap_or_default().parse::<u8>().unwrap_or(0);
                    let player_max = parts.next().unwrap_or_default().parse::<u8>().unwrap_or(0);
                    let win_at = parts.next().unwrap_or_default().parse::<u8>().unwrap_or(0);
                    Ok(Response::TableCreated(TableInfo {
                        id,
                        name,
                        player_count,
                        player_max,
                        win_at,
                    }))
                }
                Some("JOINED") => Ok(Response::TableJoined(
                    parts.next().unwrap_or_default().parse::<u8>().unwrap_or(0),
                )),
                Some("LEAVED") => Ok(Response::TableLeaved(
                    parts.next().unwrap_or_default().parse::<u8>().unwrap_or(0),
                )),
                Some("REMOVED") => Ok(Response::TableRemoved(
                    parts.next().unwrap_or_default().parse::<u8>().unwrap_or(0),
                )),
                Some("LIST") => {
                    let mut tables = Vec::new();
                    loop {
                        let mut parts = parts.next().unwrap_or_default().splitn(4, ' ');
                        let id = parts.next().unwrap_or_default().parse::<u8>().unwrap_or(0);
                        if id == 0 {
                            break;
                        }
                        let name = parts.next().unwrap_or_default().trim_matches('"').to_string();
                        let mut parts = parts.next().unwrap_or_default().splitn(2, '/');
                        let player_count = parts.next().unwrap_or_default().parse::<u8>().unwrap_or(0);
                        let player_max = parts.next().unwrap_or_default().parse::<u8>().unwrap_or(0);
                        let win_at = parts.next().unwrap_or_default().parse::<u8>().unwrap_or(0);
                        tables.push(TableInfo {
                            id,
                            name,
                            player_count,
                            player_max,
                            win_at,
                        });
                    }
                    Ok(Response::TableList(tables))
                }
                _ => Err(ServiceError::InvalidCommand),
            },
            Some("WAIT") => Ok(Response::Wait),
            Some("PLAY") => Ok(Response::Play),
            Some("GAME") => match parts.next() {
                Some("START") => Ok(Response::GameStart(
                    parts.next().unwrap_or_default().parse::<u8>().unwrap_or(0),
                )),
                Some("STATUS") => {
                    let mut game = parts.next().unwrap_or_default().to_string();
                    loop {
                        let mut parts = parts.next().unwrap_or_default().splitn(2, '\n');
                        if parts.next().unwrap_or_default() == "GAME STATUS END" {
                            break;
                        }
                        game.push_str(parts.next().unwrap_or_default());
                    }
                    Ok(Response::GameStatus(
                        serde_json::from_str(&game).expect("Should deserialize"),
                    ))
                }
                Some("END") => Ok(Response::GameEnd),
                _ => Err(ServiceError::InvalidCommand),
            },
            Some("HAND") => match parts.next() {
                Some("RESULT") => {
                    let mut result = parts.next().unwrap_or_default().to_string();
                    loop {
                        let mut parts = parts.next().unwrap_or_default().splitn(2, '\n');
                        if parts.next().unwrap_or_default() == "HAND RESULT END" {
                            break;
                        }
                        result.push_str(parts.next().unwrap_or_default());
                    }
                    Ok(Response::HandResult(
                        serde_json::from_str(&result).expect("Should deserialize"),
                    ))
                }
                _ => Err(ServiceError::InvalidCommand),
            },
            Some("STATUS") => {
                let mut parts = parts.next().unwrap_or_default().splitn(2, '\n');
                let name = parts.next().unwrap_or_default().to_string();
                let table_id = parts.next().unwrap_or_default().parse::<u8>().unwrap_or(0);
                Ok(Response::Status((name, table_id)))
            }
            _ => Err(ServiceError::InvalidCommand),
        }
    }
}

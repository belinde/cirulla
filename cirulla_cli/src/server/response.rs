use super::table::TableInfo;
use cirulla_lib::{GameError, GameForPlayer};

#[derive(Clone, Debug)]
pub enum ServiceError {
    NameInUse,
    NotHello,
    TableNotFound,
    TableNameNotQuoted,
    TableAlreadyJoined,
    InvalidCommand,
    GameError(GameError),
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
        }
    }
}

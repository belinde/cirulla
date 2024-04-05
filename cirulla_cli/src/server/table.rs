use super::response::ServiceError;
use cirulla_lib::Game;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU8, Ordering};

#[derive(Clone)]
pub struct TableInfo {
    pub id: u8,
    pub name: String,
    pub player_count: u8,
    pub player_max: u8,
    pub win_at: u8,
}

pub struct Table {
    pub id: u8,
    pub name: String,
    pub player_max: u8,
    pub game: Game,
    pub sessions_players: HashMap<String, String>,
}

impl Table {
    pub fn new(name: String, player_max: u8, win_at: u8) -> Table {
        static TABLE_ID: AtomicU8 = AtomicU8::new(1);
        Table {
            id: TABLE_ID.fetch_add(1, Ordering::SeqCst),
            name,
            player_max,
            game: Game::new(win_at),
            sessions_players: HashMap::new(),
        }
    }

    pub fn as_info(&self) -> TableInfo {
        TableInfo {
            id: self.id.clone(),
            name: self.name.clone(),
            player_max: self.player_max,
            player_count: self.game.players.len() as u8,
            win_at: self.game.win_at,
        }
    }

    pub fn add_session(
        &mut self,
        session_id: String,
        player_name: String,
    ) -> Result<(), ServiceError> {
        match self.game.add_player(&player_name, Some(session_id.clone())) {
            Ok(key) => {
                self.sessions_players.insert(session_id, key);
                Ok(())
            }
            Err(e) => Err(ServiceError::GameError(e)),
        }
    }

    pub fn remove_session(&mut self, session_id: &str) -> Result<(), ServiceError> {
        if let Some(player_id) = self.sessions_players.get(session_id) {
            return match self.game.remove_player(player_id) {
                Ok(_) => {
                    self.sessions_players
                        .remove(session_id)
                        .expect("The session has been removed by something unexpected");
                    Ok(())
                }
                Err(e) => Err(ServiceError::GameError(e)),
            };
        }
        Ok(())
    }
}

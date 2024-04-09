use super::command::Command;
use super::response::{Response, ServiceError};
use super::session::{Session, SessionCommand};
use super::table::Table;
use cirulla_lib::NextAction;
use log::{debug, info, warn};
use std::collections::HashMap;
use std::sync::{mpsc::channel, Arc, Mutex};
use std::{net::TcpListener, thread};

struct Server {
    sessions: HashMap<String, Session>,
    tables: HashMap<u8, Table>,
}

impl Server {
    pub fn new() -> Server {
        Server {
            sessions: HashMap::new(),
            tables: HashMap::new(),
        }
    }

    pub fn register_session(&mut self, session: Session) -> &Session {
        let id = session.id.clone();
        debug!("Registering session {}", id);
        self.sessions.insert(id.clone(), session);
        self.sessions.get(&id).unwrap()
    }

    fn unregister_session(&mut self, id: &str) {
        debug!("Unregistering session {}", id);
        match self.find_table(id) {
            Some(table_id) => {
                self.tables.remove(&table_id);
                self.broadcast(Response::TableRemoved(table_id));
            }
            None => {}
        }
        self.sessions.get(id).unwrap().disconnect();
        self.sessions.remove(id);
    }

    pub fn execute(&mut self, session_id: &str, command: Command) {
        match command {
            Command::Error(message) => {
                self.error(session_id, message);
            }
            Command::Hello(name) => {
                self.hello(session_id, name);
            }
            Command::Scream(message) => {
                self.scream(session_id, message);
            }
            Command::Quit => {
                self.unregister_session(session_id);
            }
            Command::TableNew((name, player_max, win_at)) => {
                self.table_new(session_id, name, player_max, win_at);
            }
            Command::TableList => {
                self.table_list(session_id);
            }
            Command::TableJoin(table_id) => {
                self.table_join(session_id, table_id);
            }
            Command::TableLeave => {
                self.table_leave(session_id);
            }
            Command::Status => {
                self.status(session_id);
            }
            Command::Play(card) => {
                self.play(session_id, card);
            }
        }
    }

    fn play(&mut self, session_id: &str, card: String) {
        let table_id = self.find_table(session_id).unwrap_or_default();

        match self.tables.get_mut(&table_id) {
            Some(table) => {
                let player_id = table
                    .sessions_players
                    .get(session_id)
                    .expect("Invalid session ID");

                if player_id != &table.game.current_player().id {
                    self.error(session_id, ServiceError::NotYourTurn);
                    return;
                }

                match table.game.player_play(&card) {
                    Ok(_) => {}
                    Err(e) => {
                        self.error(session_id, ServiceError::GameError(e));
                        return;
                    }
                }
                let next_action = table.game.next_round_action();
                debug!("Next action: {:?}", next_action);

                if match next_action {
                    NextAction::NextPlayer => true,
                    NextAction::NextRound => {
                        table.game.start_round().unwrap();
                        true
                    }
                    NextAction::EndHand => {
                        let result = table.game.end_hand().unwrap();
                        let someone_wins = result.someone_wins;
                        table
                            .sessions_players
                            .iter()
                            .for_each(|(session_id, _player_id)| {
                                let session = self
                                    .sessions
                                    .get_mut(session_id)
                                    .expect("Invalid session ID");
                                session.send_response(Response::HandResult(result.clone()));
                            });

                        if someone_wins {
                            table
                                .sessions_players
                                .iter()
                                .for_each(|(session_id, _player_id)| {
                                    let session = self
                                        .sessions
                                        .get_mut(session_id)
                                        .expect("Invalid session ID");
                                    session.send_response(Response::GameEnd);
                                });
                                false
                        } else {
                            table.game.start_hand().unwrap();
                            true
                        }
                    }
                } {

                let active_player = table.game.current_player().id.clone();
                table
                    .sessions_players
                    .iter()
                    .for_each(|(session_id, player_id)| {
                        let session = self
                            .sessions
                            .get_mut(session_id)
                            .expect("Invalid session ID");
                        session.send_response(Response::GameStatus(
                            table.game.as_game_for_player(player_id),
                        ));
                        if player_id == &active_player {
                            session.send_response(Response::Play);
                        } else {
                            session.send_response(Response::Wait);
                        }
                    });
                }
            }
            None => {
                self.error(session_id, ServiceError::TableNotFound);
            }
        }
    }

    fn status(&mut self, session_id: &str) {
        let session = self
            .sessions
            .get_mut(session_id)
            .expect("Invalid session ID");

        let name = match &session.name {
            Some(name) => name.clone(),
            None => "".to_string(),
        };
        let table = self
            .tables
            .values()
            .find(|t| t.sessions_players.contains_key(session_id))
            .map(|t| t.id)
            .unwrap_or(0);

        session.send_response(Response::Status((name, table)));
    }

    fn table_list(&mut self, session_id: &str) {
        let tables = self.tables.values().map(|t| t.as_info()).collect();
        self.sessions
            .get_mut(session_id)
            .expect("Invalid session ID")
            .send_response(Response::TableList(tables));
    }

    fn table_join(&mut self, session_id: &str, table_id: u8) {
        if self
            .tables
            .iter()
            .any(|(_, t)| t.sessions_players.contains_key(session_id))
        {
            self.error(session_id, ServiceError::TableAlreadyJoined);
            return;
        }

        match self.tables.get_mut(&table_id) {
            Some(table) => {
                let session = self
                    .sessions
                    .get_mut(session_id)
                    .expect("Invalid session ID");

                let player_name = match session.name {
                    Some(ref name) => name.clone(),
                    None => {
                        self.error(session_id, ServiceError::NotHello);
                        return;
                    }
                };

                match table.add_session(session.id.clone(), player_name) {
                    Ok(_) => {
                        session.send_response(Response::TableJoined(table_id));
                        session.send_response(Response::Wait);
                        self.maybe_table_start_game(table_id);
                    }
                    Err(e) => {
                        self.error(session_id, e);
                    }
                }
            }
            None => {
                self.error(session_id, ServiceError::TableNotFound);
            }
        }
    }

    fn maybe_table_start_game(&mut self, table_id: u8) {
        match self.tables.get_mut(&table_id) {
            Some(table) => {
                if (table.sessions_players.len() as u8) == table.player_max {
                    match table.game.start_game() {
                        Ok(_) => {
                            if let Err(e) = table.game.start_hand() {
                                table.sessions_players.iter().for_each(|(session_id, _)| {
                                    self.sessions
                                        .get_mut(session_id)
                                        .expect("Invalid session ID")
                                        .send_response(Response::Error(ServiceError::GameError(
                                            e.clone(),
                                        )));
                                });
                                return;
                            }
                            if let Err(e) = table.game.start_round() {
                                table.sessions_players.iter().for_each(|(session_id, _)| {
                                    self.sessions
                                        .get_mut(session_id)
                                        .expect("Invalid session ID")
                                        .send_response(Response::Error(ServiceError::GameError(
                                            e.clone(),
                                        )));
                                });
                                return;
                            }
                            let active_player = table.game.current_player().id.clone();
                            table
                                .sessions_players
                                .iter()
                                .for_each(|(session_id, player_id)| {
                                    let session = self
                                        .sessions
                                        .get_mut(session_id)
                                        .expect("Invalid session ID");
                                    session.send_response(Response::GameStart(table_id));
                                    session.send_response(Response::GameStatus(
                                        table.game.as_game_for_player(player_id),
                                    ));
                                    if player_id == &active_player {
                                        session.send_response(Response::Play);
                                    } else {
                                        session.send_response(Response::Wait);
                                    }
                                });
                        }
                        Err(e) => self.broadcast(Response::Error(ServiceError::GameError(e))),
                    };
                }
            }
            None => {
                self.broadcast(Response::Error(ServiceError::TableNotFound));
            }
        }
    }

    fn table_new(&mut self, session_id: &str, name: String, player_max: u8, win_at: u8) {
        if self
            .tables
            .iter()
            .any(|(_, t)| t.sessions_players.contains_key(session_id))
        {
            self.error(session_id, ServiceError::TableAlreadyJoined);
            return;
        }

        let session = self
            .sessions
            .get_mut(session_id)
            .expect("Invalid session ID");

        let player_name = match session.name {
            Some(ref name) => name.clone(),
            None => {
                self.error(session_id, ServiceError::NotHello);
                return;
            }
        };

        let mut table = Table::new(name, player_max, win_at);

        match table.add_session(session.id.clone(), player_name) {
            Ok(_) => {
                let table_id = table.id.clone();
                self.tables.insert(table_id.clone(), table);
                let table = self.tables.get(&table_id).unwrap();

                session.send_response(Response::TableJoined(table_id));
                session.send_response(Response::Wait);
                self.broadcast(Response::TableCreated(table.as_info()));
            }
            Err(e) => {
                self.error(session_id, e);
            }
        }
    }

    fn find_table(&self, session_id: &str) -> Option<u8> {
        match self
            .tables
            .values()
            .find(|t| t.sessions_players.contains_key(session_id))
        {
            Some(table) => Some(table.id),
            None => None,
        }
    }

    fn table_leave(&mut self, session_id: &str) {
        if let Some(table_id) = self.find_table(session_id) {
            match self.tables.get_mut(&table_id) {
                Some(table) => match table.remove_session(session_id) {
                    Ok(_) => {
                        self.sessions
                            .get_mut(session_id)
                            .expect("Invalid session ID")
                            .send_response(Response::TableLeaved(table_id));
                        if table.sessions_players.is_empty() {
                            self.tables.remove(&table_id);
                            self.broadcast(Response::TableRemoved(table_id));
                        }
                    }
                    Err(e) => {
                        self.error(session_id, e);
                    }
                },
                None => {
                    self.error(session_id, ServiceError::TableNotFound);
                }
            };
        }
    }

    fn error(&mut self, session_id: &str, code: ServiceError) {
        self.sessions
            .get_mut(session_id)
            .expect("Invalid session ID")
            .send_response(Response::Error(code));
    }

    fn hello(&mut self, session_id: &str, name: String) {
        let existent = self
            .sessions
            .values()
            .filter(|s| s.name == Some(name.clone()))
            .count();

        let session = self
            .sessions
            .get_mut(session_id)
            .expect("Invalid session ID");

        if existent > 0 {
            session.send_response(Response::Error(ServiceError::NameInUse));
            return;
        }

        session.name = Some(name.clone());
        session.send_response(Response::Hi(name.clone()));
    }

    fn scream(&mut self, session_id: &str, message: String) {
        let session = self.sessions.get(session_id).expect("Invalid session ID");
        match &session.name {
            Some(name) => self.broadcast(Response::Scream((name.clone(), message.clone()))),
            None => self.error(session_id, ServiceError::NotHello),
        }
    }

    fn broadcast(&mut self, message: Response) {
        for session in self.sessions.values_mut() {
            session.send_response(message.clone());
        }
    }
}

pub fn start_service(address: String, port: u16) {
    let listener = TcpListener::bind(format!("{}:{}", address, port));
    let server = Arc::new(Mutex::new(Server::new()));

    let (command_sender, command_receiver) = channel::<SessionCommand>();

    let server_clone = server.clone();
    thread::spawn(move || {
        for (session_id, command) in command_receiver {
            server_clone.lock().unwrap().execute(&session_id, command);
        }
    });

    match listener {
        Ok(listener) => {
            info!("Listening on {}:{}", address, port);

            for incoming_stream in listener.incoming() {
                match incoming_stream {
                    Ok(tcp_stream) => {
                        let session = Session::new(tcp_stream, command_sender.clone());
                        server
                            .lock()
                            .unwrap()
                            .register_session(session)
                            .read_commands();
                    }
                    Err(e) => {
                        warn!("Failed to accept connection: {}", e.to_string());
                    }
                }
            }
        }
        Err(e) => {
            panic!("Failed to bind to {}:{}: {}", address, port, e.to_string());
        }
    }
}

use super::command::Command;
use super::response::Response;
use super::session::{Session, SessionCommand};
use log::{debug, info, warn};
use std::collections::HashMap;
use std::sync::{mpsc::channel, Arc, Mutex};
use std::{net::TcpListener, thread};

struct Server {
    sessions: HashMap<String, Session>,
}

impl Server {
    pub fn new() -> Server {
        Server {
            sessions: HashMap::new(),
        }
    }

    pub fn register_session(&mut self, session: Session) -> &Session {
        let id = session.id.clone();
        debug!("Registering session {}", id );
        self.sessions.insert(id.clone(), session);
        self.sessions.get(&id).unwrap()
    }

    fn unregister_session(&mut self, id: &str) {
        debug!("Unregistering session {}", id);
        self.sessions.get(id).unwrap().disconnect();
        self.sessions.remove(id);
        // TODO: notify tables and players
    }

    pub fn execute(&mut self, session_id: &str, command: Command) {
        debug!("Executing command {:?} on session {}", command, session_id);

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
        }
    }

    pub fn error(&mut self, session_id: &str, message: String) {
        self.sessions
            .get_mut(session_id)
            .unwrap()
            .send_response(Response::Error(message));
    }

    fn hello(&mut self, session_id: &str, name: String) {
        let existent = self
            .sessions
            .values()
            .filter(|s| s.name == Some(name.clone()))
            .count();

        let session = self.sessions.get_mut(session_id).unwrap();

        if existent > 0 {
            session.send_response(Response::Error("Name already in use".to_string()));
            return;
        }

        session.name = Some(name.clone());
        session.send_response(Response::Hi(name.clone()));
    }

    fn scream(&mut self, session_id: &str, message: String) {
        let session = self.sessions.get(session_id).unwrap();
        match &session.name {
            Some(name) => self.broadcast(Response::Scream((name.clone(), message.clone()))),
            None => self.error(session_id, "You need to say hello first".to_string()),
        }
    }

    fn broadcast(&mut self, message: Response) {
        for session in self.sessions.values_mut() {
            session.send_response(message.clone());
        }
    }
}

pub fn start_server(address: String, port: u16) {
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
                        server.lock().unwrap().register_session(session).read_commands();
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

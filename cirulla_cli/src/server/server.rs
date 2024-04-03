use super::command::Command;
use super::session::{Session, SessionCommand};
use crate::server::response::Response;
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
        self.sessions.insert(id.clone(), session);
        self.sessions.get(&id).unwrap()
    }

    pub fn execute(&mut self, session_id: &str, command: Command) {
        debug!("Executing command {:?} on session {}", command, session_id);
        let session = self.sessions.get_mut(session_id).unwrap();

        match command {
            Command::Hello(name) => {
                session.send(Response::Hi(name.clone()));
            }
            Command::Scream(message) => {
                for s in self.sessions.values_mut() {
                    s.send(Response::Scream((session_id.to_owned(), message.clone())));
                }
            }
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
                        info!("New connection from {}", tcp_stream.peer_addr().unwrap());
                        let session = Session::new(tcp_stream, command_sender.clone());
                        server.lock().unwrap().register_session(session).read();
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

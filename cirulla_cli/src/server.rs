use log::{debug, info, warn};
use std::collections::HashMap;
use std::sync::mpsc::Sender;
use std::sync::{mpsc, Arc, Mutex};
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
};

#[derive(Debug)]
enum Command {
    Hello(String),
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
                _ => None,
            },
            _ => None,
        }
    }
}

type SessionCommand = (String, Command);

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

        match command {
            Command::Hello(name) => {
                format!("Hello, {}!\n", name);
            }
        }
    }
}

struct Session {
    pub id: String,
    command_sender: Sender<SessionCommand>,
    stream: TcpStream,
}

impl Session {
    pub fn new(stream: TcpStream, command_sender: Sender<SessionCommand>) -> Session {
        let id = stream.peer_addr().unwrap().to_string();

        Session {
            id,
            stream,
            command_sender,
        }
    }

    pub fn read(&mut self) {
        let sender = self.command_sender.clone();
        let session_id = self.id.clone();
        let reader = self
            .stream
            .try_clone()
            .expect("Failed to clone reading stream");
        let mut reader = BufReader::new(reader);

        thread::spawn(move || {
            loop {
                let mut incoming: Vec<u8> = vec![];

                match reader.read_until(b'\n', &mut incoming) {
                    Ok(num_bytes_read) => {
                        if num_bytes_read == 0 {
                            break;
                        }
                    }
                    Err(e) => {
                        warn!("Failed to read from stream: {}", e.to_string());
                        break;
                    }
                }

                match Command::from_string(String::from_utf8_lossy(&incoming).to_string()) {
                    Some(command) => {
                        sender.send((session_id.clone(), command)).unwrap();
                    }
                    None => {
                        warn!("Failed to parse command");
                    }
                }
            }

            info!("End handle connection - connection closed");
        });
    }
}

pub fn start_server(address: String, port: u16) {
    let listener = TcpListener::bind(format!("{}:{}", address, port));
    let server = Arc::new(Mutex::new(Server::new()));

    let (session_command_sender, session_command_receiver) = mpsc::channel::<SessionCommand>();

    let server_clone = server.clone();
    thread::spawn(move || {
        for (session_id, command) in session_command_receiver {
            server_clone.lock().unwrap().execute(&session_id, command);
        }
    });

    match listener {
        Ok(listener) => {
            info!("Listening on {}:{}", address, port);

            for incoming_stream in listener.incoming() {
                match incoming_stream {
                    Ok(tcp_stream) => {
                        let server = server.clone();
                        let mut session = Session::new(tcp_stream, session_command_sender.clone());
                        session.read();
                        {
                            // This block is to limit the scope of the mutable borrow of server
                            server.lock().unwrap().register_session(session)
                        };
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

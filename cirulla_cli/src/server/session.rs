use super::{command::Command, response::Response};
use log::{info, warn};
use std::{
    io::{prelude::*, BufReader},
    net::TcpStream,
    sync::mpsc::Sender,
    thread,
};

pub type SessionCommand = (String, Command);

pub struct Session {
    pub id: String,
    command_sender: Sender<SessionCommand>,
    stream: TcpStream,
}

impl Session {
    pub fn new(stream: TcpStream, command_sender: Sender<SessionCommand>) -> Session {
        Session {
            id: stream.peer_addr().unwrap().to_string(),
            stream,
            command_sender,
        }
    }

    pub fn send(&mut self, message: Response) {
        self.stream
            .write_all(message.to_string().as_bytes())
            .unwrap();
    }

    pub fn read(&self) {
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
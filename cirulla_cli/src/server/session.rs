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
    pub name: Option<String>,
    command_sender: Sender<SessionCommand>,
    stream: TcpStream,
}

impl Session {
    pub fn new(stream: TcpStream, command_sender: Sender<SessionCommand>) -> Session {
        let id = stream.peer_addr().unwrap().to_string();
        info!("New connection from {}", id);

        Session {
            id,
            name: None,
            stream,
            command_sender,
        }
    }

    pub fn send_response(&mut self, message: Response) {
        self.stream
            .write_all(message.to_string().as_bytes())
            .unwrap();
    }

    pub fn disconnect(&self) {
        info!("Disconnecting session {}", self.id);
        self.stream.shutdown(std::net::Shutdown::Both).unwrap();
    }

    pub fn read_commands(&self) {
        let sender = self.command_sender.clone();
        let session_id = self.id.clone();
        let stream = self
            .stream
            .try_clone()
            .expect("Failed to clone reading stream");
        let mut reader = BufReader::new(stream);

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

                let command = Command::from_string(String::from_utf8_lossy(&incoming).as_ref());
                if let Command::Quit = command {
                    break;
                }
                sender
                    .send((
                        session_id.clone(),
                        command,
                    ))
                    .expect("Failed to send command");
            }

            sender
                .send((session_id.clone(), Command::Quit))
                .expect("Cannot send disconnect command");
            info!("End handle connection - connection closed");
        });
    }
}

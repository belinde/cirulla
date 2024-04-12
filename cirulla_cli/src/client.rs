use crate::server::response::ServiceError;
use crate::server::{command::Command, response::Response};
use crate::ui::UI;
use log::{debug, error, info};
use std::{
    io::{Read, Write},
    net::TcpStream,
};

pub struct Client {
    stream: TcpStream,
    ui: UI,
    name: String,
}

impl Client {
    pub fn new(address: &str, port: u16) -> Client {
        info!("Connecting to server at {}:{}", address, port);
        match TcpStream::connect((address, port)) {
            Ok(stream) => Client {
                stream,
                ui: UI::new(),
                name: String::new(),
            },
            Err(e) => {
                error!("Error connecting to server: {}", e);
                std::process::exit(1);
            }
        }
    }

    pub fn start(&mut self) {
        let mut error: Option<String> = None;
        while self.name.is_empty() {
            match self.ui.ask_for_input("Come ti chiami?", &error) {
                Ok(name) => {
                    debug!("Sending Hello command {}", name);
                    match self.call(Command::Hello(name)) {
                        Ok(response) => {
                            if let Response::Hi(name) = response {
                                info!("Connected as {}", name);
                                self.name = name;
                            }
                        }
                        Err(e) => error = Some(e.to_string()),
                    
                    }
                }
                Err(e) => error = Some(e.to_string()),
            }
        }
    }

    fn call(&mut self, command: Command){
        let msg = command.to_string();
        self.stream.write_all(msg.as_bytes()).unwrap();
    }
}

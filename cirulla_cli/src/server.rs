use log::info;
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

pub fn start_server(address: String, port: u16) {
    let listener = TcpListener::bind(format!("{}:{}", address, port)).unwrap();
    info!("Listening on {}:{}", address, port);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    info!("Handling connection from {}", stream.peer_addr().unwrap());
    
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);
}

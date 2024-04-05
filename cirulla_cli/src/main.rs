mod local;
mod server;
mod ui;

use clap::{Parser, ValueEnum};
use local::LocalGame;
use server::start_service;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Mode {
    /// Launch a server to which people can connect to play
    Server,
    /// Connect to a server to play
    Client,
    /// Play locally on this computer
    Local,
}

#[derive(Parser, Debug)]
#[command(version, about="Play Cirulla on your computer", long_about = None)]
struct Args {
    /// What mode to run the program in
    #[arg(value_enum, default_value_t = Mode::Local)]
    mode: Mode,

    /// The address to listen on (Server mode) or connect to (Client mode)
    #[arg(short, long, default_value = "localhost")]
    address: String,

    /// The port to listen on (Server mode) or connect to (Client mode)
    #[arg(short, long, value_parser = clap::value_parser!(u16).range(1..), default_value_t = 15157)]
    port: u16,

    /// The name of a player (from 2 to 4 times)
    #[arg(short, long)]
    name: Vec<String>,

    /// The number of points to play to
    #[arg(short, long, default_value_t = 51)]
    win_at: u8,
}

fn main() {
    env_logger::init();
    let args = Args::parse();

    match args.mode {
        Mode::Server => {
            start_service(args.address, args.port);
        }
        Mode::Client => {
            println!("Connecting to {}:{}", args.address, args.port);
            panic!("Client mode is not implemented yet");
        }
        Mode::Local => {
            LocalGame::new(&args.name, args.win_at).start();
        }
    }
}

use connection::handle_connection;
use std::net::TcpListener;
use utils::parse_arguments;

pub mod connection;
pub mod utils;
pub mod logs;

fn main() {
    let config = parse_arguments();
    let listener = TcpListener::bind(format!("{}:{}", config.address, config.port)).unwrap();
    println!("Serving files from: {:?}", config.root_dir);
    println!(
        "Server running at http://{}:{}",
        config.address, config.port
    );

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream, &config.root_dir);
    }
}

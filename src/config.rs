use clap::{command, Arg};
use std::path::PathBuf; // Add missing import statement

pub struct ServerConfig {
    pub root_dir: PathBuf,
    pub address: String,
    pub port: u16,
}

pub fn parse_arguments() -> ServerConfig {
    let matches = command!()
        .arg(
            Arg::new("root_dir")
                .short('d')
                .long("dir")
                .value_name("DIR")
                .help("Sets the root directory to serve files from")
                .default_value("."),
        )
        .arg(
            Arg::new("address")
                .short('a')
                .long("address")
                .value_name("ADDRESS")
                .help("Sets the address to bind the server to")
                .default_value("0.0.0.0"),
        )
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .value_name("PORT")
                .help("Sets the port to bind the server to")
                .default_value("8080"),
        )
        .get_matches();

    let root_dir = matches.get_one::<String>("root_dir").unwrap();
    let address = matches.get_one::<String>("address").unwrap();
    let port = matches
        .get_one::<String>("port")
        .unwrap()
        .parse::<u16>()
        .unwrap();

    let config = ServerConfig {
        root_dir: PathBuf::from(root_dir),
        address: address.to_string(),
        port,
    };
    config
}

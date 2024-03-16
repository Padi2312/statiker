use clap::{command, Arg};
use std::path::PathBuf; // Add missing import statement

pub struct ServerConfig {
    pub root_dir: PathBuf,
    pub address: String,
    pub port: u16,
    pub enable_file_upload: bool,
}

pub fn parse_arguments() -> ServerConfig {
    let matches = command!()
        .arg(
            Arg::new("root_dir")
                .short('d')
                .long("dir")
                .value_name("DIR")
                .help("Sets the root directory to serve files from")
                .default_value("./public"),
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
        .arg(
            Arg::new("enable_upload")
                .short('u')
                .long("enable_upload")
                .required(false)
                .num_args(0)
                .default_value("false")
                .default_missing_value("true")
                .help(
                    "Enables file upload at route '/upload' and saves files to root directory of server",
                ),
        )
        .get_matches();

    let root_dir = matches.get_one::<String>("root_dir").unwrap();
    let address = matches.get_one::<String>("address").unwrap();
    let port = matches
        .get_one::<String>("port")
        .unwrap()
        .parse::<u16>()
        .unwrap();
    let enable_file_upload: bool = *matches.get_one::<bool>("enable_upload").unwrap();

    let config = ServerConfig {
        root_dir: PathBuf::from(root_dir),
        address: address.to_string(),
        port,
        enable_file_upload,
    };
    config
}

pub fn get_from_env() -> ServerConfig {
    let root_dir = std::env::var("ROOT_DIR").unwrap_or("./public".to_string());
    let address = std::env::var("ADDRESS")
        .unwrap_or("0.0.0.0".to_owned())
        .to_string();
    let port = std::env::var("PORT").unwrap_or("8080".to_owned()).parse::<u16>().unwrap();
    let enable_file_upload = std::env::var("ENABLE_UPLOAD")
        .unwrap_or("false".to_owned())
        .parse::<bool>()
        .unwrap();

    let config = ServerConfig {
        root_dir: PathBuf::from(root_dir),
        address: address.to_string(),
        port,
        enable_file_upload,
    };
    config
}

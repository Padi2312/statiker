use std::collections::HashMap;

use config::parse_arguments;
use surfer::{
    request::{Method::GET, Request},
    response::Response,
    route,
    server::Server,
};

pub mod config;

async fn index(request: Request) -> Response {
    println!("{:#?}", request);
    let mut headers: HashMap<String, String> = HashMap::new();
    headers.insert("Content-Type".to_string(), "text/html".to_string());
    headers.insert("Server".to_string(), "Statiker".to_string());

    Response::new(200, headers, b"Hello, World!".to_vec())
}

#[async_std::main]
async fn main() {
    let config = parse_arguments();
    let mut server = Server::new(
        config.address.to_string(),
        config.port.to_string(),
    );
    server
        .register_static_dir("/data", Some(config.root_dir.to_str().unwrap_or(".")))
        .await;
    server.register_route(route!(GET, "/", index));
    server.listen().await;
}

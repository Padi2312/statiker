use std::collections::HashMap;

use config::parse_arguments;
use surfer::{
    request::{Method::POST, Request},
    response::Response,
    route,
    server::Server,
};
pub mod config;

async fn upload(request: Request, save_path: String) -> Response {
    let files = request.files;
    for (file_name, file_content) in files {
        async_std::fs::write(format!("{}/{}", save_path, file_name), file_content)
            .await
            .unwrap();
    }

    let mut headers: HashMap<String, String> = HashMap::new();
    let response_data = b"{\"success\": true}".to_vec();
    headers.insert("Content-Type".to_string(), "application/json".to_string());
    headers.insert(
        "Content-Length".to_string(),
        response_data.len().to_string(),
    );
    headers.insert("Server".to_string(), "Statiker".to_string());
    Response::new(200, headers, response_data)
}

#[async_std::main]
async fn main() {
    let config = parse_arguments();
    let mut server = Server::new(config.address.to_string(), config.port.to_string());
    server
        .register_static_dir("/", Some(config.root_dir.to_str().unwrap_or(".")))
        .await;

    let static_dir = config.root_dir.to_str().unwrap_or(".").to_string();
    server.register_route(route!(POST, "/upload", upload, static_dir));
    server.listen().await;
}

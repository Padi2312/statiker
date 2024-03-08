use std::collections::HashMap;

use config::parse_arguments;
use surfer::{
    headers,
    request::{Method::GET, Method::POST, Request},
    response::Response,
    route,
    server::Server,
};
use upload_page::get_upload_html;
pub mod config;
mod upload_page;

async fn upload(request: Request, save_path: String) -> Response {
    let data = request.form_data;
    // Find form field with "file" name and get the files from it
    let file_data = data.iter().find(|&item| item.name == "file");
    if file_data.is_none() {
        return Response::new(
            400,
            headers!(("Content-Type", "text/plain"), ("Server", "Statiker")),
            b"Could not find file property.".to_vec(),
        );
    }
    let file_data = file_data.unwrap();
    let file_name = &file_data.filename.clone().unwrap();
    let file_content = &file_data.data;
    if file_name.is_empty() {
        return Response::new(
            400,
            headers!(("Content-Type", "text/plain"), ("Server", "Statiker")),
            b"Could not find file name.".to_vec(),
        );
    }
    let save_path = format!("{}/{}", save_path, file_name);
    let result = async_std::fs::write(save_path, file_content).await;
    if result.is_err() {
        return Response::new(
            500,
            headers!(("Content-Type", "text/plain"), ("Server", "Statiker")),
            b"Could not save file.".to_vec(),
        );
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

async fn upload_page(_: Request) -> Response {
    let response = Response::new(
        200,
        headers!(("Content-Type", "text/html"), ("Server", "Statiker")),
        get_upload_html().as_bytes().to_vec(),
    );
    response
}

#[async_std::main]
async fn main() {
    let config = parse_arguments();
    let mut server = Server::new(config.address.to_string(), config.port.to_string());
    server
        .register_static_dir("/", Some(config.root_dir.to_str().unwrap_or(".")))
        .await;

    let static_dir = config.root_dir.to_str().unwrap_or(".").to_string();
    if config.enable_file_upload {
        server.register_route(route!(GET, "/upload", upload_page));
        server.register_route(route!(POST, "/upload", upload, static_dir));
    }
    server.listen().await;
}

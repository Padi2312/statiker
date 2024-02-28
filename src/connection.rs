use std::fs;
use std::io::prelude::*;
use std::net::TcpStream;
use std::path::Path;

use crate::logs::Logger;
use crate::utils::get_content_type; // Add missing import statement

pub fn handle_connection(mut stream: TcpStream, root_dir: &Path) {
    let logger = Logger::new();

    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    // Parse the requested path from the HTTP request
    let request = String::from_utf8_lossy(&buffer[..]);
    let request_path = get_requested_path(request.as_ref());
    let request_method = get_request_method(request.as_ref());
    let user_agent = get_user_agent(request.as_ref());

    logger.info(
        format!(
            "{} {} | Client: {} | User-Agent: {}",
            request_method,
            request_path,
            stream.peer_addr().unwrap(),
            user_agent
        )
        .as_str(),
    );

    let file_path = get_file_path(request_path, root_dir);
    let content_type = get_content_type(&file_path);

    // Serve the requested file if it exists, otherwise return a 404 response
    let (status_line, contents) = if file_path.exists() && file_path.is_file() {
        (
            "HTTP/1.1 200 OK",
            fs::read(&file_path).unwrap_or_else(|_| Vec::new()),
        )
    } else {
        ("HTTP/1.1 404 NOT FOUND", b"404 Not Found".to_vec())
    };

    let response = format!(
        "{}\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n",
        status_line,
        contents.len(),
        content_type
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.write_all(&contents).unwrap();
    stream.flush().unwrap();
}

fn get_request_method(request: &str) -> &str {
    request
        .lines()
        .nth(0)
        .unwrap_or("")
        .split_whitespace()
        .nth(0)
        .unwrap_or("N/A")
}

fn get_requested_path(request: &str) -> &str {
    request
        .lines()
        .nth(0)
        .unwrap_or("")
        .split_whitespace()
        .nth(1)
        .unwrap_or("/")
}

fn get_user_agent(request: &str) -> &str {
    request
        .lines()
        .find(|line| line.starts_with("User-Agent:"))
        .unwrap_or_default()
}

fn get_file_path(requested_path: &str, root_dir: &Path) -> std::path::PathBuf {
    let mut file_path = root_dir.to_path_buf();
    if requested_path == "/" {
        file_path.push("index.html");
    } else {
        file_path.push(requested_path.trim_start_matches('/'));
    }
    file_path
}

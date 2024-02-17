use std::fs;
use std::io::prelude::*;
use std::net::TcpStream;
use std::path::Path;

use crate::utils::get_content_type; // Add missing import statement

pub fn handle_connection(mut stream: TcpStream, root_dir: &Path) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    // Parse the requested path from the HTTP request
    let request_line = String::from_utf8_lossy(&buffer[..]);
    let request_line = request_line.lines().next().unwrap_or("");
    let requested_path = request_line.split_whitespace().nth(1).unwrap_or("/");

    // Map the requested path to a filesystem path
    let mut file_path = root_dir.to_path_buf();
    if requested_path == "/" {
        file_path.push("index.html");
    } else {
        file_path.push(requested_path.trim_start_matches('/'));
    }

    // Determine MIME type
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

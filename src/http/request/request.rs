use async_std::net::TcpStream;
use async_std::prelude::*;
use std::collections::HashMap;

use crate::logs::Logger;

#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
    pub http_version: String,
}

impl Request {
    pub fn new() -> Request {
        Request {
            method: String::new(),
            path: String::new(),
            headers: HashMap::new(),
            body: Vec::new(),
            http_version: String::new(),
        }
    }

    pub async fn parse(&mut self, stream: &mut TcpStream) -> Result<(), ()> {
        // First parse the header line to get the method, path and HTTP version
        let mut buffer = [0; 1024];
        let bytes_read = stream.read(&mut buffer).await.unwrap();

        let buffer_request = String::from_utf8_lossy(&buffer[..bytes_read]);
        let mut lines = buffer_request.lines();

        // Parse request line
        let request_line = lines.next();
        if request_line.is_none() {
            return Err(());
        }
        self.process_request_line(&request_line.unwrap());
        self.process_headers(&buffer_request);

        // Read body depending on content length
        let content_length = self.headers.get("Content-Length");
        if content_length.is_some() {
            let content_length = content_length.unwrap().parse::<usize>().unwrap();
            let header_length = buffer_request.find("\r\n\r\n").unwrap_or(0) + 4;
            let body_length = bytes_read - header_length;
            if content_length <= body_length {
                let body_buffer = &buffer[header_length..header_length + content_length];
                self.body = body_buffer.to_vec();
            } else {
                let mut body_buffer = vec![0; content_length];
                // Read the beginning of body being already in the buffer
                body_buffer[..body_length].copy_from_slice(&buffer[header_length..bytes_read]);
                // Read the rest of the body from the stream
                stream
                    .read_exact(&mut body_buffer[body_length..])
                    .await
                    .unwrap();
                self.body = body_buffer.to_vec();
            }
        }

        Ok(())
    }

    fn process_request_line(&mut self, request_line: &str) {
        let mut parts = request_line.split_whitespace();
        self.method = parts.next().unwrap_or("N/A").to_string();
        self.path = parts.next().unwrap_or("/").to_string();
        self.http_version = parts.next().unwrap_or("N/A").to_string();
    }

    fn process_headers(&mut self, headers: &str) {
        for line in headers.lines() {
            if line.is_empty() {
                break;
            }

            if let Some((key, value)) = line.split_once(": ") {
                self.headers.insert(key.to_string(), value.to_string());
            }
        }
    }

    pub fn log(&self, logger: &Logger) {
        let default_user_agent = String::from("N/A").to_string();
        let user_agent = self
            .headers
            .get("User-Agent")
            .unwrap_or(&default_user_agent);

        logger.info(format!("{} {} | User-Agent: {}", self.method, self.path, user_agent).as_str());
    }
}

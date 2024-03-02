use async_std::fs::File;
use async_std::net::TcpListener;
use async_std::net::TcpStream;
use async_std::path::PathBuf;
use async_std::prelude::*;
use futures::StreamExt;
use std::fs;

use crate::http::request::request::Request;
use crate::http::utils::get_content_type;
use crate::logs::Logger;

pub struct Server {
    pub address: String,
    pub port: String,
    pub root_dir: PathBuf,
}

impl Server {
    pub fn new(address: String, port: String, root_dir: String) -> Server {
        let root_dir = PathBuf::from(root_dir);
        Server {
            address,
            port,
            root_dir,
        }
    }

    pub async fn listen(&self) {
        let listener = TcpListener::bind(format!("{}:{}", self.address, self.port))
            .await
            .unwrap();
        println!("Server running at http://{}:{}", self.address, self.port);
        println!(
            "Serving files from: {:?}",
            self.root_dir.to_str().unwrap_or(".").to_string()
        );

        listener
            .incoming()
            .for_each_concurrent(None, move |stream| async move {
                let stream = stream.unwrap();
                self.handle_connection(stream).await;
            })
            .await;
    }

    pub async fn handle_connection(&self, mut stream: TcpStream) {
        let logger = Logger::new();
        let mut request = Request::new();
        if request.parse(&mut stream).await.is_err() {
            logger.error("Error parsing request");
            let status_line = "HTTP/1.1 404 NOT FOUND";
            let content = b"404 Not Found".to_vec();
            let response = self.format_response(status_line, &content.len(), "text/plain");
            stream.write_all(response.as_bytes()).await.unwrap();
            stream.flush().await.unwrap();
        }
        request.log(&logger);

        match request.method.as_str() {
            "GET" => {
                let file_path = self.get_file_path(&request.path);
                let content_type = get_content_type(&file_path);
                // Rest of the code for handling GET requests
                let (status_line, content) =
                    if file_path.exists().await && file_path.is_file().await {
                        (
                            "HTTP/1.1 200 OK",
                            fs::read(&file_path).unwrap_or_else(|_| Vec::new()),
                        )
                    } else {
                        ("HTTP/1.1 404 NOT FOUND", b"404 Not Found".to_vec())
                    };

                let response = self.format_response(status_line, &content.len(), content_type);
                stream.write_all(response.as_bytes()).await.unwrap();
                stream.write_all(&content).await.unwrap();
                stream.flush().await.unwrap();
            }
            "POST" => {
                let content_type = &request.headers.get("Content-Type");
                if content_type.is_some() {
                    let content_type = content_type.unwrap();
                    if content_type == "application/json" {
                        // Rest of the code for handling POST requests
                    } else {
                        let mut file = File::create("uploaded.bin").await.unwrap();
                        file.write_all(&request.body).await.unwrap();
                        let content = b"File uploaded".to_vec();
                        let status_line = "HTTP/1.1 200 OK";
                        let response =
                            self.format_response(status_line, &content.len(), content_type);
                        stream.write_all(response.as_bytes()).await.unwrap();
                        stream.write_all(&content).await.unwrap();
                        stream.flush().await.unwrap();
                    }
                }
                // Code for handling POST requests
            }
            // Add more match arms for other HTTP methods if needed
            _ => {
                // Code for handling unrecognized HTTP methods
            }
        }
    }

    fn format_response(
        &self,
        status_line: &str,
        content_length: &usize,
        content_type: &str,
    ) -> String {
        format!(
            "{}\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n",
            status_line, content_length, content_type
        )
    }

    fn get_file_path(&self, requested_path: &str) -> PathBuf {
        let mut file_path = self.root_dir.clone();
        if requested_path == "/" {
            file_path.push("index.html");
        } else {
            file_path.push(requested_path.trim_start_matches('/'));
        }
        file_path
    }
}

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

        let mut buffer = [0; 1024];
        let bytes_read = stream.read(&mut buffer).await.unwrap();
        // Only convert the part of the buffer that was filled with incoming data
        let buffer_request = String::from_utf8_lossy(&buffer[..bytes_read]);

        let mut request = Request::new();
        request.parse_request(buffer_request.as_ref());
        // print!("{:#?}", &request);

        let default_user_agent = String::from("N/A").to_string();
        let user_agent = &request
            .headers
            .get("User-Agent")
            .unwrap_or(&default_user_agent);

        logger.info(
            format!(
                "{} {} | Client: {} | User-Agent: {}",
                &request.method,
                &request.path,
                stream.peer_addr().unwrap(),
                user_agent
            )
            .as_str(),
        );

        let file_path = self.get_file_path(&request.path);
        let content_type = get_content_type(&file_path);

        // Serve the requested file if it exists, otherwise return a 404 response
        let (status_line, contents) = if file_path.exists().await && file_path.is_file().await {
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

        stream.write_all(response.as_bytes()).await.unwrap();
        stream.write_all(&contents).await.unwrap();
        stream.flush().await.unwrap();
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

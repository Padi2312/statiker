use config::parse_arguments;

use crate::http::server::Server;
pub mod http;
pub mod logs;
pub mod config;

#[async_std::main]
async fn main() {
    let config = parse_arguments();
    let server = Server::new(
        config.address.to_string(),
        config.port.to_string(),
        config.root_dir.to_str().unwrap_or(".").to_string(),
    );
    server.listen().await;
}

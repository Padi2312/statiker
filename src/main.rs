use async_std::net::TcpListener;
use connection::handle_connection;
use futures::StreamExt;
use utils::parse_arguments;
pub mod connection;
pub mod logs;
pub mod utils;

#[async_std::main]
async fn main() {
    let config = parse_arguments();
    let listener = TcpListener::bind(format!("{}:{}", config.address, config.port))
        .await
        .unwrap();
    println!("Serving files from: {:?}", config.root_dir);
    println!(
        "Server running at http://{}:{}",
        config.address, config.port
    );

    let root_dir = config.root_dir.clone(); // Clone the config.root_dir
    listener
        .incoming()
        .for_each_concurrent(None, move |stream| {
            let root_dir = root_dir.clone(); // Clone the root_dir
            async move {
                let stream = stream.unwrap();
                handle_connection(stream, &root_dir).await;
            }
        })
        .await;
}

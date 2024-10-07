use tokio::net::TcpListener;
use tokio::spawn;

use client::handle_client;
mod client;

#[tokio::main]
async fn main() {
    let listener = match TcpListener::bind("127.0.0.1:8080").await {
        Ok(listener) => listener,
        Err(e) => {
            println!("Failed to bind listener: {}", e);
            return;
        }
    };
    println!("Listening on port 8080...");

    // Accept connections in an asynchronous loop
    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                // Spawn a new async task for each connection
                spawn(async move {
                    handle_client(stream).await;
                });
            }
            Err(e) => {
                println!("Couldn't establish a connection: {}", e);
            }
        }
    }
}

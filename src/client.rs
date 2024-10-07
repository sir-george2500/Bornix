use std::str;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub async fn handle_client(mut client_stream: TcpStream) {
    let mut buffer = [0; 512];
    match client_stream.read(&mut buffer).await {
        Ok(_) => {
            // Parse the request
            let request = str::from_utf8(&buffer).unwrap_or_default();
            println!(
                "Received request from {}",
                client_stream.peer_addr().unwrap()
            );
            println!("{}", request);

            // Connect to the backend server asynchronously (assuming backend is running on 127.0.0.1:8081)
            if let Ok(mut backend_stream) = TcpStream::connect("127.0.0.1:8081").await {
                // Send the client request to the backend server
                backend_stream.write_all(request.as_bytes()).await.unwrap();

                // Read the backend server's response
                let mut backend_response = Vec::new();
                backend_stream
                    .read_to_end(&mut backend_response)
                    .await
                    .unwrap();

                // Send the backend response back to the client
                client_stream.write_all(&backend_response).await.unwrap();
                client_stream.flush().await.unwrap();
            } else {
                println!("Failed to connect to the backend server.");
            }
        }
        Err(e) => {
            println!("Failed to read from client connection: {}", e);
        }
    }
}


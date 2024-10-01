use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

pub fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    match stream.read(&mut buffer) {
        Ok(_) => {
            let request = str::from_utf8(&buffer).unwrap_or_default();
            println!("Received request from {}", stream.peer_addr().unwrap());
            println!("{}", request);

            let response = "HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello, World!";
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
        Err(e) => {
            println!("Failed to read from connection: {}", e);
        }
    }
}

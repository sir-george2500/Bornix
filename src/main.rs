use std::net::TcpListener;

use client::handle_client;
mod client;

fn main() {
    let listiner = match TcpListener::bind("127.0.0.1:8080") {
        Ok(listiner) => listiner,
        Err(e) => {
            println!("Fail to bind listiner : {}", e);
            return;
        }
    };
    println!("Print listing on port 8080 ...");

    for stream in listiner.incoming() {
        match stream {
            Ok(stream) => handle_client(stream),
            Err(e) => {
                println!("Couldn't establish a connection {}", e)
            }
        }
    }
}

use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;

fn handle_client(stream: &mut TcpStream) {
    let mut buffer = [0; 128];

    stream.read(&mut buffer).unwrap();
    println!("{:?}", buffer);
}

fn main() -> std::io::Result<()> {
    let mut listener = TcpListener::bind("127.0.0.1:2138")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(&mut stream?);
    }
    Ok(())
}

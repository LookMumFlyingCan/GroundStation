use std::{io, thread};
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;

fn handle_client(stream: &mut TcpStream) {
    let mut buffer = [0; 128];

    let len = stream.read(&mut buffer).unwrap();
    println!("{:?}", &buffer[..len]);
}

fn main() -> std::io::Result<()> {

  thread::spawn(move || {
    let mut listener = TcpListener::bind("127.0.0.1:2138").unwrap();

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(&mut stream.unwrap());
    }
  });

  loop{
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
      let mut st = TcpStream::connect("127.0.0.1:2137")?;

      let mut buf: Vec<u8> = Vec::new();

      for cha in line.unwrap().chars() {
        buf.push(cha as u8);
      }

      st.write(&buf)?;
    }
  }
}

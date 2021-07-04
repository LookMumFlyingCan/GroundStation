use std::{io, thread};
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;

extern crate spmc;

fn handle_client(stream: &mut TcpStream) {
    let mut buffer = [0; 500];

    let len = stream.read(&mut buffer).unwrap();
    println!("{:?} oraz {}", &buffer[..len], String::from_utf8((&buffer[128..]).to_vec()).unwrap());
}

fn main() -> std::io::Result<()> {



  thread::spawn(move || {
    let mut listener = TcpListener::bind("127.0.0.1:2008").unwrap();

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(&mut stream.unwrap());
    }
  });

  loop{
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
      let mut st = TcpStream::connect("127.0.0.1:2009")?;

      let mut buf: Vec<u8> = Vec::new();
      
      let cline = line.unwrap().clone();
      let bline = cline.clone();
      if cline == "bell".to_string() {
        buf.push(7);
        buf.push(7);
      } else {

        for cha in bline.chars() {
          buf.push(cha as u8);
        }
        
      }

      st.write(&buf)?;
    }
  }
}

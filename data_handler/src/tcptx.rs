use crate::{config::Config, serial_handler::{BUFFER_SIZE, SerialHandler}};

use std::net::{Shutdown, TcpStream, Ipv4Addr, IpAddr, TcpListener};
use std::io::prelude::*;
use std::collections::hash_set::HashSet;

#[derive(Clone)]
pub struct Newsletter {
  pub subscribers: HashSet<String>,
  port: u32
}

impl Newsletter{
  pub fn new(conf: &Config) -> Self {
    let mut subs = HashSet::<String>::new();

    for sub in &conf.subscribers {
      subs.insert(sub.clone());
    }

    Self{ subscribers: subs, port: conf.txport }
  }
  
  pub fn send(&self, buffer: &[u8; BUFFER_SIZE]) -> Result<(), &'static str>{
    for sub in &self.subscribers {
      let mut stream = match TcpStream::connect(format!("{}:{}", sub, self.port)) { Ok(x) => x, Err(_) => continue };

      info!("sending {:?} via tcp", buffer);

      match stream.write(buffer) { Err(x) => error!("failed to send via tcp: {}", x), _ => {} };
      match stream.shutdown(Shutdown::Both) { Err(x) => error!("failed to shutdown tcp: {}", x), _ => {} };
    }
    Ok(())
  }

  pub fn subscribe(&mut self, sub: Ipv4Addr){
    let subc = sub.clone();

    self.subscribers.insert(sub.to_string());
    info!("new subscriber sucesfully added {:?}", subc.to_string());
  }

  pub fn receive(port: &mut SerialHandler, rxport: u32) -> Result<(), &'static str>{
    loop{
      for conn in (match TcpListener::bind(format!("127.0.0.1:{}", rxport)) { Ok(x) => x, _ => continue }).incoming() {
        match Newsletter::handle_stream(&mut (match conn {Ok(x) => x, _ => continue}), port) {
          Err(x) => error!("{}", x),
          _ => {}
        };
      }
    }
  }

  pub fn handle_stream(stream: &mut TcpStream, port: &mut SerialHandler) -> Result<(), &'static str>{
    let mut buffer = [0u8; BUFFER_SIZE];

    let len = match stream.read(&mut buffer) {
      Ok(x) => Ok(x),
      _ => Err("failed to read stream")
    }?;
    
    if len > BUFFER_SIZE {
      Err("message too long to recieve")
    } else {
      if buffer[0] == 7u8 && buffer[1] == 7u8 {
        info!("subsciption request recivied for {}", stream.peer_addr().unwrap().ip());
        match (match stream.peer_addr() { Ok(x) => x, _ => return Err("masked subscriber address") }).ip() {
          IpAddr::V4(ip) => port.subscribe(ip),
          IpAddr::V6(_ip) => { Err("this server does not support ipv6 connections") }
        }
      } else {
        info!("recieved {:?} via tcp", buffer);
        port.send_message(buffer)
      }
    }
  }
}

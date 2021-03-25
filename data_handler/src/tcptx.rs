use crate::{config::Config, serial_handler::{BUFFER_SIZE, SerialHandler}};

use std::net::{Shutdown, TcpStream, Ipv4Addr, IpAddr, TcpListener};
use std::io::prelude::*;
use std::collections::hash_set::HashSet;
use tokio_postgres::NoTls;
use tokio;

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
  
  pub fn send(&self, buffer: &[u8; BUFFER_SIZE]) -> Result<(), String> {
    for sub in &self.subscribers {
      let mut stream = match TcpStream::connect(format!("{}:{}", sub, self.port)) { Ok(x) => x, Err(_) => continue };

      info!("sending {:?} via tcp", buffer);


      let buffer_copy: [u8; BUFFER_SIZE] = buffer.clone();

      tokio::runtime::Runtime::new().unwrap().block_on(async {
        match Newsletter::push_db(&buffer_copy).await { Err(x) => error!("failed to push to db: {}", x), _ => {} };
      });

      match stream.write(buffer) { Err(x) => error!("failed to send via tcp: {}", x), _ => {} };
      match stream.shutdown(Shutdown::Both) { Err(x) => error!("failed to shutdown tcp: {}", x), _ => {} };
    }
    Ok(())
  }

  async fn push_db(buffer: &[u8]) -> Result<(), String> {
    info!("lecimy");
    let (client, connection) = 
      match tokio_postgres::connect("host=localhost user=remote password=\'jestemmilosz\' dbname=data", NoTls).await { Ok(x) => Ok(x), Err(x) => Err(format!("could not connect to database {}", x)) }?;

    tokio::spawn(async move {
      if let Err(_e) = connection.await {
        error!("database commit failed");
      }
    });

    match client.execute("INSERT INTO adsb VALUES (CURRENT_TIMESTAMP, $1)", &[ &buffer ]).await { Ok(x) => Ok(x), Err(x) => Err(format!("could not execute command: {}", x)) }?;

    Ok(())
  }

  pub fn subscribe(&mut self, sub: Ipv4Addr){
    let subc = sub.clone();

    self.subscribers.insert(sub.to_string());
    info!("new subscriber sucesfully added {:?}", subc.to_string());
  }

  pub fn receive(port: &mut SerialHandler, rxport: u32) -> Result<(), String>{
    loop{
      for conn in (match TcpListener::bind(format!("127.0.0.1:{}", rxport)) { Ok(x) => x, _ => continue }).incoming() {
        match Newsletter::handle_stream(&mut (match conn {Ok(x) => x, _ => continue}), port) {
          Err(x) => error!("{}", x),
          _ => {}
        };
      }
    }
  }

  pub fn handle_stream(stream: &mut TcpStream, port: &mut SerialHandler) -> Result<(), String>{
    let mut buffer = [0u8; BUFFER_SIZE];

    let len = match stream.read(&mut buffer) {
      Ok(x) => Ok(x),
      _ => Err("failed to read stream")
    }?;
    
    if len > BUFFER_SIZE {
      Err(format!("message too long to recieve"))
    } else {
      if buffer[0] == 7u8 && buffer[1] == 7u8 {
        info!("subsciption request recivied for {}", stream.peer_addr().unwrap().ip());
        match (match stream.peer_addr() { Ok(x) => x, _ => return Err(format!("masked subscriber address")) }).ip() {
          IpAddr::V4(ip) => port.subscribe(ip),
          IpAddr::V6(_ip) => { Err(format!("this server does not support ipv6 connections")) }
        }
      } else {
        info!("recieved {:?} via tcp", buffer);
        port.send_message(buffer)
      }
    }
  }
}

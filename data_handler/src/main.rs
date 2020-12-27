mod serial_handler;
mod config;
mod tcptx;

use serial_handler::SerialHandler;
use config::Config;
use tcptx::Newsletter;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::net::IpAddr;

extern crate pretty_env_logger;
#[macro_use] extern crate log;
use pretty_env_logger::env_logger;

fn main() {
  // Set the log level to be maximal and init logger
  pretty_env_logger::env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

  let c = Config::load("config.json");
  let news = Newsletter::new(&c);

  let mut port = SerialHandler::connect(&c.terminal[0..], c.baudrate, news).unwrap();
  port.init();

  loop{
    let lis = TcpListener::bind(format!("127.0.0.1:{}", c.rxport));

    for conn in lis.unwrap().incoming() {
      match rest(&mut port, &mut conn.unwrap()) {
        Err(_) => error!("message too long to recive"),
        _ => {}
      };
    }
  }
}

fn rest(port: &mut SerialHandler, conn: &mut TcpStream) -> Result<(), ()>{
  let mut buffer = [0; 128];

  let len = match conn.read(&mut buffer) {
    Ok(x) => x,
    _ => return Err(())
  };
  
  if len > 127 {
    error!("message too long to recieve");
    Err(()) 
  } else {

    if buffer[0] == 7u8 && buffer[1] == 7u8 {
      info!("subsciption request recivied for {}", conn.peer_addr().unwrap().ip());
      match conn.peer_addr().unwrap().ip() {
        IpAddr::V4(ip) => port.addsub(ip),
        IpAddr::V6(_ip) => error!("this server does not support ipv6 connections")
      };

      Ok(())
    } else {
      buffer[len] = 10;
      buffer[len+1] = 13;

      let dbuff = &buffer[..len+2];

      info!("recieved {:?} via tcp", dbuff);
      port.send_message(dbuff);

      Ok(()) 
    }
  }
}

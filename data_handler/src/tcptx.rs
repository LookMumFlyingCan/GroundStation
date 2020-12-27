use crate::config::Config;

use std::net::TcpStream;
use std::io::prelude::*;

#[derive(Clone)]
pub struct Newsletter {
  pub subscribers: Vec<String>,
  port: u32
}

impl Newsletter{
  pub fn new(conf: &Config) -> Self {
    let ctport: Vec<String> = conf.subscribers.clone();
    Self{ subscribers: ctport, port: conf.txport }
  }

  pub fn send(&self, buffer: &Vec<u8>) -> std::io::Result<()>{
    for (_i, sub) in self.subscribers.iter().enumerate() {
      let mut stream = TcpStream::connect(format!("{}:{}", sub, self.port))?;

      info!("sending {:?} via tcp", buffer);

      stream.write(buffer)?;
    }
    Ok(())
  }

  pub fn addsub(&mut self, sub: String){
    let subc = sub.clone();

    self.subscribers.push(sub);
    info!("new subscriber sucesfully added {:?}", subc);
  }
}



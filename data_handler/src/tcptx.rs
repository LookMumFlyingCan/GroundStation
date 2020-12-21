use crate::config::Config;

use std::net::TcpStream;
use std::io::prelude::*;

#[derive(Clone)]
pub struct Newsletter {
  subscribers: Vec<String>,
  port: u32
}

impl Newsletter{
  pub fn new(conf: &Config) -> Self {
    let csub = conf.subscribers.clone();
    Self{ subscribers: csub, port: conf.txport }
  }

  pub fn send(&self, buffer: &Vec<u8>) -> std::io::Result<()>{
    for (_i, sub) in self.subscribers.iter().enumerate() {
      let mut stream = TcpStream::connect(format!("{}:{}", sub, self.port))?;

      stream.write(buffer)?;
      //return std::result::Result::Ok(());
    }
    Ok(())
  }
}



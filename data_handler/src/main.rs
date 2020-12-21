mod serial_handler;
mod config;
mod tcptx;

use serial_handler::SerialHandler;
use config::Config;
use tcptx::Newsletter;
use std::io::{self, BufRead};

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

  info!("send {}", port.send_message(&[65,66,67,10,13]));
  
  loop{
    let stdin = io::stdin();
    for _line in stdin.lock().lines() {
    }
  }
}

mod serial_handler;
mod config;
mod tcptx;
mod backend;
mod websocket;

use serial_handler::SerialHandler;
use config::Config;
use tcptx::Tcp;
use crate::websocket::Socket;

use backend::telemetry;

extern crate pretty_env_logger;
#[macro_use] extern crate log;
use pretty_env_logger::env_logger;

fn main() {
  // set the log level to be maximal and init logger
  pretty_env_logger::env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

  info!("{}", std::mem::size_of::<telemetry::Telemetry>());

  // load the config
  let config = Config::load("config.json");

  // start the serial port handler
  let mut port = SerialHandler::connect(&config.terminal[0..], config.baudrate).unwrap();

  // initialize the tcp handler
  let news = Tcp::new(&config, &mut port.tx.clone(), &mut port.rx.clone());

  let sock = Socket::new(&config, &mut port.rx.clone());

  loop{}
  //let sock = Socket::new(&config);

  //drop(config.subscribers);
  // throw the main thread into tcprx as we dont need it anymore
  //Newsletter::receive(&mut port, config.tcp_rxport);
}

mod serial_handler;
mod config;
mod tcptx;
mod backend;

use serial_handler::SerialHandler;
use config::Config;
use tcptx::Newsletter;

extern crate pretty_env_logger;
#[macro_use] extern crate log;
use pretty_env_logger::env_logger;

fn main() {
  // set the log level to be maximal and init logger
  pretty_env_logger::env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

  // load the config
  let config = Config::load("config.json");
  // initialize the tcp handler
  let news = Newsletter::new(&config);

  // start the serial port handler
  let mut port = SerialHandler::connect(&config.terminal[0..], config.baudrate, news).unwrap();

  drop(config.subscribers);
  // throw the main thread into tcprx as we dont need it anymore
  Newsletter::receive(&mut port, config.rxport);
}

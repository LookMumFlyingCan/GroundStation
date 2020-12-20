mod serial_handler;

use serial_handler::SerialHandler;


extern crate pretty_env_logger;
#[macro_use] extern crate log;
use pretty_env_logger::env_logger;

fn main() {
  // Set the log level to be maximal and init logger
  pretty_env_logger::env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();



  let mut port = SerialHandler::connect("/dev/pts/6").unwrap();
  port.init();

  info!("send {}", port.send_message(&[65,66,67,10,13]));
  loop{}
}

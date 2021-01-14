mod adsb;
mod config;
mod uart;

use crate::config::Config;
use crate::adsb::Adsb;
use crate::uart::Uart;
use std::io::{self, BufRead};

extern crate pretty_env_logger;
#[macro_use] extern crate log;
use pretty_env_logger::env_logger;

fn main() {
  // Set the log level to be maximal and init logger
  pretty_env_logger::env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

  let config = Config::load("config.json");
  let mut adsb = match Adsb::new("/home/h39/Downloads/dump1090/dump1090".to_string(), config.gain, 1090) {
    Ok(x) => x,
    Err(x) => {error!("Adsb decoder start failed: {}", x); return;}
  };
  let mut had = Uart::new(adsb);

  let stdin = io::stdin();
  for line in stdin.lock().lines() {
    had.reset("/home/lubuntu/GroundStation/a.out".to_string(), line.unwrap().to_string().parse::<u32>().unwrap(), 1090);
  }
}

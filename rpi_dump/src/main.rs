mod adsb;
mod config;

use crate::config::Config;
use crate::adsb::Adsb;
use std::io::{self, BufRead};

extern crate pretty_env_logger;
#[macro_use] extern crate log;
use pretty_env_logger::env_logger;

fn main() {
  // Set the log level to be maximal and init logger
  pretty_env_logger::env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

  let config = Config::load("config.json");
  let mut adsb = Adsb::new(config.gain);

  let stdin = io::stdin();
  for line in stdin.lock().lines() {
      adsb.reset(line.unwrap().to_string().parse::<u32>().unwrap());
  }
}

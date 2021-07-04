use crate::{config::Config, serial_handler::{BUFFER_SIZE}, backend::decoder::Decoder};

use std::net::{Shutdown, TcpStream, Ipv4Addr, IpAddr, TcpListener};
use std::sync::mpsc;
use std::io::prelude::*;
use std::collections::hash_set::HashSet;
use std::{thread, time};
use tokio_postgres::NoTls;
use tokio;

#[derive(Clone)]
pub struct Tcp {
}

impl Tcp {
  pub fn new(conf: &Config, org_tx: &mut mpsc::Sender<[u8; BUFFER_SIZE]>, org_rx: &mut spmc_buffer::SPMCBufferOutput<[u8; BUFFER_SIZE + 1]>) -> Self {
    let mut subscribers = HashSet::<String>::new();

    for sub in &conf.tcp_subscribers {
      subscribers.insert(sub.clone());
    }

    let (mut ctx, crx): (mpsc::Sender<Ipv4Addr>, mpsc::Receiver<Ipv4Addr>) = mpsc::channel();

    let txport = conf.tcp_txport.clone();
    let rxport = conf.tcp_rxport.clone();
    let timeout = conf.timeout.clone();

    let mut tx = org_tx.clone();
    let mut rx = org_rx.clone();

    let mut counter: u8 = 0;

    thread::spawn(move ||  loop {
        match rx.read() {
            x => {
                if x[BUFFER_SIZE] == counter {
                    thread::sleep(time::Duration::from_millis(timeout as u64));
                    continue;
                }

                counter = x[BUFFER_SIZE];

                match crx.try_recv() {
                    Ok(ip) => {
                        subscribers.insert(ip.to_string());
                    },
                    Err(_) => {}
                };


                for sub in &subscribers {
                  let mut stream = match TcpStream::connect(format!("{}:{}", sub, txport)) { Ok(y) => y, Err(x) => { info!("failed to connect {}", x); continue; } };


                  //let buffer_copy = buffer.clone();

                  //tokio::runtime::Runtime::new().unwrap().block_on(async {
                //    match Newsletter::push_db(&buffer_copy).await { Err(x) => error!("failed to push to db: {}", x), _ => {} };
                  //});

                  let processed_data = match Decoder::decode(&x[..]) {
                      Ok(x) => x,
                      Err(_) => format!("")
                  };

                  info!("sending {:?} and {} via tcp", x, processed_data);

                  match stream.write(&[&x[..BUFFER_SIZE], processed_data.as_bytes()].concat()) { Err(x) => error!("failed to send via tcp: {}", x), _ => {} };
                  match stream.shutdown(Shutdown::Both) { Err(x) => error!("failed to shutdown tcp: {}", x), _ => {} };
                }
            }
        };
    });

    thread::spawn(move || loop {
        loop{
          for conn in (match TcpListener::bind(format!("0.0.0.0:{}", rxport)) { Ok(x) => x, _ => continue }).incoming() {
            match Tcp::handle_stream(&mut (match conn {Ok(x) => x, _ => continue}), &mut tx, &mut ctx) {
              Err(x) => { error!("{}", x); break; },
              _ => {}
            };
          }
        }
    });


    Self{}
  }

  async fn push_db(buffer: &[u8]) -> Result<(), String> {
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

  pub fn handle_stream(stream: &mut TcpStream, port: &mut mpsc::Sender<[u8; BUFFER_SIZE]>, special: &mut mpsc::Sender<Ipv4Addr>) -> Result<(), String>{
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
          IpAddr::V4(ip) => match special.send(ip) { Ok(_) => Ok(()), Err(_) => Err(format!("pipe connection failed")) },
          IpAddr::V6(_ip) => { Err(format!("this server does not support ipv6 connections")) }
        }
      } else {
        info!("recieved {:?} via tcp", &buffer[..len]);

        if len == 0 {
            return Err(format!("cannot send empty packet"));
        }

        match port.send(buffer) {
            Ok(_) => Ok(()),
            Err(x) => Err(format!("failed to send via pipe {}", x))
        }
      }
    }
  }
}

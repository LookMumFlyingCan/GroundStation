use crate::{config::Config, serial_handler::{BUFFER_SIZE, SerialHandler}, backend::decoder::Decoder};

use std::net::{Shutdown, TcpStream, Ipv4Addr, IpAddr, TcpListener};
use std::sync::mpsc;
use std::io::prelude::*;
use std::collections::hash_set::HashSet;
use std::thread;

use websocket::sync::Server;
use websocket::client::sync::Client;
use websocket::OwnedMessage;
use websocket::server::NoTlsAcceptor;

#[derive(Clone)]
pub struct Socket {
}

impl Socket {
  pub fn new(conf: &Config, org_rx: &mut spmc::Receiver<[u8; BUFFER_SIZE]>) -> Self {
    //let subscribers: Vec<Client<NoTlsAcceptor>> = vec![];

    let port = conf.socket_port.clone();

    let mut rx = org_rx.clone();

    //let (mut ctx, crx): (mpsc::Sender<Client<NoTlsAcceptor>>, mpsc::Receiver<Client<NoTlsAcceptor>>) = mpsc::channel();

    thread::spawn(move || {
        let server = Server::bind(format!("127.0.0.1:{}", port)).unwrap();
        for req in server.filter_map(Result::ok){
            let mut chan = req.accept().unwrap();
            match rx.recv() {
                Ok(x) => {
                        /*match crx.try_recv() {
                            Ok(ip) => {
                                subscribers.push(ip);
                            },
                            Err(_) => {}
                        };*/

                        let processed_data = match Decoder::decode(x) {
                            Ok(x) => x,
                            Err(_) => format!("")
                        };

                        let mess = OwnedMessage::Binary([&x[..], processed_data.as_bytes()].concat());
                        chan.send_message(&mess);
                },
                Err(x) => {
                    error!("pipe failed to recieve {}", x);
                    return;
                }
            };
        }
    });

    /*thread::spawn(move || loop {
        match rx.recv() {
            Ok(x) => {
                    match crx.try_recv() {
                        Ok(ip) => {
                            subscribers.push(ip);
                        },
                        Err(_) => {}
                    };

                    let processed_data = match Decoder::decode(x) {
                        Ok(x) => x,
                        Err(_) => format!("")
                    };

                    let mess = OwnedMessage::Binary(&[&x[..], processed_data.as_bytes()].concat());
                    for sub in &subscribers {
                        sub.send_message(&mess);
                    }
            },
            Err(x) => {
                error!("pipe failed to recieve {}", x);
                return;
            }
        };
    });*/


    Self{}
  }
}

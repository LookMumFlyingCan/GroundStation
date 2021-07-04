use crate::{config::Config, serial_handler::{BUFFER_SIZE}, backend::decoder::Decoder};


use std::net::TcpListener;

use std::{thread, time};

//use websocket::sync::Server;
//use websocket::OwnedMessage;

use tungstenite::server::accept;
use tungstenite::Message::Binary;

#[derive(Clone)]
pub struct Socket {
}

impl Socket {
    pub fn run(conf: &Config, org_rx: &mut spmc_buffer::SPMCBufferOutput<[u8; BUFFER_SIZE + 1]>) -> Self {
        let port = conf.socket_port.clone();

        loop{
            for conn in (match TcpListener::bind(format!("0.0.0.0:{}", port)) { Ok(x) => x, _ => continue }).incoming() {
                let timeout = conf.timeout.clone();
                let mut counter: u8 = 0;
                let mut rx = org_rx.clone();

                match conn {
                    Ok(chan) => {
                        info!("conncetion on socket from {}", chan.peer_addr().unwrap());

                        let mut socket = match accept(chan) {
                            Ok(x) => x,
                            Err(_) => continue
                        };

                        info!("socket connection accepted");

                        thread::spawn(move || loop{
                            match rx.read() {
                            x => {
                                if x[BUFFER_SIZE] == counter {
                                    thread::sleep(time::Duration::from_millis(timeout as u64));
                                    continue;
                                }

                                counter = x[BUFFER_SIZE];

                                let processed_data = match Decoder::decode(&x[..]) {
                                    Ok(x) => x,
                                    Err(_) => format!("")
                                };

                                info!("proc: {}", processed_data);

                                info!("sending {:?} via socket", [&x[..BUFFER_SIZE], processed_data.as_bytes()].concat());
                                match socket.write_message(Binary([&x[..BUFFER_SIZE], processed_data.as_bytes()].concat())) {
                                    Err(x) => { warn!("failed to send data via socket: {}", x); break; } ,
                                    _ => {}
                                };
                            }
                        }});
                    },
                    _ => { error!("failed to open connection"); continue; }
                }
            }
        }
    }
}

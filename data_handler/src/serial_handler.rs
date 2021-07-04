use std::io::Write;
use std::time::Duration;
use std::sync::mpsc;
use std::{io, thread};
use std::{boxed, str};


use crate::backend::telemetry;

use spmc;

pub const BUFFER_SIZE: usize = 128usize;

pub struct SerialHandler{
    pub tx: mpsc::Sender<[u8; BUFFER_SIZE]>,
    pub rx: spmc::Receiver<[u8; BUFFER_SIZE]>
}

impl SerialHandler {
    pub fn connect(name: &str, baudrate: u32) -> Result<Self, String> {
        let mut port = match serialport::new(name, baudrate)
        .timeout(Duration::from_millis(5))
        .open() {
            Ok(x) => x,
            Err(_) => return Err(format!("failed to open serial port {}", name))
        };

        let mut serial: boxed::Box<dyn serialport::SerialPort> = match port.try_clone(){
            Err(x) => {
                Err(format!("failed to clone serial port: {}", x))
            },
            Ok(x) => Ok(x)
        }?;

        let (mut stx, mut srx): (spmc::Sender<[u8; BUFFER_SIZE]>, spmc::Receiver<[u8; BUFFER_SIZE]>) = spmc::channel();

        thread::spawn(move || {
            loop {
                let mut buffer = [0u8; BUFFER_SIZE];
                match port.read(&mut buffer) {
                    Ok(x) => {
                        info!("pbuffer {:?}", buffer);
                        match stx.send(buffer) {
                            Err(x) => error!("could not handle message: {}", x),
                            _ => {}
                        };
                    },
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                    Err(e) => error!("serial port recive failed {:?}", e)
                };
            }});


        let (mut tx, mut rx): (mpsc::Sender<[u8; BUFFER_SIZE]>, mpsc::Receiver<[u8; BUFFER_SIZE]>) = mpsc::channel();

        thread::spawn(move || loop{
            let buffer = match rx.recv() {
                Ok(x) => x,
                Err(x) => {
                    error!("mpsc pipe failed to recieve {}", x);
                    return;
                }
            };

            match serial.write(&buffer) {
                Err(x) => info!("could not send serial message {}", x),
                _ => {}
            };
        });

        Ok(Self{ tx: tx, rx: srx })
    }

    pub fn init(&mut self) -> Result<(), String> {


        Ok(())
    }

    pub fn send_message(&mut self, buffer: [u8; BUFFER_SIZE]) -> Result<(), String>{
        info!("sending {:?}", buffer);

        match self.tx.send(buffer) {
            Ok(_) => Ok(()),
            Err(x) => Err(format!("failed to send {}", x))
        }
    }
}

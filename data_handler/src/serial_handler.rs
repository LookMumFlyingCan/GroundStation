use crate::tcptx::Newsletter;
use std::io::Write;
use std::time::Duration;
use std::sync::mpsc;
use std::{io, thread};
use std::boxed;

static mut BUFLEN: u32 = 0u32;

pub struct SerialHandler{
  port: boxed::Box<dyn serialport::SerialPort>,
  comm: Option<mpsc::Sender<u8>>,
  commip: Option<mpsc::Sender<std::net::Ipv4Addr>>,
  news: Newsletter
}

impl SerialHandler{
  pub fn connect(name: &str, baudrate: u32, tcptx: Newsletter) -> std::result::Result<Self, serialport::Error>{
    match serialport::new(name, baudrate)
      .timeout(Duration::from_millis(100))
      .open() {
        Ok(x) => Ok(Self{ port: x, comm: None, commip: None, news: tcptx }),
        Err(x) => {
          error!("failed to open serial port {}", name); Err(x)
        }      
    }
  }

  pub fn init(&mut self) {
    unsafe{
      let mut rclone: boxed::Box<dyn serialport::SerialPort> = match self.port.try_clone(){
        Err(x) => {
          error!("failed to clone serial port");
          Err(x)
        },
        Ok(x) => Ok(x)
      }.unwrap();

      let mut tcclone = self.news.clone();
      let (txip, rxip): (mpsc::Sender<std::net::Ipv4Addr>, mpsc::Receiver<std::net::Ipv4Addr>) = mpsc::channel();
      
      thread::spawn(move || {
        let mut buffer: Vec<u8> = Vec::new();

        loop {
          let mut pbuff: [u8; 1] = [0; 1];
          match rclone.read(&mut pbuff) {
            Ok(_b) => {
              while BUFLEN > 0 {
                let sub = rxip.recv().unwrap();
                tcclone.addsub(format!("{}.{}.{}.{}", sub.octets()[0], sub.octets()[1], sub.octets()[2], sub.octets()[3]).to_string());
                BUFLEN -= 1;
              }

              if pbuff[0] == 13 {
                SerialHandler::handle_message(&buffer, &tcclone);
                buffer.clear();
              } else {
                buffer.push(pbuff[0]);
              }
            },
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => error!("serial port recive failed {:?}", e)
          };
      }});
      
      let mut tclone: boxed::Box<dyn serialport::SerialPort> = match self.port.try_clone(){
        Err(x) => {
          error!("failed to clone serial port");
          Err(x)
        },
        Ok(x) => Ok(x)
      }.unwrap();
      
      let (tx, rx): (mpsc::Sender<u8>, mpsc::Receiver<u8>) = mpsc::channel();
      self.comm = Some(tx);
      self.commip = Some(txip);
      
      thread::spawn(move || loop{
        let buffer = match rx.recv() {
          Ok(x) => x,
          Err(_) => {
            error!("mpsc pipe failed to recieve");
            return;
          }
        };
        match tclone.write(&[buffer]) {
          Err(x) => info!("could not send serial message {}", x),
          _ => {}
        };
      });  
    }
  }
  
  pub fn send_message(&mut self, buffer: &[u8]) -> bool{
    info!("sending {:?}", buffer);

    for (_i, byte) in buffer.iter().enumerate() {
      match &self.comm {
        Some(tx) => tx.send(*byte).unwrap(),
        None => {error!("serial handler is not initialized"); return false;}
      };
    };

    true
  }

  pub fn handle_message(buffer: &Vec<u8>, tcp: &Newsletter){
    info!("recived {:?}", buffer);
    match tcp.send(buffer){
      Err(x) => error!("tcp failed to send {}", x),
      _ => {}
    };
  }

  pub fn addsub(&mut self, sub: std::net::Ipv4Addr){ 
    unsafe {
      match &self.commip {
        Some(tx) => tx.send(sub).unwrap(),
        None => {error!("serial handler is not initialized");}
      };
      BUFLEN += 1;
    }
  }
}

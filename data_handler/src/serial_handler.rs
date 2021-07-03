use crate::tcptx::Newsletter;
use std::io::Write;
use std::time::Duration;
use std::sync::mpsc;
use std::{io, thread};
use std::{boxed, str};
use std::mem;

use crate::backend::telemetry;

pub const BUFFER_SIZE: usize = 128usize;

pub struct SerialHandler{
  port: boxed::Box<dyn serialport::SerialPort>,
  comm: Option<mpsc::Sender<[u8; BUFFER_SIZE]>>,
  commip: Option<mpsc::Sender<std::net::Ipv4Addr>>,
  news: Newsletter
}

impl SerialHandler {
  pub fn connect(name: &str, baudrate: u32, tcptx: Newsletter) -> Result<Self, String> {
    let mut me = match serialport::new(name, baudrate)
      .timeout(Duration::from_millis(5))
      .open() {
        Ok(x) => Self{ port: x, comm: None, commip: None, news: tcptx },
        Err(_) => return Err(format!("failed to open serial port {}", name))
    };

    me.init()?; Ok(me)
  }

  pub fn init(&mut self) -> Result<(), String> {
    let mut rserial: boxed::Box<dyn serialport::SerialPort> = match self.port.try_clone(){
      Err(x) => {
        Err(format!("failed to clone serial port: {}", x))
      },
      Ok(x) => Ok(x)
    }?;

    let mut news = self.news.clone();
    let (txip, rxip): (mpsc::Sender<std::net::Ipv4Addr>, mpsc::Receiver<std::net::Ipv4Addr>) = mpsc::channel();

    thread::spawn(move || {
      loop {
        let mut buffer = [0u8; BUFFER_SIZE];
        match rserial.read(&mut buffer) {
          Ok(_) => {
            match rxip.try_recv() {
              Ok(x) => {
                news.subscribe(x);
              },
              _ => {}
            };


            info!("pbuffer {:?}", buffer);
            match SerialHandler::handle_message(&buffer, &news) {
              Err(x) => error!("could not handle message: {}", x),
              _ => {}
            };
          },
          Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
          Err(e) => error!("serial port recive failed {:?}", e)
        };
    }});

    let mut serial: boxed::Box<dyn serialport::SerialPort> = match self.port.try_clone(){
      Err(x) => {
        Err(format!("failed to clone serial port: {}", x))
      },
      Ok(x) => Ok(x)
    }?;

    let (tx, rx): (mpsc::Sender<[u8; BUFFER_SIZE]>, mpsc::Receiver<[u8; BUFFER_SIZE]>) = mpsc::channel();
    self.comm = Some(tx);
    self.commip = Some(txip);

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

    Ok(())
  }

  pub fn send_message(&mut self, buffer: [u8; BUFFER_SIZE]) -> Result<(), String>{
    info!("sending {:?}", buffer);

    match &self.comm {
      Some(x) =>  match x.send(buffer) { Ok(_) => Ok(()), Err(x) => Err(format!("thread pipe failed: {}", x)) },
      None => Err(format!("cannot relay messages, was this initialized?"))
    }
  }

  pub fn handle_message(buffer: &[u8; BUFFER_SIZE], tcp: &Newsletter) -> Result<(), String>{
    info!("recived {:?}", buffer);

    let mut processed = format!("");

    if buffer[0] == ('^' as u8) {
        processed.push_str("^;");
        processed.push_str(&str::from_utf8(buffer).unwrap()[1..]);
    } else if buffer[0] == ('B' as u8) {
        let mut raw_data: [u8; 71] = [0; 71];
        raw_data.copy_from_slice(&buffer[0..71]);
        processed.push_str(&unsafe { mem::transmute::<[u8; 71], telemetry::Telemetry>(raw_data) }.serialize()[..]);
    }

    tcp.send(buffer, processed)
  }

  pub fn subscribe(&mut self, sub: std::net::Ipv4Addr) -> Result<(), String> {
    match &self.commip {
      Some(x) => match x.send(sub) { Ok(_) => Ok(()), Err(x) => Err(format!("thread pipe failed: {}", x)) },
      None => Err(format!("serial handler is not initialized"))
    }
  }
}

use std::io::Write;
use std::time::Duration;
use std::sync::mpsc;
use std::{io, thread};
use std::boxed;

pub struct SerialHandler{
  port: boxed::Box<dyn serialport::SerialPort>,
  comm: Option<mpsc::Sender<u8>>
}

impl SerialHandler{
  pub fn connect(name: &str) -> std::result::Result<Self, serialport::Error>{
    match serialport::new(name, 9600)
      .open() {
        Ok(x) => Ok(Self{ port: x, comm: None}),
        Err(x) => {
          error!("failed to open serial port {}", name); Err(x)
        }      
    }
  }

  pub fn init(&mut self) {
    let mut rclone: boxed::Box<dyn serialport::SerialPort> = match self.port.try_clone(){
      Err(x) => {
        error!("failed to clone serial port");
        Err(x)
      },
      Ok(x) => Ok(x)
    }.unwrap();


    thread::spawn(move || loop {
      let mut buffer: [u8; 1] = [0; 1];
      match rclone.read(&mut buffer) {
        Ok(b) => SerialHandler::handle_message(&buffer),
        Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
        Err(e) => error!("serial port recive failed {:?}", e)
      };
    });
    
    let mut tclone: boxed::Box<dyn serialport::SerialPort> = match self.port.try_clone(){
      Err(x) => {
        error!("failed to clone serial port");
        Err(x)
      },
      Ok(x) => Ok(x)
    }.unwrap();
    
    let (tx, rx): (mpsc::Sender<u8>, mpsc::Receiver<u8>) = mpsc::channel();
    self.comm = Some(tx);
    
    thread::spawn(move || loop{
      let buffer = rx.recv().unwrap();
      match tclone.write(&[buffer]) {
        Err(x) => info!("could not send serial message {}", x),
        _ => {}
      };
    });
  }
  
  pub fn send_message(&mut self, buffer: &[u8]) -> bool{
    for (i, byte) in buffer.iter().enumerate() {
      match &self.comm {
        Some(tx) => tx.send(*byte),
        None => {error!("serial handler is not initialized"); return false;}
      };
    };

    true
  }

  pub fn handle_message(buffer: &[u8]){
    info!("recived {}", buffer[0] as char);
  }
}



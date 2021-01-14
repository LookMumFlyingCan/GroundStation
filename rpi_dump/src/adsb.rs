use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::{io, thread};
use std::io::Read;
use std::process::Child;
use stoppable_thread;

pub struct Adsb {
  pub child: mpsc::Receiver<String>,
  pub handle: Option<stoppable_thread::StoppableHandle<()>>,
  pub killer: Child
}

impl Adsb{
  pub fn new(gain: u32, freq: u32) -> Adsb{
    let dump = Adsb::get_thread(gain, freq);

    Adsb { child: dump.2, handle: Some(dump.1), killer: dump.0 }
  }

  pub fn reset(&mut self, gain: u32){
    match self.handle.take() {
      Some(t) => {t.stop(); self.killer.kill().unwrap();},
      None => {return;}
    };
    
    let dump = Adsb::get_thread(gain);
    self.child = dump.2;
    self.handle = Some(dump.1);
    self.killer = dump.0
  }

  fn get_thread(gain: u32, freq: u32) -> (Child, stoppable_thread::StoppableHandle<()>, mpsc::Receiver<String>){
    let (tx, rx): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();
    let (ctx, crx): (mpsc::Sender<Child>, mpsc::Receiver<Child>) = mpsc::channel();

    let child_handle = stoppable_thread::spawn(move |stop| {
      let mut child = Command::new("/home/h39/Downloads/dump1090/dump1090").arg("--raw").arg("--gain").arg(format!("{}", gain)).stdout(Stdio::piped()).spawn().expect("naw man");
      let mut childout = child.stdout.take().unwrap();
      ctx.send(child);
      while !stop.get() {
        let mut buffer = [0; 128];
        childout.read(&mut buffer).unwrap();
        if(buffer[0] == b'*') {
          if(buffer[30] == b';') {
            tx.send(String::from_utf8_lossy(&buffer[1..29]).to_string());
          } else {
            tx.send(String::from_utf8_lossy(&buffer[1..15]).to_string());
          }
        } else if(buffer[0] != 0) {
          info!("unknown message recieved: {}", String::from_utf8_lossy(&buffer));
        }
      }});

    (crx.recv().unwrap(), child_handle, rx)
  }
}

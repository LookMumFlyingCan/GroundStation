use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::io::Read;
use std::result::Result;
use std::process::Child;
use stoppable_thread;
use spmc;

const BUFFER_SIZE: usize = 128;

pub struct Adsb {
  pub child: spmc::Receiver<[u8; BUFFER_SIZE]>,
  handle: Option<stoppable_thread::StoppableHandle<()>>,
  killer: Child
}

impl Adsb{
  pub fn new(path: String, gain: u32, freq: u32) -> Result<Adsb, &'static str> {
    let dump = Adsb::get_thread(path, gain, freq)?;

    Ok(Adsb { child: dump.2, handle: Some(dump.1), killer: dump.0 })
  }

  pub fn reset(&mut self, path: String, gain: u32, freq: u32) -> Result<(), &'static str> {
    match self.handle.take() {
      Some(t) => {t.stop(); 
        match self.killer.kill() {
          Ok(x) => Ok(x),
          Err(_x) => Err("Failed to kill the old worker thread :(")
        }
      },
      None => { Ok(()) }
    }?;
    
    let dump = Adsb::get_thread(path, gain, freq)?;
    self.child = dump.2;
    self.handle = Some(dump.1);
    self.killer = dump.0;

    Ok(())
  }

  fn get_thread(path: String, gain: u32, freq: u32) -> Result<(Child, stoppable_thread::StoppableHandle<()>, spmc::Receiver<[u8; BUFFER_SIZE]>), &'static str> {
    let (mut tx, mut rx): (spmc::Sender<[u8; BUFFER_SIZE]>, spmc::Receiver<[u8; BUFFER_SIZE]>) = spmc::channel();
    let (ctx, crx): (mpsc::Sender<Child>, mpsc::Receiver<Child>) = mpsc::channel();

    let child_handle = stoppable_thread::spawn(move |stop| {
      let mut child = match Command::new(path)
        .arg(format!("-g {}", gain))
        .arg(format!("-f {}", freq))
        .stdout(Stdio::piped()).spawn() {
        Ok(x) => x,
        Err(x) => {error!("Could not start dump1090: {}", x); return; }
      };

      let mut childout = match child.stdout.take() {
        Some(x) => x,
        None => {error!("Was not able to capture the output of dump1090"); return; }
      };

      match ctx.send(child) {
        Ok(x) => x,
        Err(x) => {error!("Failed to pass thread handle over mpsc: {}", x); return; }
      };

      while !stop.get() {
        let mut buffer = [0; BUFFER_SIZE];
        match childout.read(&mut buffer) {
          Ok(x) => x,
          Err(x) => {error!("Cannot read from dump1090: {}", x); return;}
        };
        match tx.send(buffer) {
          Ok(x) => x,
          Err(x) => {error!("Failed to pass dump1090 data over mpsc: {}", x); return; }
        };
    }});

    match crx.recv() {
      Ok(x) => Ok((x, child_handle, rx)),
      Err(x) => { 
        Err(Box::leak(x.to_string().into_boxed_str()))
      }
    }
  }
}

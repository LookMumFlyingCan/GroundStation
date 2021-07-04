use std::io::Write;
use std::time::Duration;
use std::sync::mpsc;
use std::{io, thread};
use std::{boxed, str};

use spmc_buffer::SPMCBuffer;
use spmc_buffer;

pub const BUFFER_SIZE: usize = 128usize;

pub struct SerialHandler{
    pub tx: mpsc::Sender<[u8; BUFFER_SIZE]>,
    pub rx: spmc_buffer::SPMCBufferOutput<[u8; BUFFER_SIZE + 1]>
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

        let (mut stx, srx) = SPMCBuffer::new(10, [0u8; BUFFER_SIZE + 1]).split();

        info!("initializing serial reader");

        let mut counter: u8 = 1;
        thread::spawn(move || {
            loop {
                let mut buffer = [0u8; BUFFER_SIZE + 1];
                match port.read(&mut buffer) {
                    Ok(_x) => {
                        buffer[BUFFER_SIZE] = counter;
                        stx.write(buffer);
                        counter = counter.wrapping_add(1);

                        info!("serial data recieved");
                    },
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                    Err(e) => error!("serial port recive failed {:?}", e)
                };
            }});


        let (tx, rx): (mpsc::Sender<[u8; BUFFER_SIZE]>, mpsc::Receiver<[u8; BUFFER_SIZE]>) = mpsc::channel();

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
}

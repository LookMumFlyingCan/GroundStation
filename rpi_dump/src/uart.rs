use crate::adsb::Adsb;
use stoppable_thread;

pub struct Uart {
    pub decoder: Adsb,
    handle: Option<stoppable_thread::StoppableHandle<()>>
}

impl Uart {
    pub fn new(dec: Adsb) -> Self{
        let crx = dec.child.clone();
        let handle = stoppable_thread::spawn(move |stop| while !stop.get() {
            match crx.recv() {
                Ok(x) => {
                    info!("read: {}", String::from_utf8_lossy(&x));
                },
                _ => {}
            };
        });

        Uart { decoder: dec, handle: Some(handle) }
    }

    pub fn reset(&mut self, path: String, gain: u32, freq: u32) -> Result<(), &'static str> {
        match self.handle.take() {
            Some(t) => {t.stop(); Ok(())},
            None => { Err("Failed to stop worker thread, did you initialize??") }
        }?;

        self.decoder.reset(path, gain, freq)?;

        let crx = self.decoder.child.clone();
        let handle = stoppable_thread::spawn(move |stop| while !stop.get() {
            match crx.recv() {
                Ok(x) => {
                    info!("read: {}", String::from_utf8_lossy(&x));
                },
                _ => {}
            };
        });

        self.handle = Some(handle);

        Ok(())
    }
}
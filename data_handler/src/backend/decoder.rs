use crate::backend::telemetry::Telemetry;
use crate::backend::gps::Gps;
use std::{mem, str};

pub struct Decoder {

}

impl Decoder {
    pub fn decode(data: &[u8]) -> Result<String,String> {
        let mut processed: String = Default::default();
        if data[0] == ('^' as u8) {
            processed.push_str("^;");
            processed.push_str(&str::from_utf8(&data[..]).unwrap()[1..]);
            Ok(processed)
        } else if data[0] == ('B' as u8) {
            let mut raw_data: [u8; 72] = [0; 72];
            raw_data.copy_from_slice(&data[0..72]);
            processed.push_str(&unsafe { mem::transmute::<[u8; 72], Telemetry>(raw_data) }.serialize()[..]);
            Ok(processed)
        } else if data[0] == ('G' as u8) {
            let mut raw_data: [u8; 48] = [0; 48];
            raw_data.copy_from_slice(&data[0..48]);
            processed.push_str(&unsafe { mem::transmute::<[u8; 48], Gps>(raw_data) }.serialize()[..]);
            Ok(processed)
        } else {
            Err(format!("not implemented"))
        }
    }
}

#[repr(packed)]
pub struct Gps {
    pub dformat: u8,
    pub pad0: u8,

    pub frame_num: u16,
    pub pad1: u8,
    pub pad2: u8,
    pub pad3: u8,
    pub pad4: u8,

    pub latitude: f64,
    pub longitude: f64,
    pub speed: f64,
    pub altitude: f64,

    pub time_hour: u8,
    pub pad5: u8,
    pub time_min: u8,
    pub pad6: u8,
    pub time_sec: u8,
    pub pad7: u8,

    pub sat_number: u8,
    pub pad8: u8
}

impl Gps {
    pub fn serialize(&self) -> String {
        unsafe{ [format!("{}", self.dformat as char), ";".to_string(), format!("{}", self.frame_num), ";".to_string(), format!("{}", self.latitude), ";".to_string(), format!("{}", self.longitude), ";".to_string(), format!("{}", self.speed), ";".to_string(), format!("{}", self.altitude), ";".to_string(), format!("{}", self.time_hour), ";".to_string(), format!("{:.2}", self.time_min), ";".to_string(), format!("{}", self.time_sec), ";".to_string(), format!("{}", self.sat_number)].concat().to_string() }
    }
}

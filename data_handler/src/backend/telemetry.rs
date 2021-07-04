//#[repr(packed)]
pub struct Telemetry {
    pub dformat: u8,
    pub frame_num: u16,

    pub temp_pcb: f32,
    pub temp_sdr: f32,
    pub temp_env1: f32,
    pub temp_env2: f32,
    pub temp_rpi: f32,

    pub pressure: f64,

    pub acc_x: f32,
    pub acc_y: f32,
    pub acc_z: f32,

    pub bat_v: f32,
    pub bat_a: f32,
    pub main_bus_voltage: f32,
    pub main_bus_amperage: f32,
    pub low_bus_voltage: f32,
    pub low_bus_amperage: f32,

    pub rpi_status: [u8; 2],
    pub flags: u16
}

impl Telemetry {
    pub fn serialize(&self) -> String {
        [format!("{}", self.dformat as char), ";".to_string(), format!("{}", self.frame_num), ";".to_string(), format!("{}", self.temp_pcb), ";".to_string(), format!("{}", self.temp_sdr), ";".to_string(), format!("{}", self.temp_env1), ";".to_string(), format!("{}", self.temp_env2), ";".to_string(), format!("{}", self.temp_rpi), ";".to_string(), format!("{}", self.pressure), ";".to_string(), format!("{}", self.acc_x), ";".to_string(), format!("{}", self.acc_y), ";".to_string(), format!("{}", self.acc_z), ";".to_string(), format!("{}", self.bat_v), ";".to_string(), format!("{}", self.bat_a), ";".to_string(), format!("{}", self.main_bus_voltage), ";".to_string(), format!("{}", self.main_bus_amperage), ";".to_string(), format!("{}", self.low_bus_voltage), ";".to_string(), format!("{}", self.low_bus_amperage)].concat().to_string()
    }
}

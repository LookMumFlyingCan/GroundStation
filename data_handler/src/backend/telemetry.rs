#[repr(packed)]
pub struct Telemetry {
    pub dformat: u8,
    pub frameNum: u16,

    pub tempPCB: f32,
    pub tempSDR: f32,
    pub tempENV1: f32,
    pub tempENV2: f32,
    pub tempRPI: f32,

    pub pressure: f32,

    pub accX: f32,
    pub accY: f32,
    pub accZ: f32,

    pub batV: f32,
    pub batA: f32,
    pub mainBusVoltage: f32,
    pub mainBusAmperage: f32,
    pub lowBusVoltage: f32,
    pub lowBusAmperage: f32,

    pub rpiStatus: [u8; 2],
    pub flags: [u8; 2]
}

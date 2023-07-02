pub mod blocking;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct OutGauge {
    pub time: u32, // packet time (ms)
    pub car: [u8; 4], // car name
    pub flags: u16, // flags
    pub gear: u8, // Reverse:0, Neutral:1, First:2...
    pub player_id: u8, // Unique ID of viewed player (0 = none)

    pub speed: f32, // (m/s)
    pub rpm: f32, // (rpm)
    pub turbo: f32, // (BAR)
    pub engine_temp: f32, // (°C)
    pub fuel: f32, // 0-1 value
    pub oil_pressure: f32, // (BAR)
    pub oil_temp: f32, // (°C)

    pub available_lights: u32, // dash lights available
    pub dash_lights: u32, // dash lights currently switched on
    pub throttle: f32, // 0-1 value
    pub brake: f32, // 0-1 value
    pub clutch: f32, // 0-1 value
    pub display1: [u8; 16], // usually fuel
    pub display2: [u8; 16], // usually settings
    pub id: i32 // ID
}

impl Default for OutGauge {
    fn default() -> Self {
        unsafe {std::mem::zeroed()}
    }
}
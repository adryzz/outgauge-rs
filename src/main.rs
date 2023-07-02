#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct OutGauge {
    time: u32, // packet time (ms)
    car: [u8; 4], // car name
    flags: u16, // flags
    gear: u8, // Reverse:0, Neutral:1, First:2...
    player_id: u8, // Unique ID of viewed player (0 = none)

    speed: f32, // (m/s)
    rpm: f32, // (rpm)
    turbo: f32, // (BAR)
    engine_temp: f32, // (°C)
    fuel: f32, // 0-1 value
    oil_pressure: f32, // (BAR)
    oil_temp: f32, // (°C)

    available_lights: u32, // dash lights available
    dash_lights: u32, // dash lights currently switched on
    throttle: f32, // 0-1 value
    brake: f32, // 0-1 value
    clutch: f32, // 0-1 value
    display1: [u8; 16], // usually fuel
    display2: [u8; 16], // usually settings
    id: i32 // ID
}
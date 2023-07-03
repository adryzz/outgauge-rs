#![cfg_attr(not(feature = "std"), no_std)]

use ascii::AsciiChar;
use bitflags::bitflags;

#[cfg(feature = "blocking")]
pub mod blocking;

#[cfg(feature = "tokio")]
pub mod nonblocking;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct OutGauge {
    pub time: u32,                    // packet time (ms)
    pub car: [AsciiChar; 4],          // car name
    pub flags: Flags,                 // flags
    pub gear: u8,                     // Reverse:0, Neutral:1, First:2...
    pub player_id: u8,                // Unique ID of viewed player (0 = none)
    pub speed: f32,                   // (m/s)
    pub rpm: f32,                     // (rpm)
    pub turbo: f32,                   // (BAR)
    pub engine_temp: f32,             // (°C)
    pub fuel: f32,                    // 0-1 value
    pub oil_pressure: f32,            // (BAR)
    pub oil_temp: f32,                // (°C)
    pub available_lights: DashLights, // dash lights available
    pub dash_lights: DashLights,      // dash lights currently switched on
    pub throttle: f32,                // 0-1 value
    pub brake: f32,                   // 0-1 value
    pub clutch: f32,                  // 0-1 value
    pub display1: [AsciiChar; 16],    // usually fuel
    pub display2: [AsciiChar; 16],    // usually settings
    pub id: i32,                      // ID
}

impl Default for OutGauge {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct Flags: u16 {
        const SHIFT = 1;
        const CTRL  = 2;
        const TURBO = 8192; // show turbo gauge
        const KM    = 16384; // if not set - user prefers MILES
        const BAR   = 32768; // if not set - user prefers PSI
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct DashLights: u32 {
        const SHIFT       = 0b000000000001; // shift light
        const FULLBEAM    = 0b000000000010; // full beam
        const HANDBRAKE   = 0b000000000100; // handbrake
        const PITSPEED    = 0b000000001000; // pit speed limiter
        const TC          = 0b000000010000; // TC active or switched off
        const SIGNAL_L    = 0b000000100000; // left turn signal
        const SIGNAL_R    = 0b000001000000; // right turn signal
        const SIGNAL_ANY  = 0b000010000000; // shared turn signal
        const OILWARN     = 0b000100000000; // oil pressure warning
        const BATTERY     = 0b001000000000; // battery warning
        const ABS         = 0b010000000000; // ABS active or switched off
        const SPARE       = 0b100000000000; // N/A
    }
}

use convert_macro::{FieldsPos, ToVec};
use rinex::navigation::Ephemeris;

/// All SBAS navigation data
#[derive(Debug, Clone, PartialEq, FieldsPos, ToVec, Default)]
pub struct SBASNavData {
    pub clock_bias: f64,
    pub clock_drift: f64,
    // time of message
    pub tom: f64,
    pub x: f64,
    pub vel_x: f64,
    pub accel_x: f64,
    pub health: f64,
    pub y: f64,
    pub vel_y: f64,
    pub accel_y: f64,
    pub ura: f64,
    pub z: f64,
    pub vel_z: f64,
    pub accel_z: f64,
    // issue of data navigation
    pub iodn: f64,
}

impl From<&Ephemeris> for SBASNavData {
    fn from(value: &Ephemeris) -> Self {
        Self {
            clock_bias: value.clock_bias,
            clock_drift: value.clock_drift,
            tom: value.get_orbit_f64("week").unwrap_or(0.0),
            x: value.get_orbit_f64("satPosX").unwrap_or(0.0),
            vel_x: value.get_orbit_f64("velX").unwrap_or(0.0),
            accel_x: value.get_orbit_f64("accelX").unwrap_or(0.0),
            health: value.get_orbit_f64("health").unwrap_or(0.0),
            y: value.get_orbit_f64("satPosY").unwrap_or(0.0),
            vel_y: value.get_orbit_f64("velY").unwrap_or(0.0),
            accel_y: value.get_orbit_f64("accelY").unwrap_or(0.0),
            ura: value.get_orbit_f64("accuracyCode").unwrap_or(0.0),
            z: value.get_orbit_f64("satPosZ").unwrap_or(0.0),
            vel_z: value.get_orbit_f64("velZ").unwrap_or(0.0),
            accel_z: value.get_orbit_f64("accelZ").unwrap_or(0.0),
            iodn: value.get_orbit_f64("iodn").unwrap_or(0.0),
        }
    }
}

use convert_macro::{FieldsPos, ToVec};
use rinex::navigation::Ephemeris;

/// Glonass navigation data
#[derive(Debug, Clone, PartialEq, FieldsPos, ToVec, Default)]
pub struct GlonassNavData {
    pub clock_bias: f64,
    pub clock_drift: f64,
    // message frame time
    pub mrt: f64,
    pub x: f64,
    pub vel_x: f64,
    pub accel_x: f64,
    pub health: f64,
    pub y: f64,
    pub vel_y: f64,
    pub accel_y: f64,
    pub z: f64,
    pub vel_z: f64,
    pub accel_z: f64,
    pub age: f64,
}

impl From<&Ephemeris> for GlonassNavData {
    /// Converts a `Ephemeris` to a `GlonassNavData`.
    fn from(value: &Ephemeris) -> Self {
        Self {
            clock_bias: value.clock_bias,
            clock_drift: value.clock_drift,
            mrt: value.get_orbit_f64("mrt").unwrap_or(0.0),
            x: value.get_orbit_f64("satPosX").unwrap_or(0.0),
            vel_x: value.get_orbit_f64("velX").unwrap_or(0.0),
            accel_x: value.get_orbit_f64("accelX").unwrap_or(0.0),
            health: value.get_orbit_f64("health").unwrap_or(0.0),
            y: value.get_orbit_f64("satPosY").unwrap_or(0.0),
            vel_y: value.get_orbit_f64("velY").unwrap_or(0.0),
            accel_y: value.get_orbit_f64("accelY").unwrap_or(0.0),
            z: value.get_orbit_f64("satPosZ").unwrap_or(0.0),
            vel_z: value.get_orbit_f64("velZ").unwrap_or(0.0),
            accel_z: value.get_orbit_f64("accelZ").unwrap_or(0.0),
            age: value.get_orbit_f64("age").unwrap_or(0.0),
        }
    }
}

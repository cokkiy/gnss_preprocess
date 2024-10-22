use convert_macro::{FieldsPos, ToVec};
use rinex::navigation::Ephemeris;

/// All SBAS navigation data
#[derive(Debug, Clone, PartialEq, FieldsPos, ToVec)]
pub struct SBASNavData {
    clock_bias: f64,
    clock_drift: f64,
    // time of message
    tom: f64,
    x: f64,
    vel_x: f64,
    accel_x: f64,
    health: f64,
    y: f64,
    vel_y: f64,
    accel_y: f64,
    ura: f64,
    z: f64,
    vel_z: f64,
    accel_z: f64,
    // issue of data navigation
    iodn: f64,
}

impl From<&Ephemeris> for SBASNavData {
    fn from(value: &Ephemeris) -> Self {
        Self {
            clock_bias: value.clock_bias,
            clock_drift: value.clock_drift,
            tom: value.get_orbit_f64("tom").unwrap_or(0.0),
            x: value.get_orbit_f64("x").unwrap_or(0.0),
            vel_x: value.get_orbit_f64("velX").unwrap_or(0.0),
            accel_x: value.get_orbit_f64("accelX").unwrap_or(0.0),
            health: value.get_orbit_f64("health").unwrap_or(0.0),
            y: value.get_orbit_f64("y").unwrap_or(0.0),
            vel_y: value.get_orbit_f64("velY").unwrap_or(0.0),
            accel_y: value.get_orbit_f64("accelY").unwrap_or(0.0),
            ura: value.get_orbit_f64("ura").unwrap_or(0.0),
            z: value.get_orbit_f64("z").unwrap_or(0.0),
            vel_z: value.get_orbit_f64("velZ").unwrap_or(0.0),
            accel_z: value.get_orbit_f64("accelZ").unwrap_or(0.0),
            iodn: value.get_orbit_f64("iodn").unwrap_or(0.0),
        }
    }
}

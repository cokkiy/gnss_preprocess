use convert_macro::{FieldsPos, ToVec};
use rinex::navigation::Ephemeris;

#[derive(Debug, Clone, PartialEq, FieldsPos, ToVec)]
pub struct BeiDouNavData {
    clock_bias: f64,
    clock_drift: f64,
    // age of data
    aode: f64,
    crs: f64,
    delta_n: f64,
    m0: f64,
    cuc: f64,
    e: f64,
    cus: f64,
    sqrt_a: f64,
    toe: f64,
    cic: f64,
    omega_0: f64,
    cis: f64,
    i0: f64,
    crc: f64,
    omega: f64,
    omega_dot: f64,
}

impl From<&Ephemeris> for BeiDouNavData {
    /// Converts a `Ephemeris` to a `BeiDouNavData`.
    fn from(value: &Ephemeris) -> Self {
        Self {
            clock_bias: value.clock_bias,
            clock_drift: value.clock_drift,
            aode: value.get_orbit_f64("aode").unwrap_or(0.0),
            crs: value.get_orbit_f64("crs").unwrap_or(0.0),
            delta_n: value.get_orbit_f64("deltaN").unwrap_or(0.0),
            m0: value.get_orbit_f64("m0").unwrap_or(0.0),
            cuc: value.get_orbit_f64("cuc").unwrap_or(0.0),
            e: value.get_orbit_f64("e").unwrap_or(0.0),
            cus: value.get_orbit_f64("cus").unwrap_or(0.0),
            sqrt_a: value.get_orbit_f64("sqrta").unwrap_or(0.0),
            toe: value.get_orbit_f64("toe").unwrap_or(0.0),
            cic: value.get_orbit_f64("cic").unwrap_or(0.0),
            omega_0: value.get_orbit_f64("omega0").unwrap_or(0.0),
            cis: value.get_orbit_f64("cis").unwrap_or(0.0),
            i0: value.get_orbit_f64("i0").unwrap_or(0.0),
            crc: value.get_orbit_f64("crc").unwrap_or(0.0),
            omega: value.get_orbit_f64("omega").unwrap_or(0.0),
            omega_dot: value.get_orbit_f64("omegaDot").unwrap_or(0.0),
        }
    }
}

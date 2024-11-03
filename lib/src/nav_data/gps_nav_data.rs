use convert_macro::{FieldsPos, ToVec};
use rinex::navigation::Ephemeris;

/// GPS 导航电文主要信息
#[derive(Debug, Clone, PartialEq, FieldsPos, ToVec, Default)]
pub struct GPSNavData {
    /// The sv clock bias
    pub clock_bias: f64,
    pub clock_drift: f64,
    /// 卫星星历数据的数据龄期
    pub iode: f64,
    pub crs: f64,
    pub delta_n: f64,
    pub m0: f64,
    pub cuc: f64,
    pub e: f64,
    pub cus: f64,
    pub sqrt_a: f64,
    pub toe: f64,
    pub cic: f64,
    pub omega_0: f64,
    pub cis: f64,
    pub i0: f64,
    pub crc: f64,
    pub omega: f64,
    pub omega_dot: f64,
    pub i_dot: f64,
}

impl From<&Ephemeris> for GPSNavData {
    fn from(value: &Ephemeris) -> Self {
        Self {
            clock_bias: value.clock_bias,
            clock_drift: value.clock_drift,
            iode: value.get_orbit_f64("iode").unwrap_or(0.0),
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
            i_dot: value.get_orbit_f64("idot").unwrap_or(0.0),
        }
    }
}

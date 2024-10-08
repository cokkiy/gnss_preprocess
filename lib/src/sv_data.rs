use gnss_rs::sv::SV;
use rinex::prelude::Constellation;

use crate::GnssData;
/// A struct that represents the SV data.
///
/// The SV data is a tuple that contains the SV prn and the GNSS data.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct SVData(u8, GnssData);

#[allow(dead_code)]
impl SVData {
    /// Get the satellite vehicle information from prn and the GNSS data type.
    pub fn get_sv(&self) -> SV {
        match self.1 {
            GnssData::GPSData(_) => SV::new(Constellation::GPS, self.0),
            GnssData::GlonassData(_) => SV::new(Constellation::Glonass, self.0),
            GnssData::SBASData(_) => SV::new(Constellation::SBAS, self.0),
            GnssData::QZSSData(_) => SV::new(Constellation::QZSS, self.0),
            GnssData::GalileoData(_) => SV::new(Constellation::Galileo, self.0),
            GnssData::BeidouData(_) => SV::new(Constellation::BeiDou, self.0),
            GnssData::IRNSSData(_) => SV::new(Constellation::IRNSS, self.0),
        }
    }

    /// Retrieves the GNSS data reference.
    pub fn get_data(&self) -> &GnssData {
        &self.1
    }
}

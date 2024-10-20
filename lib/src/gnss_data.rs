use std::collections::HashMap;

use fields_count::AllFieldsCount;
use rinex::{
    observation::ObservationData,
    prelude::{Constellation, Observable},
};
use ssc::SignalStrengthComparer;

use crate::{
    beidou_data::BeidouData, galileo_data::GalileoData, glonass_data::GlonassData,
    gps_data::GPSData, irnss_data::IRNSSData, qzss_data::QZSSData, sbas_data::SBASData,
};

/// Gnss data structure
#[derive(Clone, Debug)]
pub enum GnssData {
    /// GPS data
    GPSData(GPSData),
    /// Glonass data
    GlonassData(GlonassData),
    /// Galileo data
    GalileoData(GalileoData),
    /// SBAS data
    SBASData(SBASData),
    /// QZSS data
    QZSSData(QZSSData),
    /// BeiDou data
    BeidouData(BeidouData),
    /// IRNSS data
    IRNSSData(IRNSSData),
}

impl GnssData {
    /// Get the maximum length of all GNSS constellation type data.
    pub fn max_len() -> usize {
        let gps_len = GPSData::get_fields_count();
        let galileo_len = GalileoData::get_fields_count();
        let glonass_len = GlonassData::get_fields_count();
        let beidou_len = BeidouData::get_fields_count();
        let qzss_len = QZSSData::get_fields_count();
        let sbas_len = SBASData::get_fields_count();
        let irnss_len = IRNSSData::get_fields_count();

        gps_len
            .max(galileo_len)
            .max(glonass_len)
            .max(beidou_len)
            .max(qzss_len)
            .max(sbas_len)
            .max(irnss_len)
    }

    /// Create GNSS data from the given data.
    /// # Arguments
    /// * `constellation` - The GNSS constellation type.
    /// * `data` - The observation data.
    /// # Returns
    /// The GNSS data.
    pub fn create(
        constellation: &Constellation,
        data: &HashMap<Observable, ObservationData>,
    ) -> Self {
        match constellation {
            Constellation::GPS => GnssData::GPSData(GPSData::from(data)),
            Constellation::Glonass => GnssData::GlonassData(GlonassData::from(data)),
            Constellation::Galileo => GnssData::GalileoData(GalileoData::from(data)),
            Constellation::QZSS => GnssData::QZSSData(QZSSData::from(data)),
            Constellation::BeiDou => GnssData::BeidouData(BeidouData::from(data)),
            Constellation::IRNSS => GnssData::IRNSSData(IRNSSData::from(data)),
            _ => GnssData::SBASData(SBASData::from(data)),
        }
    }
}

impl From<&GnssData> for Vec<f64> {
    /// Convert GnssData to Vec<f64>.
    /// The length of the vector is the maximum length of all GNSS data,
    /// The missing data is filled with 0.0.
    fn from(value: &GnssData) -> Self {
        let len = GnssData::max_len();
        let mut data: Vec<f64> = match value {
            GnssData::GPSData(data) => data.into(),
            GnssData::GlonassData(data) => data.into(),
            GnssData::GalileoData(data) => data.into(),
            GnssData::SBASData(data) => data.into(),
            GnssData::QZSSData(data) => data.into(),
            GnssData::BeidouData(data) => data.into(),
            GnssData::IRNSSData(data) => data.into(),
        };
        let mut tail = vec![0.0; len - data.len()];
        data.append(&mut tail);
        data
    }
}

impl From<GPSData> for GnssData {
    /// Convert GPSData to GnssData
    fn from(value: GPSData) -> Self {
        GnssData::GPSData(value)
    }
}

impl From<GlonassData> for GnssData {
    /// Convert GlonassData to GnssData
    fn from(value: GlonassData) -> Self {
        GnssData::GlonassData(value)
    }
}

impl From<GalileoData> for GnssData {
    /// Convert GalileoData to GnssData
    fn from(value: GalileoData) -> Self {
        GnssData::GalileoData(value)
    }
}

impl From<SBASData> for GnssData {
    /// Convert SBASData to GnssData
    fn from(value: SBASData) -> Self {
        GnssData::SBASData(value)
    }
}

impl From<QZSSData> for GnssData {
    /// Convert QZSSData to GnssData
    fn from(value: QZSSData) -> Self {
        GnssData::QZSSData(value)
    }
}

impl From<BeidouData> for GnssData {
    /// Convert BeidouData to GnssData
    fn from(value: BeidouData) -> Self {
        GnssData::BeidouData(value)
    }
}

impl From<IRNSSData> for GnssData {
    /// Convert IRNSSData to GnssData
    fn from(value: IRNSSData) -> Self {
        GnssData::IRNSSData(value)
    }
}

impl SignalStrengthComparer for GnssData {
    fn ss_compare(&self, other: &Self) -> Vec<f64> {
        match self {
            GnssData::GPSData(data) => {
                if let GnssData::GPSData(other_data) = other {
                    data.ss_compare(other_data)
                } else {
                    vec![]
                }
            }
            GnssData::GlonassData(data) => {
                if let GnssData::GlonassData(other_data) = other {
                    data.ss_compare(other_data)
                } else {
                    vec![]
                }
            }
            GnssData::GalileoData(data) => {
                if let GnssData::GalileoData(other_data) = other {
                    data.ss_compare(other_data)
                } else {
                    vec![]
                }
            }
            GnssData::SBASData(data) => {
                if let GnssData::SBASData(other_data) = other {
                    data.ss_compare(other_data)
                } else {
                    vec![]
                }
            }
            GnssData::QZSSData(data) => {
                if let GnssData::QZSSData(other_data) = other {
                    data.ss_compare(other_data)
                } else {
                    vec![]
                }
            }
            GnssData::BeidouData(data) => {
                if let GnssData::BeidouData(other_data) = other {
                    data.ss_compare(other_data)
                } else {
                    vec![]
                }
            }
            GnssData::IRNSSData(data) => {
                if let GnssData::IRNSSData(other_data) = other {
                    data.ss_compare(other_data)
                } else {
                    vec![]
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use rinex::observation::LliFlags;

    use super::*;

    #[test]
    fn test_create_gps_data() {
        let mut data = HashMap::new();
        data.insert(
            Observable::PseudoRange("c1c".to_string()),
            ObservationData::new(
                1.0,
                Some(LliFlags::OK_OR_UNKNOWN),
                Some(rinex::observation::SNR::DbHz0),
            ),
        );
        data.insert(
            Observable::Phase("l1c".to_string()),
            ObservationData::new(
                2.0,
                Some(LliFlags::OK_OR_UNKNOWN),
                Some(rinex::observation::SNR::DbHz0),
            ),
        );
        let gnss_data = GnssData::create(&Constellation::GPS, &data);
        if let GnssData::GPSData(_) = gnss_data {
            // Test passed
        } else {
            panic!("Expected GnssData::GPSData");
        }
    }

    #[test]
    fn test_create_glonass_data() {
        let mut data = HashMap::new();
        data.insert(
            Observable::PseudoRange("c1c".to_string()),
            ObservationData::new(
                1.0,
                Some(LliFlags::OK_OR_UNKNOWN),
                Some(rinex::observation::SNR::DbHz0),
            ),
        );
        data.insert(
            Observable::Phase("l1c".to_string()),
            ObservationData::new(
                2.0,
                Some(LliFlags::OK_OR_UNKNOWN),
                Some(rinex::observation::SNR::DbHz0),
            ),
        );
        let gnss_data = GnssData::create(&Constellation::Glonass, &data);
        if let GnssData::GlonassData(_) = gnss_data {
            // Test passed
        } else {
            panic!("Expected GnssData::GlonassData");
        }
    }

    #[test]
    fn test_create_galileo_data() {
        let mut data = HashMap::new();
        data.insert(
            Observable::PseudoRange("c1c".to_string()),
            ObservationData::new(
                1.0,
                Some(LliFlags::OK_OR_UNKNOWN),
                Some(rinex::observation::SNR::DbHz0),
            ),
        );
        data.insert(
            Observable::Phase("l1c".to_string()),
            ObservationData::new(
                2.0,
                Some(LliFlags::OK_OR_UNKNOWN),
                Some(rinex::observation::SNR::DbHz0),
            ),
        );
        let gnss_data = GnssData::create(&Constellation::Galileo, &data);
        if let GnssData::GalileoData(_) = gnss_data {
            // Test passed
        } else {
            panic!("Expected GnssData::GalileoData");
        }
    }

    #[test]
    fn test_create_sbas_data() {
        let mut data = HashMap::new();
        data.insert(
            Observable::PseudoRange("c1c".to_string()),
            ObservationData::new(
                1.0,
                Some(LliFlags::OK_OR_UNKNOWN),
                Some(rinex::observation::SNR::DbHz0),
            ),
        );
        data.insert(
            Observable::Phase("l1c".to_string()),
            ObservationData::new(
                2.0,
                Some(LliFlags::OK_OR_UNKNOWN),
                Some(rinex::observation::SNR::DbHz0),
            ),
        );
        let gnss_data = GnssData::create(&Constellation::SBAS, &data);
        if let GnssData::SBASData(_) = gnss_data {
            // Test passed
        } else {
            panic!("Expected GnssData::SBASData");
        }
    }

    #[test]
    fn test_create_qzss_data() {
        let mut data = HashMap::new();
        data.insert(
            Observable::PseudoRange("c1c".to_string()),
            ObservationData::new(
                1.0,
                Some(LliFlags::OK_OR_UNKNOWN),
                Some(rinex::observation::SNR::DbHz0),
            ),
        );
        data.insert(
            Observable::Phase("l1c".to_string()),
            ObservationData::new(
                2.0,
                Some(LliFlags::OK_OR_UNKNOWN),
                Some(rinex::observation::SNR::DbHz0),
            ),
        );
        let gnss_data = GnssData::create(&Constellation::QZSS, &data);
        if let GnssData::QZSSData(_) = gnss_data {
            // Test passed
        } else {
            panic!("Expected GnssData::QZSSData");
        }
    }

    #[test]
    fn test_create_beidou_data() {
        let mut data = HashMap::new();
        data.insert(
            Observable::PseudoRange("c1c".to_string()),
            ObservationData::new(
                1.0,
                Some(LliFlags::OK_OR_UNKNOWN),
                Some(rinex::observation::SNR::DbHz0),
            ),
        );
        data.insert(
            Observable::Phase("l1c".to_string()),
            ObservationData::new(
                2.0,
                Some(LliFlags::OK_OR_UNKNOWN),
                Some(rinex::observation::SNR::DbHz0),
            ),
        );
        let gnss_data = GnssData::create(&Constellation::BeiDou, &data);
        if let GnssData::BeidouData(_) = gnss_data {
            // Test passed
        } else {
            panic!("Expected GnssData::BeidouData");
        }
    }

    #[test]
    fn test_create_irnss_data() {
        let mut data = HashMap::new();
        data.insert(
            Observable::PseudoRange("c1c".to_string()),
            ObservationData::new(
                1.0,
                Some(LliFlags::OK_OR_UNKNOWN),
                Some(rinex::observation::SNR::DbHz0),
            ),
        );
        data.insert(
            Observable::Phase("l1c".to_string()),
            ObservationData::new(
                2.0,
                Some(LliFlags::OK_OR_UNKNOWN),
                Some(rinex::observation::SNR::DbHz0),
            ),
        );
        let gnss_data = GnssData::create(&Constellation::IRNSS, &data);
        if let GnssData::IRNSSData(_) = gnss_data {
            // Test passed
        } else {
            panic!("Expected GnssData::IRNSSData");
        }
    }

    #[test]
    fn test_gnss_data_to_vec() {
        let gps_data = GPSData::default(); // Assuming GPSData has a default implementation
        let gnss_data = GnssData::GPSData(gps_data);
        let vec: Vec<f64> = (&gnss_data).into();
        assert_eq!(vec.len(), GnssData::max_len());
    }
}

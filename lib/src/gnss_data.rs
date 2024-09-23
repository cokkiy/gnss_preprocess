use std::collections::HashMap;

use rinex::{observation::ObservationData, prelude::Observable};

use crate::{
    beidou_data::BeidouData, galileo_data::GalileoData, glonass_data::GlonassData,
    gps_data::GPSData, irnss_data::IRNSSData, qzss_data::QZSSData, sbas_data::SBASData,
};

/// Gnss data structure
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
        let gps_len = GPSData::fields_pos().len();
        let galileo_len = GalileoData::fields_pos().len();
        let glonass_len = GlonassData::fields_pos().len();
        let beidou_len = BeidouData::fields_pos().len();
        let qzss_len = QZSSData::fields_pos().len();
        let sbas_len = SBASData::fields_pos().len();
        let irnss_len = IRNSSData::fields_pos().len();

        gps_len
            .max(galileo_len)
            .max(glonass_len)
            .max(beidou_len)
            .max(qzss_len)
            .max(sbas_len)
            .max(irnss_len)
    }

    /// Create GnssData from HashMap<Observable, ObservationData>.
    ///
    /// # Arguments
    ///
    /// * `data` - HashMap<Observable, ObservationData>
    ///
    /// # Example
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use rinex::{observation::{ObservationData, LliFlags}, prelude::Observable};
    /// use gnss_preprocess::{GnssData, GPSData};
    ///
    /// let mut data = HashMap::new();
    /// data.insert(
    ///    Observable::PseudoRange("c1c".to_string()),
    ///    ObservationData::new(
    ///         1.0,
    ///         Some(LliFlags::OK_OR_UNKNOWN),
    ///         Some(rinex::observation::SNR::DbHz0),
    ///         ),
    ///     );
    /// data.insert(
    ///     Observable::Phase("l1c".to_string()),
    ///     ObservationData::new(
    ///         2.0,
    ///         Some(LliFlags::OK_OR_UNKNOWN),
    ///         Some(rinex::observation::SNR::DbHz0),
    ///         ),
    ///     );
    /// let gnss_data = GnssData::create::<GPSData>(&data);
    /// ```
    pub fn create<'a, T>(data: &'a HashMap<Observable, ObservationData>) -> Self
    where
        T: From<&'a HashMap<Observable, ObservationData>> + Into<GnssData>,
    {
        T::from(data).into()
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
        let gnss_data = GnssData::create::<GPSData>(&data);
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
        let gnss_data = GnssData::create::<GlonassData>(&data);
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
        let gnss_data = GnssData::create::<GalileoData>(&data);
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
        let gnss_data = GnssData::create::<SBASData>(&data);
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
        let gnss_data = GnssData::create::<QZSSData>(&data);
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
        let gnss_data = GnssData::create::<BeidouData>(&data);
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
        let gnss_data = GnssData::create::<IRNSSData>(&data);
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

use crate::{
    glonass_data::GlonassData, BeidouData, GPSData, GalileoData, IRNSSData, QZSSData, SBASData,
    SVData,
};
use core::f64;
use fields_count::SignalStrengthFieldsCount;
use hifitime::{Duration, Epoch};
use rinex::prelude::GroundPosition;
use ssc::SignalStrengthComparer;

/// A struct that represents the station coordinates.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub struct Station(f64, f64, f64);

impl From<(f64, f64, f64)> for Station {
    /// Converts from a tuple to a `Station` instance.
    fn from(data: (f64, f64, f64)) -> Self {
        Self(data.0, data.1, data.2)
    }
}

impl From<GroundPosition> for Station {
    /// Converts from a `GroundPosition` instance to a `Station` instance.
    fn from(data: GroundPosition) -> Self {
        data.to_ecef_wgs84().into()
    }
}

/// A struct that represents the GNSS epoch data.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct GnssEpochData {
    /// The epoch of the GNSS data.
    epoch: Epoch,
    /// The GNSS data in the epoch.
    data: Vec<SVData>,
    /// The station coordinates.
    station: Station,
}

#[allow(dead_code)]
impl GnssEpochData {
    /// The maximum number of signal strength fields in all types of GNSS data.
    pub fn max_ss_fields_number() -> usize {
        let gps_len = GPSData::get_ss_fields_count();
        let galileo_len = GalileoData::get_ss_fields_count();
        let glonass_len = GlonassData::get_ss_fields_count();
        let beidou_len = BeidouData::get_ss_fields_count();
        let qzss_len = QZSSData::get_ss_fields_count();
        let sbas_len = SBASData::get_ss_fields_count();
        let irnss_len = IRNSSData::get_ss_fields_count();

        gps_len
            .max(galileo_len)
            .max(glonass_len)
            .max(beidou_len)
            .max(qzss_len)
            .max(sbas_len)
            .max(irnss_len)
    }

    /// Creates a new `GnssEpochData` instance.
    ///
    /// # Arguments
    ///
    /// * `epoch` - The epoch of the GNSS data.
    /// * `station` - The station coordinates.
    /// * `data` - The GNSS data in the epoch.
    ///
    /// # Returns
    ///
    /// A new `GnssEpochData` instance.
    pub fn new(epoch: Epoch, station: Station, data: Vec<SVData>) -> Self {
        Self {
            epoch,
            data,
            station,
        }
    }

    /// Retrieves the epoch of the GNSS data.
    pub fn get_epoch(&self) -> Epoch {
        self.epoch
    }

    /// Retrieves the SV data in the epoch.
    ///
    /// # Returns
    ///
    /// The SV data in the epoch.
    pub fn get_data(&self) -> &Vec<SVData> {
        self.data.as_ref()
    }

    /// Retrieves the station coordinates.
    /// # Returns
    /// The station coordinates.
    pub fn get_station(&self) -> Station {
        self.station
    }

    /// Retrieves the time gap between the current epoch and the other epoch.
    pub fn time_gap(&self, other: &GnssEpochData) -> Duration {
        self.epoch - other.epoch
    }

    /// Iterates over the SV data in the epoch.
    /// # Returns
    /// An iterator over the SV data in the epoch.
    /// # Note
    /// This method returns a reference to the SV data in the epoch.
    pub fn iter(&self) -> impl Iterator<Item = &SVData> + '_ {
        self.data.iter()
    }

    pub fn signal_strength_compare(&self, other: &GnssEpochData) -> Vec<Vec<f64>> {
        let mut result = Vec::new();
        for data in self.iter() {
            let sv_data = data.get_data();
            let sv = data.get_sv();
            let sv_data_other = other
                .iter()
                .find(|d| d.get_sv() == sv)
                .map(|d| d.get_data());
            if let Some(sv_data_other) = sv_data_other {
                let mut c_result = sv_data.ss_compare(sv_data_other);
                let max_len = GnssEpochData::max_ss_fields_number();
                c_result.extend_from_slice(&vec![0.0; max_len - c_result.len()]);
                result.push(c_result);
            } else {
                result.push(vec![f64::MAX; GnssEpochData::max_ss_fields_number()]);
            }
        }
        result
    }
}

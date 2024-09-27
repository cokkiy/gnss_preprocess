use gnss_rs::sv::SV;
use hifitime::Epoch;
use ssc::SignalStrengthComparer;

use crate::GnssData;

/// A struct that represents the GNSS epoch data.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct GnssEpochData {
    /// The epoch of the GNSS data.
    pub epoch: Epoch,
    /// The satellite vehicle (SV).
    pub sv: SV,
    /// The GNSS data in the epoch.
    pub data: GnssData,
}

#[allow(dead_code)]
impl GnssEpochData {
    /// Creates a new `GnssEpochData` instance.
    ///
    /// # Arguments
    ///
    /// * `epoch` - The epoch of the GNSS data.
    /// * `sv` - The satellite vehicle (SV).
    /// * `data` - The GNSS data in the epoch.
    ///
    /// # Returns
    ///
    /// A new `GnssEpochData` instance.
    pub fn new(epoch: Epoch, sv: SV, data: GnssData) -> Self {
        Self { epoch, sv, data }
    }

    /// Retrieves the epoch of the GNSS data.
    pub fn get_epoch(&self) -> Epoch {
        self.epoch
    }

    /// Retrieves the satellite vehicle (SV).
    pub fn get_sv(&self) -> SV {
        self.sv
    }

    /// Retrieves the GNSS data in the epoch.
    ///
    /// # Returns
    ///
    /// The GNSS data in the epoch.
    ///
    /// # Note
    ///
    /// This method returns a clone of the GNSS data in the epoch.
    pub fn get_data(&self) -> GnssData {
        self.data.clone()
    }
}

impl SignalStrengthComparer for GnssEpochData {
    fn ss_compare(&self, other: &Self) -> Vec<f64> {
        self.data.ss_compare(&other.data)
    }
}

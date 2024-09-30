use hifitime::{Duration, Epoch};

use crate::GnssData;

/// A struct that represents the GNSS epoch data.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct GnssEpochData {
    /// The epoch of the GNSS data.
    pub epoch: Epoch,
    /// The GNSS data in the epoch.
    pub data: Vec<GnssData>,
}

#[allow(dead_code)]
impl GnssEpochData {
    /// Creates a new `GnssEpochData` instance.
    ///
    /// # Arguments
    ///
    /// * `epoch` - The epoch of the GNSS data.
    /// * `data` - The GNSS data in the epoch.
    ///
    /// # Returns
    ///
    /// A new `GnssEpochData` instance.
    pub fn new(epoch: Epoch, data: Vec<GnssData>) -> Self {
        Self { epoch, data }
    }

    /// Retrieves the epoch of the GNSS data.
    pub fn get_epoch(&self) -> Epoch {
        self.epoch
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
    pub fn get_data(&self) -> Vec<GnssData> {
        self.data.clone()
    }

    /// Retrieves the time gap between the current epoch and the other epoch.
    pub fn time_gap(&self, other: &GnssEpochData) -> Duration {
        self.epoch - other.epoch
    }

    /// Iterates over the GNSS data in the epoch.
    /// # Returns
    /// An iterator over the GNSS data in the epoch.
    /// # Note
    /// This method returns a reference to the GNSS data in the epoch.
    pub fn next(&self) -> impl Iterator<Item = &GnssData> + '_ {
        self.data.iter()
    }
}

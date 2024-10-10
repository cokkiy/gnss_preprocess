use std::path::PathBuf;

use rinex::Rinex;

use crate::gnss_epoch_data::{GnssEpochData, Station};

#[allow(dead_code)]
pub struct StationEpochProvider {
    station_name: String,
    station: Station,
    year: u16,
    day_of_year: u16,
    obs_file: Rinex,
}

#[allow(dead_code)]
impl StationEpochProvider {
    /// Creates a new `StationEpochProvider` instance.
    ///
    /// # Arguments
    ///
    /// * `station_name` - The name of the station.
    /// * `station` - The station coordinates.
    /// * `init_year` - The initial year. In this year the station starts to observe the gnss data.
    /// * `init_day_of_year` - The initial day of the year. In this day the station starts to observe the gnss data.
    /// * `obs_files_tree` - The observation files tree.
    ///
    /// # Returns
    ///
    /// A new `StationEpochProvider` instance.
    pub(super) fn create(
        base_path: &str,
        station_name: &str,
        year: u16,
        day_of_year: u16,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let obs_file = PathBuf::from(base_path)
            .join(year.to_string())
            .join("daily")
            .join(format!("{:03}.obs", day_of_year));
        let obs_file = Rinex::from_file(&obs_file.to_string_lossy())?;

        Ok(Self {
            station_name: station_name.to_string(),
            station: obs_file.header.ground_position.unwrap_or_default().into(),
            year,
            day_of_year,
            obs_file,
        })
    }

    pub fn next_epoch(&self) -> impl Iterator<Item = &GnssEpochData> + '_ {
        // TODO: need complete implementation
        // Example implementation returning an empty iterator
        std::iter::empty()
    }
}

use rinex::Rinex;

use crate::{
    gnss_epoch_data::{GnssEpochData, Station},
    obs_files_tree::ObsFilesTree,
};

#[allow(dead_code)]
pub struct StationEpochProvider<'a> {
    station_name: String,
    station: Station,
    current_year: u16,
    current_day_of_year: u16,
    obs_files_tree: &'a ObsFilesTree,
    cur_obs_file: Rinex,
}

#[allow(dead_code)]
impl<'a> StationEpochProvider<'a> {
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
        station_name: &str,
        station: Station,
        init_year: u16,
        init_day_of_year: u16,
        obs_files_tree: &'a ObsFilesTree,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let obs_file = obs_files_tree
            .find_file(init_year, init_day_of_year, station_name)
            .unwrap(); // when calling this function, the file is guaranteed to exist

        Ok(Self {
            station_name: station_name.to_string(),
            station,
            current_year: init_year,
            current_day_of_year: init_day_of_year,
            obs_files_tree,
            cur_obs_file: Rinex::from_file(&obs_file.to_string_lossy())?,
        })
    }

    pub fn next_epoch(&self) -> impl Iterator<Item = &GnssEpochData> + '_ {
        // TODO: need complete implementation
        // Example implementation returning an empty iterator
        std::iter::empty()
    }
}

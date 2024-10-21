use crate::{
    obs_files_tree::ObsFilesTree, station_alive::StationAlive,
    station_epoch_provider::StationEpochProvider,
};
/// StationsManager is a struct that will manage the all gnss stations information.
///
/// It will be responsible for:
/// - Scan all obs files (We really load data from `ObsFileTree` instead of scan file by ourself.)
/// and load all stations name and it's observation time (year and day_of_year).
/// - Create a `StationAlive` represents the station alive days for each station.
/// - Provide a method `get_all_stations` for retrieves all stations name.
/// - Provide a method `get_station_epoch_provider` for retrieves the `StationEpochProvider` instance
/// for the specified station.
#[allow(dead_code)]
pub struct StationsManager {
    stations_alive: Vec<StationAlive>,
}

#[allow(dead_code)]
impl StationsManager {
    /// Creates a new `StationsManager` instance from the `ObsFilesTree`.
    /// # Arguments
    /// * `tree` - The ObsFilesTree instance.
    /// # Returns
    /// A new `StationsManager` instance.
    /// # Note
    /// Iterates over the `ObsFilesTree` and creates a `StationAlive` instance for each station.
    pub fn new(tree: &ObsFilesTree) -> Self {
        let mut stations_alive: Vec<StationAlive> = vec![];
        tree.iter().for_each(|(y, d, name)| {
            if let Some(station) = stations_alive
                .iter_mut()
                .find(|s| s.get_station_name() == name)
            {
                station.add_alive_day(y, d);
            } else {
                let mut station = StationAlive::new(name);
                station.add_alive_day(y, d);
                stations_alive.push(station);
            }
        });
        Self { stations_alive }
    }

    /// Retrieves all stations name.
    pub fn get_all_stations(&self) -> Vec<String> {
        self.stations_alive
            .iter()
            .map(|s| s.get_station_name().to_string())
            .collect()
    }

    pub fn get_station_epoch_provider<'a>(
        &'a self,
        base_path: &'a str,
        station_name: &str,
    ) -> StationEpochProvider {
        let station = self
            .stations_alive
            .iter()
            .find(|s| s.get_station_name() == station_name)
            .unwrap();
        StationEpochProvider::new(base_path, station)
    }
}

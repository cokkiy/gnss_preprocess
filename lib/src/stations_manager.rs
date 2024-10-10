use crate::obs_files_tree::ObsFilesTree;

/// StationAlive is a struct that will store the station name and the station alive days.
#[allow(dead_code)]
struct StationAlive {
    station_name: String,
    alive_days: Vec<(u16, u16)>,
}

#[allow(dead_code)]
impl StationAlive {
    /// Creates a new `StationAlive` instance.
    /// # Arguments
    /// * `station_name` - The name of the station.
    /// # Returns
    /// A new `StationAlive` instance.
    fn new(station_name: String) -> Self {
        Self {
            station_name,
            alive_days: vec![],
        }
    }

    /// Add a new alive day to the station.
    /// # Arguments
    /// * `year` - The year of the alive day.
    /// * `day_of_year` - The day of the year of the alive day.
    /// # Returns
    /// A new `StationAlive` instance.
    /// # Note
    /// If the alive day is already in the station, it will not be added.
    fn add_alive_day(&mut self, year: u16, day_of_year: u16) {
        if self
            .alive_days
            .iter()
            .find(|(y, d)| y == &year && d == &day_of_year)
            .is_none()
        {
            self.alive_days.push((year, day_of_year));
        }
    }
}

/// StationsManager is a struct that will manage the all gnss stations information.
///
/// It will be responsible for:
/// - Scan all obs files (We really load data from `ObsFileTree` instead of scan file by ourself.)
/// and load all stations name and observation time (year and day_of_year).
/// - Create a StationEpochProvider for each station at the station works day.
///  For a period of continuous observation days, only create a StationEpochProvider.
/// - Provide a method to get the next epoch data for all stations.
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
            if let Some(station) = stations_alive.iter_mut().find(|s| s.station_name == name) {
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
            .map(|s| s.station_name.clone())
            .collect()
    }
}
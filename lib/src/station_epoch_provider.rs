use crate::{
    gnss_epoch_data::GnssEpochData, single_file_epoch_provider::SingleFileEpochProvider,
    station_alive::StationAlive,
};
/// StationEpochProvider is a struct that will provide the GNSS epoch data received
/// by the specified station in epoch by epoch mode.
/// It will be responsible for:
/// - Iterating over the `StationAlive` instance and create a `SingleFileEpochProvider` instance
/// for each station alive day.
/// - Iterating over the `SingleFileEpochProvider` instance and retrieve `GnssEpochData` for each epoch.
/// - Continuously provide the GNSS data in the epoch by epoch mode even the receiver lost some data.
/// - NOT ASSURED the returned epoch is just next to the previous one.
/// - Iterating retrieves the `GnssEpochData` instance for each epoch from the station alive days and no
/// gap between alive days.
/// # Note
/// The `StationEpochProvider` instance will provide the GNSS data in the epoch by epoch mode and
/// NOT ASSURED the returned epoch is just next to the previous one. The user should use the `time_gap`
/// method to calculate the time gap between the epochs.
///
#[allow(dead_code)]
pub struct StationEpochProvider<'a> {
    base_path: &'a str,
    station_alive: &'a StationAlive,
}

#[allow(dead_code)]
impl<'a> StationEpochProvider<'a> {
    /// Creates a new `StationEpochProvider` instance.
    /// # Arguments
    /// * `base_path` - The base path of the observation files.
    /// * `station_alive` - The station alive info.
    /// # Returns
    /// A new `StationEpochProvider` instance.
    pub(crate) fn new(base_path: &'a str, station_alive: &'a StationAlive) -> Self {
        Self {
            base_path,
            station_alive,
        }
    }

    /// Retrieves the next epoch Gnss Data from the station.
    /// # Returns
    /// An iterator over the GNSS data in the epoch batch.
    /// # Note
    /// The returned epoch data NOT ASSURED is just next to the previous one if the
    /// receive station lost some data in receiving. It's the user's responsibility to
    /// calculate the time gap between the epochs. This method just assures the returned
    /// epoch is later than the previous one and no more epochs between there.
    pub fn next_epoch(&self) -> impl Iterator<Item = GnssEpochData> + '_ {
        self.station_alive
            .next_alive_day()
            .flat_map(|(year, day_of_year)| {
                let single_file_epoch_provider = SingleFileEpochProvider::new(
                    self.station_alive.get_station_name(),
                    self.base_path,
                    *year,
                    *day_of_year,
                );
                single_file_epoch_provider
                    .into_iter()
                    .map(|epoch_data| epoch_data)
            })
    }
}

#[cfg(test)]
mod tests {
    use hifitime::Epoch;

    use super::*;
    #[test]
    fn test_next_epoch() {
        let mut station_alive = StationAlive::new("abmf".to_string());
        station_alive.add_alive_day(2020, 1);
        station_alive.add_alive_day(2020, 2);
        station_alive.add_alive_day(2021, 266);

        let base_path = "D:\\Data\\Obs";
        let provider = StationEpochProvider::new(base_path, &station_alive);

        let epochs: Vec<GnssEpochData> = provider.next_epoch().collect();
        assert!(!epochs.is_empty());
        assert_eq!(epochs.len(), 2880 * 3);
    }

    #[test]
    fn test_next_epoch_iter() {
        let mut station_alive = StationAlive::new("abmf".to_string());
        station_alive.add_alive_day(2020, 1);
        station_alive.add_alive_day(2020, 2);
        station_alive.add_alive_day(2021, 266);

        let base_path = "D:\\Data\\Obs";
        let provider = StationEpochProvider::new(base_path, &station_alive);

        let first_epochs = provider.next_epoch().next().unwrap();
        assert_eq!(
            first_epochs.get_epoch(),
            Epoch::from_gregorian(2020, 1, 1, 0, 0, 0, 0, hifitime::TimeScale::GPST)
        );

        let the_2881th_epochs = provider.next_epoch().nth(2880).unwrap();
        assert_eq!(
            the_2881th_epochs.get_epoch(),
            Epoch::from_gregorian(2020, 1, 2, 0, 0, 0, 0, hifitime::TimeScale::GPST)
        );

        let the_5761th_epochs = provider.next_epoch().nth(2880 * 2).unwrap();
        assert_eq!(
            the_5761th_epochs.get_epoch(),
            Epoch::from_gregorian(2021, 9, 23, 0, 0, 0, 0, hifitime::TimeScale::GPST)
        );
    }
}

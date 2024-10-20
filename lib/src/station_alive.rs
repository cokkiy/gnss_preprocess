/// StationAlive is a struct that will store the station name and the station alive days.
/// The station alive days are stored as a tuple of year and day of the year.
#[allow(dead_code)]
pub(super) struct StationAlive {
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
    pub(crate) fn new(station_name: String) -> Self {
        Self {
            station_name,
            alive_days: vec![],
        }
    }

    /// Retrieves the station name.
    pub(crate) fn get_station_name(&self) -> &str {
        &self.station_name
    }

    /// Add a new alive day to the station.
    /// # Arguments
    /// * `year` - The year of the alive day.
    /// * `day_of_year` - The day of the year of the alive day.
    /// # Returns
    /// A new `StationAlive` instance.
    /// # Note
    /// If the alive day is already in the station, it will not be added.
    pub(crate) fn add_alive_day(&mut self, year: u16, day_of_year: u16) {
        if self
            .alive_days
            .iter()
            .find(|(y, d)| y == &year && d == &day_of_year)
            .is_none()
        {
            self.alive_days.push((year, day_of_year));
        }
    }

    /// Retrieves the next alive day.
    /// # Returns
    /// An iterator over the alive days.
    /// # Note
    /// The iterator will return a tuple of year and day of the year.
    pub(crate) fn next_alive_day(&self) -> impl Iterator<Item = &(u16, u16)> {
        self.alive_days.iter()
    }
}

use crate::{
    gnss_epoch_data::{GnssEpochData, Station},
    GnssData, SVData,
};
use log::error;
use rinex::{prelude::EpochFlag, Rinex};
use std::{cell::Cell, path::PathBuf};

/// A struct that provides the epoch from a single obs file.
pub(crate) struct SingleFileEpochProvider {
    cur_index: Cell<usize>,
    rinex: Result<Rinex, rinex::Error>,
}

impl SingleFileEpochProvider {
    /// Creates a new `SingleFileEpochProvider` instance.
    /// # Arguments
    /// * `station_name` - The name of the station.
    /// * `base_path` - The base path of the observation files.
    /// * `year` - The year of the observation file.
    /// * `day_of_year` - The day of year of the observation file.
    /// # Returns
    /// A new `SingleFileEpochProvider` instance.
    pub(crate) fn new(station_name: &str, base_path: &str, year: u16, day_of_year: u16) -> Self {
        let path = PathBuf::from(base_path)
            .join(format!("{}", year))
            .join(format!("{:03}", day_of_year))
            .join("daily")
            .join(format!(
                "{}{:03}0.{}o",
                station_name,
                day_of_year,
                year % 2000
            ));
        let rinex = Rinex::from_file(path.to_str().unwrap_or_default());
        if rinex.is_err() {
            error!("Error reading file: {:?}", path);
        }
        Self {
            cur_index: Cell::new(0),
            rinex,
        }
    }

    /// Retrieves the sample rate of the obs file.
    pub(crate) fn get_sample_rate(&self) -> Option<hifitime::Duration> {
        if let Ok(rinex) = &self.rinex {
            rinex.sample_rate()
        } else {
            None
        }
    }

    /// Retrieves the next epoch from the obs file.
    /// # Returns
    /// The next epoch data.
    /// # Note
    /// If there are no more epochs, it will return None.
    ///
    /// This method IS NOT assured the returned epoch is just next to the previous one.
    /// For example, if the current epoch is not OK, it will skip the current epoch and return the next one.
    pub(crate) fn next_epoch(&self) -> Option<GnssEpochData> {
        if let Ok(rinex) = &self.rinex {
            let station: Station = rinex.header.ground_position.into();
            let mut flag = EpochFlag::PowerFailure;
            let mut result = None;
            while !flag.is_ok() {
                if let Some(((epoch, epoch_flag), (_, vehicles))) =
                    rinex.observation().nth(self.cur_index.get())
                {
                    self.cur_index.set(self.cur_index.get() + 1);
                    flag = *epoch_flag;
                    if flag.is_ok() {
                        let mut epoch_sv_data = Vec::new();
                        for (sv, data) in vehicles {
                            let gnss_data = GnssData::create(&sv.constellation, data);
                            let sv_data = SVData::new(sv.prn, gnss_data);
                            epoch_sv_data.push(sv_data);
                        }
                        result = Some(GnssEpochData::new(epoch.clone(), station, epoch_sv_data));
                    }
                } else {
                    result = None;
                    break;
                }
            }
            result
        } else {
            None
        }
    }
}

impl Iterator for SingleFileEpochProvider {
    type Item = GnssEpochData;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_epoch()
    }
}

#[cfg(test)]
mod tests {
    use hifitime::Epoch;

    use super::*;
    #[test]
    fn test_next_epoch() {
        let provider = SingleFileEpochProvider::new("abmf", "D:\\Data\\Obs", 2020, 1);
        let epoch = provider.next_epoch();
        assert!(epoch.is_some());

        assert_eq!(
            epoch.unwrap().get_epoch(),
            Epoch::from_gregorian(2020, 1, 1, 0, 0, 0, 0, hifitime::TimeScale::GPST)
        );
    }

    #[test]
    fn test_iter() {
        let provider = SingleFileEpochProvider::new("abmf", "D:\\Data\\Obs", 2020, 1);
        let epochs: Vec<GnssEpochData> = provider.collect();
        assert!(!epochs.is_empty());
        assert_eq!(epochs.len(), 2880);
        assert_eq!(
            epochs.last().unwrap().get_epoch(),
            Epoch::from_gregorian(2020, 1, 1, 23, 59, 30, 0, hifitime::TimeScale::GPST)
        );
    }
}

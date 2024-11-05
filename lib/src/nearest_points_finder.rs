use std::cell::RefCell;

use crate::nav_data::NavData;
use hifitime::{Duration, Epoch};
use rinex::{prelude::SV, Rinex};

/// Nearest point finder trait
pub(crate) trait NearestPointsFinder {
    /// Find nearest points to the given epoch
    /// # Arguments
    /// * `epoch` - The epoch to find the nearest points
    /// # Returns
    /// * A vector of `NavData` that contains the nearest points to the given epoch
    /// # Note
    /// The vector of `NavData` should be sorted by the distance to the given epoch.
    fn find_nearest_points(&self, sv: &SV, epoch: &Epoch) -> Vec<NavData>;
}

/// TreePointsFinder is a NearestPointsFinder that finds three nearest points.
pub(crate) struct TreePointsFinder {
    base_path: String,
    year_and_days: Vec<(u16, u16)>,
    cached_rinex: RefCell<Vec<(u16, u16, Option<Rinex>)>>,
}

impl TreePointsFinder {
    /// Create a new TreePointsFinder
    /// # Arguments
    /// * `base_path` - The base path to the RINEX nav files.
    pub(crate) fn new(base_path: String) -> Self {
        Self {
            year_and_days: Self::get_all_doy(&base_path),
            base_path,
            // initialize the cached rinex with 4 elements
            cached_rinex: RefCell::new(Vec::with_capacity(4)),
        }
    }
    //read all files in the base path and get year and doy information
    fn get_all_doy(base_path: &str) -> Vec<(u16, u16)> {
        let mut year_and_days = Vec::new();
        if let Ok(root_dir) = std::fs::read_dir(base_path) {
            root_dir
                .filter_map(|year_entries| year_entries.ok())
                .for_each({
                    |year_entries| {
                        let year = year_entries.file_name().to_string_lossy().parse::<u16>();
                        if let Ok(year) = year {
                            if let Ok(doy_entries) = std::fs::read_dir(year_entries.path()) {
                                doy_entries
                                    .filter_map(|doy_entry| doy_entry.ok())
                                    .for_each({
                                        |doy_entry| {
                                            let doy = doy_entry.file_name().to_string_lossy()[4..7]
                                                .parse::<u16>();
                                            if let Ok(doy) = doy {
                                                year_and_days.push((year, doy));
                                            }
                                        }
                                    });
                            }
                        }
                    }
                });
        }
        year_and_days
    }

    fn get_rinex_index(&self, epoch: &Epoch) -> usize {
        let year = epoch.year() as u16;
        let doy = epoch.day_of_year().floor() as u16;
        // find in the cached rinex
        for (i, cached) in self.cached_rinex.borrow().iter().enumerate() {
            if cached.0 == year && cached.1 == doy {
                return i;
            }
        }
        let mut found_rinex = None;
        // not found in the cached, we need to find it
        for (y, d) in &self.year_and_days {
            if *y == year && *d == doy {
                let _rinex = Rinex::from_file(&format!(
                    "{}/{}/brdm{:03}0.{}p",
                    self.base_path,
                    year,
                    doy,
                    year % 2000
                ));
                if _rinex.as_ref().is_ok_and(|f| f.is_navigation_rinex()) {
                    found_rinex = Some(_rinex.unwrap());
                }
                break;
            }
        }

        if self.cached_rinex.borrow().len() == 4 {
            // remove the first element
            self.cached_rinex.borrow_mut().remove(0);
        }
        self.cached_rinex
            .borrow_mut()
            .push((year, doy, found_rinex));

        self.cached_rinex.borrow().len() - 1
    }

    fn get_last_epoch_nav_data(&self, cache_index: usize, epoch: &Epoch, sv: &SV) -> NavData {
        if let Some(rinex) = self
            .cached_rinex
            .borrow()
            .get(cache_index)
            .unwrap()
            .2
            .as_ref()
        {
            let last_epoch_frames = rinex
                .navigation()
                .filter(|(_, nvf)| {
                    nvf.iter()
                        .any(|f| f.as_eph().is_some_and(|(_, this_sv, _)| this_sv == *sv))
                })
                .last();
            if let Some((e, fs)) = last_epoch_frames {
                let frame = fs
                    .iter()
                    .find(|f| f.as_eph().is_some_and(|(_, this_sv, _)| this_sv == *sv))
                    .unwrap(); // safe to unwrap
                return NavData::from_rinex_frame(e, sv, frame.as_eph().unwrap().2);
            }
        }
        return NavData::create_default(epoch, sv);
    }

    fn get_first_epoch_nav_data(&self, cache_index: usize, epoch: &Epoch, sv: &SV) -> NavData {
        if let Some(rinex) = self
            .cached_rinex
            .borrow()
            .get(cache_index)
            .unwrap()
            .2
            .as_ref()
        {
            let first_epoch_frames = rinex
                .navigation()
                .filter(|(_, nvf)| {
                    nvf.iter()
                        .any(|f| f.as_eph().is_some_and(|(_, this_sv, _)| this_sv == *sv))
                })
                .next();
            if let Some((e, fs)) = first_epoch_frames {
                let frame = fs
                    .iter()
                    .find(|f| f.as_eph().is_some_and(|(_, this_sv, _)| this_sv == *sv))
                    .unwrap(); // safe to unwrap
                return NavData::from_rinex_frame(e, sv, frame.as_eph().unwrap().2);
            }
        }
        return NavData::create_default(epoch, sv);
    }
}

impl NearestPointsFinder for TreePointsFinder {
    fn find_nearest_points(&self, sv: &SV, epoch: &Epoch) -> Vec<NavData> {
        let i = self.get_rinex_index(epoch);
        if let Some(rinex) = self.cached_rinex.borrow().get(i).unwrap().2.as_ref() {
            let epoch_frames = rinex
                .navigation()
                .filter(|(_, nvf)| {
                    nvf.iter()
                        .any(|f| f.as_eph().is_some_and(|(_, this_sv, _)| this_sv == *sv))
                })
                .min_by(|(&e1, _), (&e2, _)| (e1 - *epoch).abs().cmp(&(e2 - *epoch).abs()));
            if let Some((&epoch, frames)) = epoch_frames {
                let frame = frames
                    .iter()
                    .find(|f| f.as_eph().is_some_and(|(_, this_sv, _)| this_sv == *sv))
                    .unwrap(); // safe to unwrap
                let current = NavData::from_rinex_frame(&epoch, sv, frame.as_eph().unwrap().2);

                // index in filter epoch_frames
                if epoch
                    == rinex
                        .navigation()
                        .filter(|(_, nvf)| {
                            nvf.iter()
                                .any(|f| f.as_eph().is_some_and(|(_, this_sv, _)| this_sv == *sv))
                        })
                        .next()
                        .unwrap()
                        .0
                        .clone()
                {
                    // first frame
                    let prev_epoch = epoch - Duration::from_days(1.0);
                    let prev_rinex_index = self.get_rinex_index(&prev_epoch);
                    let prev_nav_data = self.get_last_epoch_nav_data(prev_rinex_index, &epoch, sv);
                } else if epoch
                    >= rinex
                        .navigation()
                        .filter(|(_, nvf)| {
                            nvf.iter()
                                .any(|f| f.as_eph().is_some_and(|(_, this_sv, _)| this_sv == *sv))
                        })
                        .last()
                        .unwrap()
                        .0
                        .clone()
                {
                    // last frame
                    let next_epoch = epoch + Duration::from_days(1.0);
                    let next_rinex_index = self.get_rinex_index(&next_epoch);
                    let next_nav_data = self.get_first_epoch_nav_data(next_rinex_index, &epoch, sv);
                } else {
                    // middle frame
                }
            }
        }
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_get_all_doy() {
        let base_path = "d:/data/test_nav";
        let expected = vec![
            (2020, 1),
            (2020, 2),
            (2020, 3),
            (2020, 5),
            (2020, 6),
            (2020, 366),
            (2021, 1),
            (2021, 2),
            (2021, 3),
            (2021, 4),
        ];
        let result = TreePointsFinder::get_all_doy(base_path);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_rinex_initial() {
        let finder = TreePointsFinder::new("d:/data/test_nav".to_string());
        let epoch = Epoch::from_gregorian_utc(2020, 1, 1, 0, 0, 0, 0);
        let rinex_index = finder.get_rinex_index(&epoch);
        assert_eq!(rinex_index, 0);
        assert!(finder.cached_rinex.borrow().get(0).is_some());
    }

    #[test]
    fn test_get_rinex_next_day() {
        let finder = TreePointsFinder::new("d:/data/test_nav".to_string());
        let epoch = Epoch::from_gregorian_utc(2020, 1, 1, 0, 0, 0, 0);
        let index = finder.get_rinex_index(&epoch);
        assert_eq!(0, index);
        let next_epoch = Epoch::from_gregorian_utc(2020, 1, 2, 0, 0, 0, 0);
        let index = finder.get_rinex_index(&next_epoch);
        assert_eq!(1, index);
        assert!(finder.cached_rinex.borrow().get(1).is_some());
        let binding = finder.cached_rinex.borrow();
        let r = binding.get(1).unwrap();
        assert_eq!(r.0, 2020);
        assert_eq!(r.1, 2);
        assert!(r.2.is_some());
    }

    #[test]
    fn test_get_rinex_previous_day() {
        let finder = TreePointsFinder::new("d:/data/test_nav".to_string());
        let epoch = Epoch::from_gregorian_utc(2020, 1, 2, 0, 0, 0, 0);
        let index = finder.get_rinex_index(&epoch);
        assert_eq!(0, index);
        let prev_epoch = Epoch::from_gregorian_utc(2020, 1, 1, 0, 0, 0, 0);
        let index = finder.get_rinex_index(&prev_epoch);
        assert_eq!(1, index);
    }

    #[test]
    fn test_get_rinex_cur_day_not_found() {
        let finder = TreePointsFinder::new("d:/data/test_nav".to_string());
        let epoch = Epoch::from_gregorian_utc(2020, 1, 4, 0, 0, 0, 0);
        let index = finder.get_rinex_index(&epoch);
        assert_eq!(0, index);
    }

    #[test]
    fn test_get_rinex_next_day_not_found() {
        let finder = TreePointsFinder::new("d:/data/test_nav".to_string());
        let epoch = Epoch::from_gregorian_utc(2020, 1, 3, 0, 0, 0, 0);
        let index = finder.get_rinex_index(&epoch);
        assert_eq!(0, index);
        let epoch = Epoch::from_gregorian_utc(2020, 1, 4, 0, 0, 0, 0);
        let index = finder.get_rinex_index(&epoch);
        assert_eq!(1, index);
    }

    #[test]
    fn test_find_nearest_points() {
        let finder = TreePointsFinder::new("test_data".to_string());
        let sv = SV::from_str("G01").unwrap();
        let epoch = Epoch::from_gregorian_utc(2023, 1, 1, 0, 0, 0, 0);
        let points = finder.find_nearest_points(&sv, &epoch);
        assert!(points.is_empty());
    }
}

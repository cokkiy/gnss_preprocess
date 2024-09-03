use std::{collections::HashMap, path::PathBuf};

use rinex::prelude::{Constellation, Epoch, SV};

use crate::{
    constellation_keys::CONSTELLATION_KEYS,
    navdata_interpolation::{NavDataInterpolation, SampleResult},
    navigation_data::{
        combine_navigation_data, get_current_day_last_epoch, get_navigation_data,
        get_next_day_first_epoch, NavigationData,
    },
};

/// The `NavDataProvider` struct provides navigation data.
/// It reads navigation data from the navigation files path and provides interpolation for the navigation data foy any
/// valid time.
#[derive(Debug, Clone)]
pub struct NavDataProvider {
    nav_file_path: PathBuf,
    /// The current year.
    current_year: u16,
    /// The current day of the year.
    current_day: u16,

    /// The current day navigation data.
    current_day_nav_data: Option<NavigationData>,
    /// The next day navigation data.
    next_day_nav_data: Option<NavigationData>,
    /// The current single day interpolation.
    single_interpolation: Option<NavDataInterpolation>,
    /// The current cross day (current and next day) interpolation.
    cross_interpolation: Option<NavDataInterpolation>,
}

#[allow(dead_code)]
impl NavDataProvider {
    /// Creates a new instance of `NavDataProvider`.
    ///
    /// # Arguments
    ///
    /// * `nav_files_path` - The path to the navigation files.
    ///
    /// # Returns
    ///
    /// A new instance of `NavDataProvider`.
    pub fn new(nav_files_path: &str) -> Self {
        Self {
            nav_file_path: PathBuf::from(nav_files_path),
            current_year: 0,
            current_day: 0,
            single_interpolation: None,
            cross_interpolation: None,
            current_day_nav_data: None,
            next_day_nav_data: None,
        }
    }

    /// Performs a sample on the navigation data provider.
    ///
    /// # Arguments
    ///
    /// * `year` - The year of the sample.
    /// * `day_of_year` - The day of the year of the sample.
    /// * `sv` - The satellite vehicle to sample.
    /// * `epoch` - The epoch to sample.
    ///
    /// # Returns
    ///
    /// An optional `Vec<f64>` containing the sample results, where the values are floats.
    /// Returns `None` if the sample results contain any errors or if the navigation data provider does not have the required data.
    pub fn sample(
        &mut self,
        year: u16,
        day_of_year: u16,
        sv: &SV,
        epoch: &Epoch,
    ) -> Option<Vec<f64>> {
        let mut year = year;
        if year > 1000 {
            year -= 2000;
        }

        if self.current_year != year || self.current_day != day_of_year {
            // if not current day, update the navigation data
            self.update_data(year, day_of_year);
        }
        if let Some(interpolation) = self.single_interpolation.as_ref() {
            let sample_results = interpolation.samples(sv, epoch);
            if sample_results.iter().any(|(_, r)| r.as_ref().is_err()) {
                None
            } else if sample_results.iter().all(|(_, r)| match r.as_ref() {
                Ok(result) => result.is_valid(),
                Err(_) => false,
            }) {
                convert_results(sv, &sample_results)
            } else {
                let results = if let Some(cross_interpolation) = self.cross_interpolation.as_ref() {
                    cross_interpolation.samples(sv, epoch)
                } else {
                    sample_results.clone()
                };
                if results.iter().any(|(_, r)| r.is_err()) {
                    convert_results(sv, &sample_results)
                } else {
                    convert_results(sv, &results)
                }
            }
        } else {
            None
        }
    }

    /// Updates the navigation data based on the given year and day of year.
    fn update_data(&mut self, year: u16, day_of_year: u16) {
        // check if the day is current day's next day
        let next_day = get_next_day(self.current_year, self.current_day);
        if year == next_day.0 && day_of_year == next_day.1 {
            // if is next day, update the current day and next day navigation data
            self.current_year = year;
            self.current_day = day_of_year;
            self.current_day_nav_data = self.next_day_nav_data.take();
            self.single_interpolation = Some(NavDataInterpolation::new(
                self.current_day_nav_data.as_ref().unwrap(),
            ));
            // then load the next day data
            self.load_next_day_data();
        } else {
            // not the next day, update the current day navigation data
            self.current_year = year;
            self.current_day = day_of_year;
            let nav_file = self
                .nav_file_path
                .join(format!("20{}/brdm{:03}0.{:02}p", year, day_of_year, year));
            if let Ok(navigation_data) = get_navigation_data(nav_file.to_str().unwrap()) {
                self.current_day_nav_data = Some(navigation_data);
                let nav_data_interpolation =
                    NavDataInterpolation::new(self.current_day_nav_data.as_ref().unwrap());
                self.single_interpolation = Some(nav_data_interpolation);
            } else {
                self.single_interpolation = None;
            }

            self.load_next_day_data();
        }
    }

    fn load_next_day_data(&mut self) {
        // get the next day
        let next_day = get_next_day(self.current_year, self.current_day);
        // load next day navigation data
        let next_nav_file = self.nav_file_path.join(format!(
            "20{}/brdm{:03}0.{:02}p",
            next_day.0, next_day.1, next_day.0
        ));
        if let Ok(navigation_data) = get_navigation_data(next_nav_file.to_str().unwrap()) {
            self.next_day_nav_data = Some(navigation_data);
            let first_epoch = get_next_day_first_epoch(self.next_day_nav_data.as_ref().unwrap());
            let last_epoch =
                get_current_day_last_epoch(self.current_day_nav_data.as_ref().unwrap());

            let combined_data = combine_navigation_data(&last_epoch, &first_epoch);
            self.cross_interpolation = Some(NavDataInterpolation::new(&combined_data));
        } else {
            self.next_day_nav_data = None;
            self.cross_interpolation = None;
        }
    }
}

fn convert_results(
    sv: &SV,
    sample_results: &HashMap<String, Result<SampleResult, String>>,
) -> Option<Vec<f64>> {
    let mut results = vec![0.0; 20];
    sample_results.iter().for_each(|(field, r)| {
        let index = match sv.constellation {
            Constellation::GPS => CONSTELLATION_KEYS
                .get(&Constellation::GPS)
                .unwrap()
                .iter()
                .position(|k| k == field)
                .unwrap(),
            Constellation::Glonass => CONSTELLATION_KEYS
                .get(&Constellation::Glonass)
                .unwrap()
                .iter()
                .position(|k| k == field)
                .unwrap(),
            Constellation::Galileo => CONSTELLATION_KEYS
                .get(&Constellation::Galileo)
                .unwrap()
                .iter()
                .position(|k| k == field)
                .unwrap(),
            Constellation::BeiDou => CONSTELLATION_KEYS
                .get(&Constellation::BeiDou)
                .unwrap()
                .iter()
                .position(|k| k == field)
                .unwrap(),
            Constellation::IRNSS => CONSTELLATION_KEYS
                .get(&Constellation::IRNSS)
                .unwrap()
                .iter()
                .position(|k| k == field)
                .unwrap(),
            Constellation::QZSS => CONSTELLATION_KEYS
                .get(&Constellation::QZSS)
                .unwrap()
                .iter()
                .position(|k| k == field)
                .unwrap(),
            _ => CONSTELLATION_KEYS
                .get(&Constellation::SBAS)
                .unwrap()
                .iter()
                .position(|k| k == field)
                .unwrap(),
        };
        results[index] = r.as_ref().unwrap().value();
    });

    Some(results)
}

fn get_next_day(year: u16, day_of_year: u16) -> (u16, u16) {
    if is_leap_year(year) {
        if day_of_year == 366 {
            return (year + 1, 1);
        }
    } else if day_of_year == 365 {
        return (year + 1, 1);
    }
    (year, day_of_year + 1)
}

/// Determines if a given year is a leap year. If the year is two digital,
/// it is converted to a four digital year by add 2000.
fn is_leap_year(year: u16) -> bool {
    let mut year = year;
    if year < 100 {
        year += 2000;
    }
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use rinex::prelude::{Constellation, TimeScale};
    use rstest::rstest;

    #[test]
    fn test_is_leap_year_with_leap_year() {
        let year = 2020;
        assert!(is_leap_year(year));
    }

    #[test]
    fn test_is_leap_year_with_non_leap_year() {
        let year = 2021;
        assert!(!is_leap_year(year));
    }

    #[test]
    fn test_is_leap_year_with_two_digital_leap_year() {
        let year = 20;
        assert!(is_leap_year(year));
    }

    #[test]
    fn test_is_leap_year_with_two_digit_non_leap_year() {
        let year = 21;
        assert!(!is_leap_year(year));
    }

    #[test]
    fn test_get_next_day_with_leap_year() {
        let year = 2020;
        let day_of_year = 365;
        let (next_year, next_day) = get_next_day(year, day_of_year);
        assert_eq!(next_year, 2020);
        assert_eq!(next_day, 366);
    }

    #[test]
    fn test_get_next_day_with_non_leap_year() {
        let year = 2021;
        let day_of_year = 364;
        let (next_year, next_day) = get_next_day(year, day_of_year);
        assert_eq!(next_year, 2021);
        assert_eq!(next_day, 365);
    }

    #[test]
    fn test_get_next_day_with_leap_year_last_day() {
        let year = 2020;
        let day_of_year = 366;
        let (next_year, next_day) = get_next_day(year, day_of_year);
        assert_eq!(next_year, 2021);
        assert_eq!(next_day, 1);
    }

    #[test]
    fn test_get_next_day_with_non_leap_year_last_day() {
        let year = 2021;
        let day_of_year = 365;
        let (next_year, next_day) = get_next_day(year, day_of_year);
        assert_eq!(next_year, 2022);
        assert_eq!(next_day, 1);
    }

    #[test]
    fn test_sample_with_no_exist_day() {
        let mut nav_data_store = NavDataProvider::new("/mnt/d/GNSS_Data/Data/Nav");
        let year = 2022;
        let day_of_year = 100;
        let sv = SV::new(Constellation::GPS, 1);
        let epoch = Epoch::from_gregorian(2022, 4, 10, 12, 0, 0, 0, TimeScale::GPST);

        let result = nav_data_store.sample(year, day_of_year, &sv, &epoch);

        assert_eq!(result, None);
    }

    #[rstest]
    #[case(100, 10, 1)]
    #[case(101, 11, 2)]
    #[case(105, 15, 5)]
    fn test_sample_with_single_interpolation(
        #[case] day_of_year: u16,
        #[case] day: u8,
        #[case] prn: u8,
        #[values("g", "c", "r", "e")] s: &str,
    ) {
        let mut nav_data_store = NavDataProvider::new("/mnt/d/GNSS_Data/Data/Nav");
        let c = Constellation::from_str(s).unwrap();
        let sv = SV::new(c, prn);
        let epoch = Epoch::from_gregorian(2021, 4, day, 12, 0, 0, 0, TimeScale::GPST);
        nav_data_store.update_data(21, day_of_year);
        if let Some(interpolation) = nav_data_store.single_interpolation.as_ref() {
            let sample_results = interpolation.samples(&sv, &epoch);
            sample_results.iter().for_each(|(_, r)| {
                assert!(r.is_ok());
                assert!(r.as_ref().unwrap().is_sampled() || r.as_ref().unwrap().is_guessed());
            });
        } else {
            panic!("No single interpolation found");
        }
    }

    #[rstest]
    #[case("G01")]
    #[case("G02")]
    #[case("C01")]
    #[case("C02")]
    #[case("R03")]
    #[case("R04")]
    #[case("E03")]
    #[case("E05")]
    #[case("S38")]
    #[case("S43")]
    fn test_sample_with_cross_interpolation(#[case] sv: String) {
        let mut nav_data_store = NavDataProvider::new("/mnt/d/GNSS_Data/Data/Nav");
        let year = 2020;
        let day_of_year = 366;
        let sv = SV::from_str(&sv).unwrap();

        let ts = match sv.constellation {
            Constellation::GPS => TimeScale::GPST,
            Constellation::Glonass => TimeScale::UTC,
            Constellation::BeiDou => TimeScale::BDT,
            Constellation::Galileo => TimeScale::GST,
            _ => TimeScale::GPST,
        };
        let epoch = Epoch::from_gregorian(2020, 12, 31, 23, 59, 0, 0, ts);

        nav_data_store.update_data(year - 2000, day_of_year);
        if let Some(interpolation) = nav_data_store.cross_interpolation.as_ref() {
            let sample_results = interpolation.samples(&sv, &epoch);
            sample_results.iter().for_each(|(_, r)| {
                assert!(r.is_ok());
                //assert!(r.as_ref().unwrap().is_sampled() || r.as_ref().unwrap().is_guessed());
            });
        } else {
            panic!("No single interpolation found");
        }
    }

    #[rstest]
    #[case(2021, 100, 10, "G01")]
    #[case(2021, 101, 11, "G02")]
    #[case(2021, 105, 15, "C01")]
    #[case(2021, 110, 20, "C02")]
    #[case(2021, 115, 25, "R03")]
    #[case(2021, 120, 30, "R04")]
    #[case(2020, 100, 10, "E03")]
    #[case(2023, 110, 20, "E05")]
    fn test_sample_with_existing_data(
        #[case] year: i32,
        #[case] day_of_year: i32,
        #[case] day: u8,
        #[case] sv: &str,
    ) {
        let mut nav_data_store = NavDataProvider::new("/mnt/d/GNSS_Data/Data/Nav");
        let sv = SV::from_str(sv).unwrap();
        let epoch = Epoch::from_gregorian(year, 4, day, 12, 55, 30, 0, TimeScale::GPST);

        let result = nav_data_store.sample(year as u16, day_of_year as u16, &sv, &epoch);

        assert!(result.is_some());
        //let sample_results = result.unwrap();
    }
    #[rstest]
    #[case(2022, 200, "G01")]
    #[case(2022, 201, "G02")]
    #[case(2022, 205, "C01")]
    #[case(2022, 210, "C02")]
    #[case(2022, 215, "R03")]
    #[case(2022, 220, "R04")]
    #[case(2022, 225, "E03")]
    #[case(2022, 230, "E05")]
    fn test_sample_with_no_data(#[case] year: i32, #[case] day_of_year: u16, #[case] sv: &str) {
        let mut nav_data_store = NavDataProvider::new("/mnt/d/GNSS_Data/Data/Nav");
        let sv = SV::from_str(sv).unwrap();
        let epoch = Epoch::from_gregorian(year, 4, 10, 12, 0, 0, 0, TimeScale::GPST);

        let result = nav_data_store.sample(year as u16, day_of_year, &sv, &epoch);

        assert!(result.is_none());
    }

    #[rstest]
    #[case(2020, 366, "G01")]
    #[case(2020, 366, "G02")]
    #[case(2020, 366, "C01")]
    #[case(2020, 366, "C02")]
    #[case(2020, 366, "R03")]
    #[case(2020, 366, "R04")]
    #[case(2020, 366, "E03")]
    #[case(2020, 366, "E05")]
    fn test_sample_with_cross_interpolation_existing_data(
        #[case] year: i32,
        #[case] day_of_year: u16,
        #[case] sv: &str,
    ) {
        let mut nav_data_store = NavDataProvider::new("/mnt/d/GNSS_Data/Data/Nav");
        let sv = SV::from_str(sv).unwrap();
        let epoch = Epoch::from_gregorian(year, 12, 31, 23, 59, 59, 0, TimeScale::GPST);

        let result = nav_data_store.sample(year as u16, day_of_year, &sv, &epoch);

        assert!(result.is_some());
    }

    #[test]
    fn test_sample_at_special_time_point() {
        let mut nav_data_store = NavDataProvider::new("/mnt/d/GNSS_Data/Data/Nav");
        let sv = SV::from_str("C01").unwrap();
        let epoch = Epoch::from_gregorian(2021, 3, 10, 01, 00, 00, 0, TimeScale::BDT);

        let result = nav_data_store.sample(21, 69, &sv, &epoch);

        assert!(result.is_some());
        let index = CONSTELLATION_KEYS
            .get(&Constellation::BeiDou)
            .unwrap()
            .iter()
            .position(|k| *k == "crs")
            .unwrap();
        let results = result.unwrap();
        assert_eq!(results[index], 7.542812500000E+02_f64);
        let index = CONSTELLATION_KEYS
            .get(&Constellation::BeiDou)
            .unwrap()
            .iter()
            .position(|k| *k == "cic")
            .unwrap();
        assert_eq!(results[index], -1.094304025173E-07_f64);
        let index = CONSTELLATION_KEYS
            .get(&Constellation::BeiDou)
            .unwrap()
            .iter()
            .position(|k| *k == "crc")
            .unwrap();
        assert_eq!(results[index], -2.069062500000E+02_f64);
    }

    #[test]
    fn test_sample_at_no_exists_time_point() {
        let mut nav_data_store = NavDataProvider::new("/mnt/d/GNSS_Data/Data/Nav");
        let sv = SV::from_str("R01").unwrap();
        let epoch = Epoch::from_gregorian(2020, 3, 14, 00, 20, 00, 0, TimeScale::UTC);

        let result = nav_data_store.sample(20, 74, &sv, &epoch);

        assert!(result.is_some());
        let results = result.unwrap();
        let index = CONSTELLATION_KEYS
            .get(&Constellation::Glonass)
            .unwrap()
            .iter()
            .position(|k| *k == "satPosX")
            .unwrap();
        assert!(results[index] > 1.031281201172E+04_f64 && results[index] < 1.358619677734E+04_f64);
        let index = CONSTELLATION_KEYS
            .get(&Constellation::Glonass)
            .unwrap()
            .iter()
            .position(|k| *k == "satPosY")
            .unwrap();
        assert!(
            results[index] < -1.483764208984E+04_f64 && results[index] > -1.694177734375E+04_f64
        );
        let index = CONSTELLATION_KEYS
            .get(&Constellation::Glonass)
            .unwrap()
            .iter()
            .position(|k| *k == "velZ")
            .unwrap();
        assert!(results[index] > 2.213027954102E+00_f64 && results[index] < 2.895979881287E+00_f64);
    }

    #[test]
    fn test_sample_at_two_year_boundary() {
        let mut nav_data_store = NavDataProvider::new("/mnt/d/GNSS_Data/Data/Nav");
        let sv = SV::from_str("S38").unwrap();
        let epoch = Epoch::from_gregorian(2020, 12, 31, 23, 59, 59, 0, TimeScale::GPST);

        let result = nav_data_store.sample(20, 366, &sv, &epoch);

        assert!(result.is_some());
        let results = result.unwrap();
        let index = CONSTELLATION_KEYS
            .get(&Constellation::SBAS)
            .unwrap()
            .iter()
            .position(|k| *k == "satPosX")
            .unwrap();
        assert!(
            results[index] > -1.252322560000E+04_f64 && results[index] < -1.252299320000E+04_f64
        );
        let index = CONSTELLATION_KEYS
            .get(&Constellation::SBAS)
            .unwrap()
            .iter()
            .position(|k| *k == "satPosY")
            .unwrap();
        assert!(
            results[index] > -4.025408624000E+04_f64 && results[index] < -4.025389512000E+04_f64
        );
        let index = CONSTELLATION_KEYS
            .get(&Constellation::SBAS)
            .unwrap()
            .iter()
            .position(|k| *k == "velZ")
            .unwrap();
        assert!(results[index] > 1.940000000000E-03_f64 && results[index] < 1.944000000000E-03_f64);
    }

    #[test]
    fn test_sample_for_galileo() {
        let mut nav_data_store = NavDataProvider::new("/mnt/d/GNSS_Data/Data/Nav");
        let sv = SV::from_str("E01").unwrap();
        let epoch = Epoch::from_gregorian(2020, 1, 1, 0, 0, 0, 0, TimeScale::GPST);

        let result = nav_data_store.sample(20, 1, &sv, &epoch);

        assert!(result.is_some());
        assert_eq!(result.unwrap()[0], -7.641562260687E-04);
    }
}

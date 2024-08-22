use std::{collections::HashMap, fmt::Debug};

use rinex::{
    navigation::{Ephemeris, OrbitItem},
    prelude::{Constellation, Epoch, SV},
};
use splines::{Interpolation, Key, Spline};

use crate::constellation_keys::CONSTELLATION_KEYS;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
/// Represents the result of a sample.
pub(crate) enum SampleResult {
    /// The sample was successfully retrieved.
    Sampled(f64),
    /// The sample was retrieved, but the time was clamped.
    /// This means that the time was outside the range of the navigation data, and the sample was
    /// clamped to the nearest value.
    Clamped(f64),
    /// The value not present in the navigation data. We guessed the value.
    Guessed(f64),
}

impl Debug for SampleResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            SampleResult::Sampled(value) => write!(f, "Sampled({})", value),
            SampleResult::Clamped(value) => write!(f, "Clamped({})", value),
            SampleResult::Guessed(value) => write!(f, "Guessed({})", value),
        }
    }
}

impl PartialEq<f64> for SampleResult {
    fn eq(&self, other: &f64) -> bool {
        self.value() == *other
    }
}

impl PartialOrd<f64> for SampleResult {
    fn partial_cmp(&self, other: &f64) -> Option<std::cmp::Ordering> {
        self.value().partial_cmp(other)
    }
}

impl SampleResult {
    /// Retrieves the value of the sample result.
    pub(crate) fn value(&self) -> f64 {
        match *self {
            SampleResult::Sampled(value)
            | SampleResult::Clamped(value)
            | SampleResult::Guessed(value) => value,
        }
    }
    /// Returns `true` if the sample was successfully retrieved.
    /// Otherwise, returns `false` if the value is clamped or guessed.
    pub(crate) fn is_sampled(&self) -> bool {
        matches!(self, SampleResult::Sampled(_))
    }

    /// Returns `true` if the value is clamped.
    pub(crate) fn is_clamped(&self) -> bool {
        matches!(self, SampleResult::Clamped(_))
    }

    /// Returns `true` if the value is guessed.
    pub(crate) fn is_guessed(&self) -> bool {
        matches!(self, SampleResult::Guessed(_))
    }

    /// Creates a new `SampleResult::Sampled` instance from a sampled value.
    pub(crate) fn from_sampled(value: f64) -> Self {
        SampleResult::Sampled(value)
    }

    /// Creates a new `SampleResult::Clamped` instance from a clamped value.
    pub(crate) fn from_clamped(value: f64) -> Self {
        SampleResult::Clamped(value)
    }

    /// Creates a new `SampleResult::Guessed` instance from a guessed value.
    pub(crate) fn from_guessed(value: f64) -> Self {
        SampleResult::Guessed(value)
    }
}

/// A structure for interpolating navigation data.
pub(crate) struct NavDataInterpolation {
    //multi_navigation_data: &'a HashMap<SV, Vec<(Epoch, Ephemeris)>>,
    /// A `HashMap` containing the navigation data records for each satellite.
    /// For a given satellite, the key is the navigation record name and the value is a vector of
    /// epoch and value pair.
    sv_nav_keys: HashMap<SV, HashMap<String, Vec<Key<f64, f64>>>>,
}
#[allow(dead_code)]
impl NavDataInterpolation {
    /// Creates a new instance of `NavDataInterpolation`.
    ///
    /// # Arguments
    ///
    /// * `multi_navigation_data` - A `HashMap` containing navigation data for multiple satellites.
    ///
    /// # Returns
    ///
    /// A new instance of `NavDataInterpolation`.
    ///
    /// # Example
    ///
    /// ```
    /// use std::collections::HashMap;
    ///
    /// let multi_navigation_data: HashMap<SV, Vec<(Epoch, Ephemeris)>> = HashMap::new();
    /// let nav_data_interpolation = NavDataInterpolation::new(multi_navigation_data);
    /// ```
    pub(crate) fn new(multi_navigation_data: &HashMap<SV, Vec<(Epoch, Ephemeris)>>) -> Self {
        let constellation_keys = &CONSTELLATION_KEYS;
        let mut sv_nav_keys: HashMap<SV, HashMap<String, Vec<Key<f64, f64>>>> = HashMap::new();
        for (sv, nav_data) in multi_navigation_data {
            if !sv_nav_keys.contains_key(sv) {
                let mut _nav_keys: HashMap<String, Vec<Key<f64, f64>>> = HashMap::new();
                sv_nav_keys.insert(*sv, _nav_keys);
            }
            if let Some(nav_keys) = sv_nav_keys.get_mut(sv) {
                let constellation = if sv.constellation.is_sbas() {
                    Constellation::SBAS
                } else {
                    sv.constellation
                };
                if let Some(constellation) = constellation_keys.get(&constellation) {
                    constellation.iter().for_each(|v| {
                        if !nav_keys.contains_key(&v.to_string()) {
                            nav_keys.insert(v.to_string(), Vec::new());
                        }
                    });
                } else {
                    // Handle the case when the key is not found in constellation_keys.
                    // You can choose to log an error, return an error, or take any other appropriate action.
                    // Here, we are printing a warning message.
                    panic!("Warning: Constellation key not found for SV: {:?}", sv);
                }

                for (epoch, eph) in nav_data.clone() {
                    let time_of_seconds = epoch.to_duration_since_j1900().to_seconds();
                    let key = Key::new(time_of_seconds, eph.clock_bias, Interpolation::Linear);
                    nav_keys.get_mut("clock_bias").unwrap().push(key);

                    let key = Key::new(time_of_seconds, eph.clock_drift, Interpolation::Linear);
                    nav_keys.get_mut("clock_drift").unwrap().push(key);

                    let key =
                        Key::new(time_of_seconds, eph.clock_drift_rate, Interpolation::Linear);
                    nav_keys.get_mut("clock_drift_rate").unwrap().push(key);

                    for (prn, orbit) in &eph.orbits {
                        if nav_keys.contains_key(prn) {
                            match orbit {
                                OrbitItem::F64(value) => {
                                    let key =
                                        Key::new(time_of_seconds, *value, Interpolation::Linear);
                                    nav_keys.get_mut(prn).unwrap().push(key);
                                }
                                OrbitItem::U32(value) => {
                                    let key = Key::new(
                                        time_of_seconds,
                                        *value as f64,
                                        Interpolation::Step(1.0),
                                    );
                                    nav_keys.get_mut(prn).unwrap().push(key);
                                }
                                OrbitItem::U8(value) => {
                                    let key = Key::new(
                                        time_of_seconds,
                                        *value as f64,
                                        Interpolation::Step(1.0),
                                    );
                                    nav_keys.get_mut(prn).unwrap().push(key);
                                }
                                OrbitItem::I8(value) => {
                                    let key = Key::new(
                                        time_of_seconds,
                                        *value as f64,
                                        Interpolation::Step(1.0),
                                    );
                                    nav_keys.get_mut(prn).unwrap().push(key);
                                }
                                OrbitItem::Health(value) => {
                                    let key = Key::new(
                                        time_of_seconds,
                                        value.clone() as i32 as f64,
                                        Interpolation::Step(1.0),
                                    );
                                    nav_keys.get_mut(prn).unwrap().push(key);
                                }
                                OrbitItem::GalHealth(value) => {
                                    let key = Key::new(
                                        time_of_seconds,
                                        value.clone().bits() as f64,
                                        Interpolation::Step(1.0),
                                    );
                                    nav_keys.get_mut(prn).unwrap().push(key);
                                }
                                OrbitItem::GeoHealth(value) => {
                                    let key = Key::new(
                                        time_of_seconds,
                                        value.clone() as i32 as f64,
                                        Interpolation::Step(1.0),
                                    );
                                    nav_keys.get_mut(prn).unwrap().push(key);
                                }
                                OrbitItem::GloHealth(value) => {
                                    let key = Key::new(
                                        time_of_seconds,
                                        value.clone() as i32 as f64,
                                        Interpolation::Step(1.0),
                                    );
                                    nav_keys.get_mut(prn).unwrap().push(key);
                                }
                                // do nothing  for other types
                                _ => {}
                            }
                        }
                    }
                }
            }
        }

        Self {
            //multi_navigation_data,
            sv_nav_keys,
        }
    }

    ///
    /// Retrieves a sample value for a given satellite, time, and data record name.
    ///
    /// # Arguments
    ///
    /// * `sv` - The satellite identifier.
    /// * `time` - The time at which to retrieve the sample value.
    /// * `record` - The navigation data record name.
    ///
    /// # Returns
    ///
    /// The sample value.
    ///
    /// # Errors
    ///
    /// Errors occured if the navigation data does not exist for the given satellite and record name.    
    fn sample(&self, sv: &SV, time: f64, record: &str) -> Result<SampleResult, String> {
        if let Some(keys) = self
            .sv_nav_keys
            .get(sv)
            .and_then(|nav_keys| nav_keys.get(record))
        {
            let spline = Spline::from_vec(keys.clone());
            if keys.is_empty() {
                return Ok(SampleResult::from_guessed(0.00));
            }
            if time >= keys[0].t && time < keys[keys.len() - 1].t {
                Ok(SampleResult::from_sampled(spline.sample(time).unwrap()))
            } else {
                Ok(SampleResult::from_clamped(
                    spline.clamped_sample(time).unwrap(),
                ))
            }
        } else {
            Err(format!(
                "Data not found for the given SV:\"{}\" and record \"{}\".",
                sv, record
            ))
        }
    }

    /// Retrieves a sample value for a given satellite and epoch.
    ///
    /// # Arguments
    ///
    /// * `sv` - The satellite identifier.
    /// * `epoch` - The epoch at which to retrieve the sample values.
    ///
    /// # Returns
    ///
    /// A `HashMap` containing the sample values for each data record.
    pub(crate) fn samples(
        &self,
        sv: &SV,
        epoch: &Epoch,
    ) -> HashMap<String, Result<SampleResult, String>> {
        let time: f64 = epoch.to_duration_since_j1900().to_seconds();
        let mut samples = HashMap::new();
        self.sv_nav_keys[sv].iter().for_each(|(record, _)| {
            samples.insert(record.to_string(), self.sample(sv, time, record));
        });
        samples
    }
}

#[cfg(test)]
mod tests {

    use rinex::prelude::Constellation::GPS;

    use super::*;

    #[test]
    fn test_new() {
        let multi_navigation_data: HashMap<SV, Vec<(Epoch, Ephemeris)>> = HashMap::new();
        let nav_data_interpolation = NavDataInterpolation::new(&multi_navigation_data);

        // Assert that the `SingleFileNavDataInterpolation` instance is created correctly
        assert_eq!(nav_data_interpolation.sv_nav_keys.len(), 0);
    }

    #[test]
    fn test_new_with_data() {
        let epoch1 = Epoch::from_gpst_days(65536.123);
        let epoch2 = Epoch::from_gpst_days(65536.223);
        let eph1 = Ephemeris {
            clock_bias: 1.0,
            clock_drift: 2.0,
            clock_drift_rate: 3.0,
            orbits: HashMap::new(),
        };
        let eph2 = Ephemeris {
            clock_bias: 3.0,
            clock_drift: 4.0,
            clock_drift_rate: 3.0,
            orbits: HashMap::new(),
        };

        let mut multi_navigation_data: HashMap<SV, Vec<(Epoch, Ephemeris)>> = HashMap::new();
        multi_navigation_data.insert(SV::new(GPS, 1), vec![(epoch1, eph1), (epoch2, eph2)]);

        let nav_data_interpolation = NavDataInterpolation::new(&multi_navigation_data);

        assert_eq!(nav_data_interpolation.sv_nav_keys.len(), 1);
        assert_eq!(
            nav_data_interpolation.sv_nav_keys[&SV::new(GPS, 1)]["clock_bias"].len(),
            2
        );
        assert_eq!(
            nav_data_interpolation.sv_nav_keys[&SV::new(GPS, 1)]["clock_drift"].len(),
            2
        );
    }

    #[test]
    fn test_new_with_orbits() {
        let epoch1 = Epoch::from_gpst_days(65536.123);
        let epoch2 = Epoch::from_gpst_days(65536.223);

        let mut orbits1 = HashMap::new();
        orbits1.insert("crs".to_string(), OrbitItem::U32(12345));
        orbits1.insert("cus".to_string(), OrbitItem::F64(32345.05));

        let mut orbits2 = HashMap::new();
        orbits2.insert("crs".to_string(), OrbitItem::U32(12346));
        orbits2.insert("cus".to_string(), OrbitItem::F64(32355.05));

        let eph1 = Ephemeris {
            clock_bias: 1.0,
            clock_drift: 2.0,
            clock_drift_rate: 3.0,
            orbits: orbits1,
        };
        let eph2 = Ephemeris {
            clock_bias: 3.0,
            clock_drift: 4.0,
            clock_drift_rate: 3.0,
            orbits: orbits2,
        };

        let mut multi_navigation_data: HashMap<SV, Vec<(Epoch, Ephemeris)>> = HashMap::new();
        multi_navigation_data.insert(SV::new(GPS, 1), vec![(epoch1, eph1), (epoch2, eph2)]);

        let nav_data_interpolation = NavDataInterpolation::new(&multi_navigation_data);

        assert_eq!(
            nav_data_interpolation.sv_nav_keys[&SV::new(GPS, 1)]["crs"].len(),
            2
        );
        assert_eq!(
            nav_data_interpolation.sv_nav_keys[&SV::new(GPS, 1)]["cus"].len(),
            2
        );
    }

    #[test]
    fn test_samples() {
        let epoch1 = Epoch::from_gpst_days(65536.123);
        let epoch2 = Epoch::from_gpst_days(65538.123);
        let eph1 = Ephemeris {
            clock_bias: 1.0,
            clock_drift: 2.0,
            clock_drift_rate: 3.0,
            orbits: HashMap::new(),
        };
        let eph2 = Ephemeris {
            clock_bias: 3.0,
            clock_drift: 4.0,
            clock_drift_rate: 3.0,
            orbits: HashMap::new(),
        };

        let mut multi_navigation_data: HashMap<SV, Vec<(Epoch, Ephemeris)>> = HashMap::new();
        multi_navigation_data.insert(
            SV::new(Constellation::BeiDou, 1),
            vec![(epoch1, eph1), (epoch2, eph2)],
        );

        let nav_data_interpolation = NavDataInterpolation::new(&multi_navigation_data);

        let samples = nav_data_interpolation.samples(&SV::new(Constellation::BeiDou, 1), &epoch1);

        // Assert that the samples are retrieved correctly
        assert_eq!(
            samples.len(),
            CONSTELLATION_KEYS
                .get(&Constellation::BeiDou)
                .unwrap()
                .len()
        );
        assert_eq!(samples["clock_bias"].clone().unwrap(), 1.0);
        assert_eq!(samples["clock_drift"].clone().unwrap(), 2.0);
        assert_eq!(samples["clock_drift_rate"].clone().unwrap(), 3.0);

        let sample_epoch = Epoch::from_gpst_days(65537.123);
        let samples =
            nav_data_interpolation.samples(&SV::new(Constellation::BeiDou, 1), &sample_epoch);
        assert_eq!(samples["clock_bias"].clone().unwrap(), 2.0);
        assert_eq!(samples["clock_drift"].clone().unwrap(), 3.0);
        assert_eq!(samples["clock_drift_rate"].clone().unwrap(), 3.0);

        let samples = nav_data_interpolation.samples(&SV::new(Constellation::BeiDou, 1), &epoch2);

        // Assert that the samples are retrieved correctly
        assert_eq!(samples["clock_bias"].clone().unwrap(), 3.0);
        assert_eq!(samples["clock_drift"].clone().unwrap(), 4.0);
        assert_eq!(samples["clock_drift_rate"].clone().unwrap(), 3.0);
    }

    #[test]
    fn test_samples_with_orbits() {
        let epoch1 = Epoch::from_gpst_days(65536.123);
        let epoch2 = Epoch::from_gpst_days(65538.123);

        let mut orbits1 = HashMap::new();
        orbits1.insert("crs".to_string(), OrbitItem::U32(12345));
        orbits1.insert("cus".to_string(), OrbitItem::F64(32345.05));

        let mut orbits2 = HashMap::new();
        orbits2.insert("crs".to_string(), OrbitItem::U32(12346));
        orbits2.insert("cus".to_string(), OrbitItem::F64(32355.05));

        let eph1 = Ephemeris {
            clock_bias: 1.0,
            clock_drift: 2.0,
            clock_drift_rate: 3.0,
            orbits: orbits1,
        };
        let eph2 = Ephemeris {
            clock_bias: 3.0,
            clock_drift: 4.0,
            clock_drift_rate: 3.0,
            orbits: orbits2,
        };

        let mut multi_navigation_data: HashMap<SV, Vec<(Epoch, Ephemeris)>> = HashMap::new();
        multi_navigation_data.insert(SV::new(GPS, 1), vec![(epoch1, eph1), (epoch2, eph2)]);

        let nav_data_interpolation = NavDataInterpolation::new(&multi_navigation_data);

        let samples = nav_data_interpolation.samples(&SV::new(GPS, 1), &epoch1);

        // Assert that the samples with orbits are retrieved correctly
        assert_eq!(
            samples.len(),
            CONSTELLATION_KEYS.get(&Constellation::GPS).unwrap().len()
        );
        assert_eq!(samples["clock_bias"].clone().unwrap(), 1.0);
        assert_eq!(samples["clock_drift"].clone().unwrap(), 2.0);
        assert_eq!(samples["clock_drift_rate"].clone().unwrap(), 3.0);
        assert_eq!(samples["crs"].clone().unwrap(), 12345.0);
        assert_eq!(samples["cus"].clone().unwrap(), 32345.05);

        let sample_epoch = Epoch::from_gpst_days(65537.123);
        let samples = nav_data_interpolation.samples(&SV::new(GPS, 1), &sample_epoch);
        assert_eq!(
            samples.len(),
            CONSTELLATION_KEYS.get(&Constellation::GPS).unwrap().len()
        );
        assert_eq!(samples["clock_bias"].clone().unwrap(), 2.0);
        assert_eq!(samples["clock_drift"].clone().unwrap(), 3.0);
        assert_eq!(samples["clock_drift_rate"].clone().unwrap(), 3.0);
        assert_eq!(samples["crs"].clone().unwrap(), 12345.0);
        assert_eq!(samples["cus"].clone().unwrap(), 32350.05);

        let samples = nav_data_interpolation.samples(&SV::new(GPS, 1), &epoch2);
        assert_eq!(
            samples.len(),
            CONSTELLATION_KEYS.get(&Constellation::GPS).unwrap().len()
        );
        assert_eq!(samples["clock_bias"].clone().unwrap(), 3.0);
        assert_eq!(samples["clock_drift"].clone().unwrap(), 4.0);
        assert_eq!(samples["clock_drift_rate"].clone().unwrap(), 3.0);
        assert_eq!(samples["crs"].clone().unwrap(), 12346.0);
        assert_eq!(samples["cus"].clone().unwrap(), 32355.05);
    }
}

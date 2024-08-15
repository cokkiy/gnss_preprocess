#[allow(dead_code)]
use std::{collections::HashMap, error::Error};

use rinex::{
    navigation::Ephemeris,
    prelude::{Epoch, SV},
    Rinex,
};

pub(crate) type NavigationData = HashMap<SV, Vec<(Epoch, Ephemeris)>>;

/// Reads a navigation file and extracts the satellite trajectory information from it.
///
/// # Arguments
///
/// * `nav_file` - The path to the navigation file.
///
/// # Returns
///
/// A `Result` containing the navigation data as a `HashMap` where the key is the satellite vehicle (SV) and the value is a vector of tuples containing the epoch and ephemeris data.
///
/// # Errors
///
/// Returns an error if there is an issue reading the navigation file or parsing its contents.
///
/// # Example
///
/// ```
/// use std::collections::HashMap;
/// use rinex::prelude::{Epoch, SV};
/// use crate::navigation_data::NavigationData;
///
/// let nav_file = "/path/to/navigation_file.nav";
/// let result = get_navigation_data(nav_file);
/// match result {
///     Ok(navigation_data) => {
///         println!("Navigation data: {:?}", navigation_data);
///     }
///     Err(err) => {
///         eprintln!("Error: {}", err);
///     }
/// }
/// ```
pub(crate) fn get_navigation_data(nav_file: &str) -> Result<NavigationData, Box<dyn Error>> {
    // 读取导航文件
    let nav = Rinex::from_file(nav_file)?;

    // 提取导航中的卫星轨迹信息
    let mut multi_navigation_data: NavigationData = HashMap::new();

    for (epoch, nav_frames) in nav.navigation() {
        for frame in nav_frames {
            if let Some((_, sv, eph)) = frame.as_eph() {
                if let Some(data) = multi_navigation_data.get_mut(&sv) {
                    data.push((*epoch, eph.clone()));
                } else {
                    let navigation_data = vec![(*epoch, eph.clone())];
                    multi_navigation_data.insert(sv, navigation_data);
                }
            }
        }
    }

    Ok(multi_navigation_data)
}

/// Given a navigation data, this function returns a new navigation data containing only the first epoch of each satellite for the next day.
///
/// # Arguments
///
/// * `next_day_navigation_data` - A reference to the navigation data for the next day.
///
/// # Returns
///
/// A new navigation data containing only the first epoch of each satellite for the next day.
///
/// # Example
///
/// ```
/// use std::collections::HashMap;
/// use rinex::prelude::{Epoch, SV};
/// use crate::navigation_data::NavigationData;
///
/// let mut navigation_data: NavigationData = HashMap::new();
/// navigation_data.insert(SV::new(Constellation::GPS, 1), vec![(Epoch::new(), Ephemeris::new())]);
///
/// let result = get_next_day_first_epoch(&navigation_data);
/// assert_eq!(result.len(), 1);
/// assert_eq!(result.contains_key(&SV::new(Constellation::GPS, 1)), true);
/// assert_eq!(result.get(&SV::new(Constellation::GPS, 1)).unwrap().len(), 1);
/// ```
pub(crate) fn get_next_day_first_epoch(
    next_day_navigation_data: &NavigationData,
) -> NavigationData {
    let mut next_day_first_epoch: NavigationData = HashMap::new();
    for (sv, ephemeris) in next_day_navigation_data {
        let first_epoch = ephemeris[0].0;
        next_day_first_epoch.insert(*sv, vec![(first_epoch, ephemeris[0].1.clone())]);
    }
    next_day_first_epoch
}

/// Given a navigation data, this function returns a new navigation data containing only the last epoch of each satellite for the current day.
///
/// # Arguments
///
/// * `current_day_navigation_data` - A reference to the navigation data for the current day.
///
/// # Returns
///
/// A new navigation data containing only the last epoch of each satellite for the current day.
///
/// # Example
///
/// ```
/// use std::collections::HashMap;
/// use rinex::prelude::{Epoch, SV};
/// use crate::navigation_data::NavigationData;
///     let mut current_day_navigation_data: NavigationData = HashMap::new();
/// let mut navigation_data: NavigationData = HashMap::new();
/// navigation_data.insert(SV::new(Constellation::GPS, 1), vec![(Epoch::new(), Ephemeris::new())]);
///
/// let result = get_current_day_last_epoch(&navigation_data);
/// assert_eq!(result.len(), 1);
/// assert_eq!(result.contains_key(&SV::new(Constellation::GPS, 1)), true);
/// assert_eq!(result.get(&SV::new(Constellation::GPS, 1)).unwrap().len(), 1);
/// ```
pub(crate) fn get_current_day_last_epoch(
    current_day_navigation_data: &NavigationData,
) -> NavigationData {
    let mut current_day_last_epoch: NavigationData = HashMap::new();
    for (sv, ephemeris) in current_day_navigation_data {
        let last_epoch = ephemeris[ephemeris.len() - 1].0;
        current_day_last_epoch.insert(
            *sv,
            vec![(last_epoch, ephemeris[ephemeris.len() - 1].1.clone())],
        );
    }
    current_day_last_epoch
}

/// Combines the navigation data from the current day with the navigation data from the next day.
///
/// # Arguments
///
/// * `current_day_navigation_data` - A reference to the navigation data for the current day.
/// * `next_day_navigation_data` - A reference to the navigation data for the next day.
///
/// # Returns
///
/// A new navigation data containing the combined data from both days.
///
/// # Example
///
/// ```
/// use std::collections::HashMap;
/// let mut next_day_first_epoch: NavigationData = HashMap::new();
/// for (sv, ephemeris) in next_day_navigation_data {
///     let mut current_day_navigation_data: NavigationData = HashMap::new();
///     current_day_navigation_data.insert(SV::new(Constellation::GPS, 1), vec![(Epoch::new(), Ephemeris::new())]);
/// }
/// let mut next_day_navigation_data: NavigationData = HashMap::new();
/// next_day_navigation_data.insert(SV::new(Constellation::GPS, 2), vec![(Epoch::new(), Ephemeris::new())]);
///
/// let result = combine_navigation_data(&current_day_navigation_data, &next_day_navigation_data);
/// assert_eq!(result.len(), 2);
/// assert_eq!(result.contains_key(&SV::new(Constellation::GPS, 1)), true);
/// assert_eq!(result.contains_key(&SV::new(Constellation::GPS, 2)), true);
/// assert_eq!(result.get(&SV::new(Constellation::GPS, 1)).unwrap().len(), 1);
/// assert_eq!(result.get(&SV::new(Constellation::GPS, 2)).unwrap().len(), 1);
/// ```
pub(crate) fn combine_navigation_data(
    current_day_navigation_data: &NavigationData,
    next_day_navigation_data: &NavigationData,
) -> NavigationData {
    let mut combined_navigation_data: NavigationData = HashMap::new();
    for (sv, ephemeris) in current_day_navigation_data {
        let mut combined_ephemeris = ephemeris.clone();
        if let Some(next_day_ephemeris) = next_day_navigation_data.get(sv) {
            combined_ephemeris.extend(next_day_ephemeris.clone());
        }
        combined_navigation_data.insert(*sv, combined_ephemeris);
    }
    for (sv, ephemeris) in next_day_navigation_data {
        if !combined_navigation_data.contains_key(sv) {
            combined_navigation_data.insert(*sv, ephemeris.clone());
        }
    }
    combined_navigation_data
}

#[cfg(test)]
mod tests {
    use rinex::{
        navigation::OrbitItem,
        prelude::{Constellation, TimeScale},
    };

    use super::*;

    #[test]
    fn test_get_navigation_data() {
        // Test case 1: Empty navigation file
        let nav_file = "";
        let result = get_navigation_data(nav_file);
        assert!(result.is_err());

        // Test case 2: Valid navigation file with multiple epochs and SVs
        let nav_file = "/mnt/d/GNSS_Data/Data/Nav/2020/brdm0010.20p";
        let result = get_navigation_data(nav_file);
        assert!(result.is_ok());
        let navigation_data = result.unwrap();
        assert_eq!(navigation_data.len() > 0, true);
        assert_eq!(
            navigation_data.contains_key(&SV::new(Constellation::GPS, 1)),
            true
        );

        let epoch_ephemeris = navigation_data
            .get(&SV::new(Constellation::GPS, 1))
            .unwrap();
        assert!(
            epoch_ephemeris[0].0
                == Epoch::maybe_from_gregorian(2020, 1, 1, 0, 0, 0, 0, TimeScale::GPST).unwrap()
        );

        // Test case 3: Invalid navigation file
        let nav_file = "path/to/invalid_navigation_file.nav";
        let result = get_navigation_data(nav_file);
        assert!(result.is_err());
        // Add more assertions to validate the error type and message
    }

    #[test]
    fn test_get_next_day_first_epoch() {
        // Test case 1: Empty navigation data
        let navigation_data: NavigationData = HashMap::new();
        let result = get_next_day_first_epoch(&navigation_data);
        assert_eq!(result.len(), 0);

        // Test case 2: Navigation data with multiple satellites and epochs
        let mut navigation_data: NavigationData = HashMap::new();

        let mut orbits1 = HashMap::new();
        orbits1.insert("o1".to_string(), OrbitItem::U32(12345));
        orbits1.insert("o2".to_string(), OrbitItem::F64(32345.05));

        let mut orbits2 = HashMap::new();
        orbits2.insert("o1".to_string(), OrbitItem::U32(12346));
        orbits2.insert("o2".to_string(), OrbitItem::F64(32355.05));

        let mut orbits3 = HashMap::new();
        orbits3.insert("o1".to_string(), OrbitItem::U32(12347));
        orbits3.insert("o2".to_string(), OrbitItem::F64(32365.05));

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

        let eph3 = Ephemeris {
            clock_bias: 4.0,
            clock_drift: 5.0,
            clock_drift_rate: 3.0,
            orbits: orbits3,
        };

        navigation_data.insert(
            SV::new(Constellation::GPS, 1),
            vec![
                (Epoch::from_bdt_days(386089000.23), eph1.clone()),
                (Epoch::from_bdt_days(386089000.24), eph2.clone()),
                (Epoch::from_bdt_days(386089000.25), eph3.clone()),
            ],
        );
        navigation_data.insert(
            SV::new(Constellation::GPS, 2),
            vec![
                (Epoch::from_bdt_days(386089000.23), eph1.clone()),
                (Epoch::from_bdt_days(386089000.24), eph2.clone()),
            ],
        );
        navigation_data.insert(
            SV::new(Constellation::Glonass, 1),
            vec![(Epoch::from_bdt_days(386089000.23), eph1.clone())],
        );

        let result = get_next_day_first_epoch(&navigation_data);
        assert_eq!(result.len(), 3);
        assert_eq!(result.contains_key(&SV::new(Constellation::GPS, 1)), true);
        assert_eq!(result.contains_key(&SV::new(Constellation::GPS, 2)), true);
        assert_eq!(
            result.contains_key(&SV::new(Constellation::Glonass, 1)),
            true
        );
        assert_eq!(
            result.get(&SV::new(Constellation::GPS, 1)).unwrap().len(),
            1
        );
        assert_eq!(
            result.get(&SV::new(Constellation::GPS, 2)).unwrap().len(),
            1
        );
        assert_eq!(
            result
                .get(&SV::new(Constellation::Glonass, 1))
                .unwrap()
                .len(),
            1
        );
        assert_eq!(
            result.get(&SV::new(Constellation::GPS, 1)).unwrap()[0].0,
            Epoch::from_bdt_days(386089000.23)
        );
        assert_eq!(
            result.get(&SV::new(Constellation::GPS, 2)).unwrap()[0].0,
            Epoch::from_bdt_days(386089000.23)
        );
        assert_eq!(
            result.get(&SV::new(Constellation::Glonass, 1)).unwrap()[0].0,
            Epoch::from_bdt_days(386089000.23)
        );
    }

    #[test]
    fn test_get_current_day_last_epoch() {
        // Test case 1: Empty navigation data
        let navigation_data: NavigationData = HashMap::new();
        let result = get_current_day_last_epoch(&navigation_data);
        assert_eq!(result.len(), 0);

        // Test case 2: Navigation data with multiple satellites and epochs
        let mut navigation_data: NavigationData = HashMap::new();

        let mut orbits1 = HashMap::new();
        orbits1.insert("o1".to_string(), OrbitItem::U32(12345));
        orbits1.insert("o2".to_string(), OrbitItem::F64(32345.05));

        let mut orbits2 = HashMap::new();
        orbits2.insert("o1".to_string(), OrbitItem::U32(12346));
        orbits2.insert("o2".to_string(), OrbitItem::F64(32355.05));

        let mut orbits3 = HashMap::new();
        orbits3.insert("o1".to_string(), OrbitItem::U32(12347));
        orbits3.insert("o2".to_string(), OrbitItem::F64(32365.05));

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

        let eph3 = Ephemeris {
            clock_bias: 4.0,
            clock_drift: 5.0,
            clock_drift_rate: 3.0,
            orbits: orbits3,
        };

        navigation_data.insert(
            SV::new(Constellation::GPS, 1),
            vec![
                (Epoch::from_bdt_days(386089000.23), eph1.clone()),
                (Epoch::from_bdt_days(386089000.24), eph2.clone()),
                (Epoch::from_bdt_days(386089000.25), eph3.clone()),
            ],
        );
        navigation_data.insert(
            SV::new(Constellation::GPS, 2),
            vec![
                (Epoch::from_bdt_days(386089000.23), eph1.clone()),
                (Epoch::from_bdt_days(386089000.24), eph2.clone()),
            ],
        );
        navigation_data.insert(
            SV::new(Constellation::Glonass, 1),
            vec![(Epoch::from_bdt_days(386089000.23), eph1.clone())],
        );

        let result = get_current_day_last_epoch(&navigation_data);
        assert_eq!(result.len(), 3);
        assert_eq!(result.contains_key(&SV::new(Constellation::GPS, 1)), true);
        assert_eq!(result.contains_key(&SV::new(Constellation::GPS, 2)), true);
        assert_eq!(
            result.contains_key(&SV::new(Constellation::Glonass, 1)),
            true
        );
        assert_eq!(
            result.get(&SV::new(Constellation::GPS, 1)).unwrap().len(),
            1
        );
        assert_eq!(
            result.get(&SV::new(Constellation::GPS, 2)).unwrap().len(),
            1
        );
        assert_eq!(
            result
                .get(&SV::new(Constellation::Glonass, 1))
                .unwrap()
                .len(),
            1
        );
        assert_eq!(
            result.get(&SV::new(Constellation::GPS, 1)).unwrap()[0].0,
            Epoch::from_bdt_days(386089000.25)
        );
        assert_eq!(
            result.get(&SV::new(Constellation::GPS, 2)).unwrap()[0].0,
            Epoch::from_bdt_days(386089000.24)
        );
        assert_eq!(
            result.get(&SV::new(Constellation::Glonass, 1)).unwrap()[0].0,
            Epoch::from_bdt_days(386089000.23)
        );
    }

    #[test]
    fn test_combine_navigation_data() {
        // Test case 1: Empty navigation data
        let current_day_navigation_data: NavigationData = HashMap::new();
        let next_day_navigation_data: NavigationData = HashMap::new();
        let result =
            combine_navigation_data(&current_day_navigation_data, &next_day_navigation_data);
        assert_eq!(result.len(), 0);

        // Test case 2: Navigation data with multiple satellites and epochs
        let mut current_day_navigation_data: NavigationData = HashMap::new();
        let mut next_day_navigation_data: NavigationData = HashMap::new();

        let mut orbits1 = HashMap::new();
        orbits1.insert("o1".to_string(), OrbitItem::U32(12345));
        orbits1.insert("o2".to_string(), OrbitItem::F64(32345.05));

        let mut orbits2 = HashMap::new();
        orbits2.insert("o1".to_string(), OrbitItem::U32(12346));
        orbits2.insert("o2".to_string(), OrbitItem::F64(32355.05));

        let mut orbits3 = HashMap::new();
        orbits3.insert("o1".to_string(), OrbitItem::U32(12347));
        orbits3.insert("o2".to_string(), OrbitItem::F64(32365.05));

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

        let eph3 = Ephemeris {
            clock_bias: 4.0,
            clock_drift: 5.0,
            clock_drift_rate: 3.0,
            orbits: orbits3,
        };

        current_day_navigation_data.insert(
            SV::new(Constellation::GPS, 1),
            vec![
                (Epoch::from_bdt_days(386089000.23), eph1.clone()),
                (Epoch::from_bdt_days(386089000.24), eph2.clone()),
            ],
        );
        current_day_navigation_data.insert(
            SV::new(Constellation::GPS, 2),
            vec![(Epoch::from_bdt_days(386089000.23), eph1.clone())],
        );

        next_day_navigation_data.insert(
            SV::new(Constellation::GPS, 2),
            vec![(Epoch::from_bdt_days(386089000.24), eph2.clone())],
        );
        next_day_navigation_data.insert(
            SV::new(Constellation::GPS, 3),
            vec![(Epoch::from_bdt_days(386089000.25), eph3.clone())],
        );

        let result =
            combine_navigation_data(&current_day_navigation_data, &next_day_navigation_data);
        assert_eq!(result.len(), 3);
        assert_eq!(result.contains_key(&SV::new(Constellation::GPS, 1)), true);
        assert_eq!(result.contains_key(&SV::new(Constellation::GPS, 2)), true);
        assert_eq!(result.contains_key(&SV::new(Constellation::GPS, 3)), true);
        assert_eq!(
            result.get(&SV::new(Constellation::GPS, 1)).unwrap().len(),
            2
        );
        assert_eq!(
            result.get(&SV::new(Constellation::GPS, 2)).unwrap().len(),
            2
        );
        assert_eq!(
            result.get(&SV::new(Constellation::GPS, 3)).unwrap().len(),
            1
        );
        assert_eq!(
            result.get(&SV::new(Constellation::GPS, 1)).unwrap()[0].0,
            Epoch::from_bdt_days(386089000.23)
        );
        assert_eq!(
            result.get(&SV::new(Constellation::GPS, 1)).unwrap()[1].0,
            Epoch::from_bdt_days(386089000.24)
        );
        assert_eq!(
            result.get(&SV::new(Constellation::GPS, 2)).unwrap()[0].0,
            Epoch::from_bdt_days(386089000.23)
        );
        assert_eq!(
            result.get(&SV::new(Constellation::GPS, 2)).unwrap()[1].0,
            Epoch::from_bdt_days(386089000.24)
        );
        assert_eq!(
            result.get(&SV::new(Constellation::GPS, 3)).unwrap()[0].0,
            Epoch::from_bdt_days(386089000.25)
        );
    }
}

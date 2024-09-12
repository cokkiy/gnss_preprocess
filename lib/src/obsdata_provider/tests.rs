use rinex::{
    observation::LliFlags,
    prelude::{Epoch, TimeScale},
};

use super::*;

#[test]
fn test_epoch_time_utc() {
    let epoch = Epoch::from_gregorian_hms(2020, 1, 1, 0, 0, 0, TimeScale::UTC);

    assert_eq!(
        epoch.to_gregorian_str(TimeScale::UTC),
        "2020-01-01T00:00:00 UTC"
    );

    assert_eq!(
        epoch.to_gregorian_str(TimeScale::GPST),
        "2020-01-01T00:00:00 GPST"
    );
}

#[test]
fn test_nth() {
    let v = vec![1, 2, 3, 4, 5];
    assert_eq!(v.iter().nth(0), Some(&1));
    assert_eq!(v.iter().nth(0), Some(&1));
    assert_eq!(v.iter().nth(2), Some(&3));
    assert_eq!(v.iter().nth(5), None);
}

#[test]
fn test_get_data() {
    let provider = ObsDataProvider {
        obs_file: Rinex::default(),
        index: 0,
        inner_index: 0,
        gps_fields: HashMap::from([("C1C", 4), ("L1C", 6), ("S1C", 8)]),
        glonass_fields: HashMap::new(),
        galileo_fields: HashMap::new(),
        beidou_fields: HashMap::new(),
        qzss_fields: HashMap::new(),
        irnss_fields: HashMap::new(),
        sbas_fields: HashMap::new(),
    };

    let mut observations = HashMap::new();
    observations.insert(
        Observable::PseudoRange("C1C".to_string()),
        ObservationData {
            obs: 20000000.0,
            lli: None,
            snr: Some(rinex::observation::SNR::DbHz18_23),
        },
    );
    observations.insert(
        Observable::Phase("L1C".to_string()),
        ObservationData {
            obs: 100000000.0,
            lli: Some(LliFlags::OK_OR_UNKNOWN),
            snr: Some(rinex::observation::SNR::DbHz36_41),
        },
    );
    observations.insert(
        Observable::SSI("S1C".to_string()),
        ObservationData {
            obs: 30.0,
            lli: None,
            snr: None,
        },
    );

    let result = provider.get_data(&observations, &provider.gps_fields);

    assert_eq!(result[4], 20000000.0);
    assert_eq!(result[5], 23.0);
    assert_eq!(result[6], 100000000.0);
    assert_eq!(result[7], 41.0);
    assert_eq!(result[8], 30.0);
    assert_eq!(result[9], 0.0); // No SNR for S1C
}

#[test]
fn test_vec_to_hash() {
    let input = vec!["C1C", "L1C", "S1C"];
    let result = ObsDataProvider::vec_to_hash(&input);

    assert_eq!(result.len(), 3);
    assert_eq!(result.get("C1C"), Some(&6));
    assert_eq!(result.get("L1C"), Some(&8));
    assert_eq!(result.get("S1C"), Some(&10));
    assert_eq!(result.get("D1C"), None);
}

#[test]
fn test_get_observable_field_name() {
    assert_eq!(
        ObsDataProvider::get_observable_field_name(&Observable::PseudoRange("C1C".to_string())),
        Some("C1C")
    );
    assert_eq!(
        ObsDataProvider::get_observable_field_name(&Observable::Phase("L1C".to_string())),
        Some("L1C")
    );
}

#[test]
fn test_next() {
    let provider = ObsDataProvider::new(PathBuf::from(
        "/mnt/d/GNSS_Data/Data/Obs/2020/001/daily/abmf0010.20o",
    ));
    let mut provider = provider.unwrap();
    let (sv, epoch, data) = provider.next().unwrap();
    assert_eq!(sv, SV::new(Constellation::GPS, 1));
    assert_eq!(
        epoch,
        Epoch::from_gregorian(2020, 1, 1, 0, 0, 0, 0, TimeScale::GPST)
    );
    assert_eq!(data[4], 1774604.6920);
    assert_eq!(data[5], 0.0);
    assert_eq!(data[6], 23059848.224);

    let (sv, epoch, _) = provider.next().unwrap();
    assert_eq!(sv, SV::new(Constellation::Galileo, 01));
    assert_eq!(
        epoch,
        Epoch::from_gregorian(2020, 1, 1, 0, 0, 0, 0, TimeScale::GPST)
    );

    let (sv, epoch, data) = provider.nth(41 - 2).unwrap();
    assert_eq!(sv, SV::new(Constellation::GPS, 01));
    assert_eq!(
        epoch,
        Epoch::from_gregorian(2020, 1, 1, 0, 0, 30, 0, TimeScale::GPST)
    );
    assert_eq!(data[6], 23040259.781);
    assert_eq!(data[8], 121077442.941);
}

// Add more tests for other methods and functionalities of ObsDataProvider

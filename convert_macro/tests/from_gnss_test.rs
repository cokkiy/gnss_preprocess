#[cfg(feature = "gnss")]
#[test]
fn test_field_pos() {
    use std::collections::HashMap;

    use convert_macro::FromGnss;
    use rinex::{
        observation::{LliFlags, ObservationData},
        prelude::Observable,
    };

    #[allow(unused)]
    #[derive(Default, FromGnss)]
    struct TestStruct {
        c1c: f64,
        l1c: f64,
        d1c: f64,
        s1c: f64,
    }

    let mut data: HashMap<Observable, ObservationData> = HashMap::new();
    data.insert(
        Observable::PseudoRange("c1c".to_string()),
        ObservationData::new(
            1.0,
            Some(LliFlags::OK_OR_UNKNOWN),
            Some(rinex::observation::SNR::DbHz0),
        ),
    );

    data.insert(
        Observable::Phase("l1c".to_string()),
        ObservationData::new(
            2.0,
            Some(LliFlags::OK_OR_UNKNOWN),
            Some(rinex::observation::SNR::DbHz0),
        ),
    );

    data.insert(
        Observable::Doppler("d1c".to_string()),
        ObservationData::new(
            3.0,
            Some(LliFlags::OK_OR_UNKNOWN),
            Some(rinex::observation::SNR::DbHz0),
        ),
    );

    data.insert(
        Observable::SSI("s1c".to_string()),
        ObservationData::new(
            4.0,
            Some(LliFlags::OK_OR_UNKNOWN),
            Some(rinex::observation::SNR::DbHz0),
        ),
    );

    let test_struct: TestStruct = (&data).into();
    assert!(test_struct.c1c == 1.0);
    assert!(test_struct.l1c == 2.0);
    assert!(test_struct.d1c == 3.0);
    assert!(test_struct.s1c == 4.0);
}

#[cfg(feature = "gnss")]
#[test]
fn test_from_gnss_some_field_no_exists() {
    use std::collections::HashMap;

    use convert_macro::FromGnss;
    use rinex::{
        observation::{LliFlags, ObservationData},
        prelude::Observable,
    };

    #[allow(unused)]
    #[derive(Default, FromGnss)]
    struct TestStruct {
        c1c: f64,
        l1c: f64,
        d1c: f64,
        s1c: f64,
        s2y: f64,
    }

    let mut data: HashMap<Observable, ObservationData> = HashMap::new();
    data.insert(
        Observable::PseudoRange("c1c".to_string()),
        ObservationData::new(
            1.0,
            Some(LliFlags::OK_OR_UNKNOWN),
            Some(rinex::observation::SNR::DbHz0),
        ),
    );

    data.insert(
        Observable::Phase("l1c".to_string()),
        ObservationData::new(
            2.0,
            Some(LliFlags::OK_OR_UNKNOWN),
            Some(rinex::observation::SNR::DbHz0),
        ),
    );

    data.insert(
        Observable::Doppler("d1c".to_string()),
        ObservationData::new(
            3.0,
            Some(LliFlags::OK_OR_UNKNOWN),
            Some(rinex::observation::SNR::DbHz0),
        ),
    );

    data.insert(
        Observable::SSI("s1c".to_string()),
        ObservationData::new(
            4.0,
            Some(LliFlags::OK_OR_UNKNOWN),
            Some(rinex::observation::SNR::DbHz0),
        ),
    );

    let test_struct: TestStruct = (&data).into();
    assert!(test_struct.c1c == 1.0);
    assert!(test_struct.l1c == 2.0);
    assert!(test_struct.d1c == 3.0);
    assert!(test_struct.s1c == 4.0);
    assert!(test_struct.s2y == 0.0);
}

#[cfg(feature = "gnss")]
#[test]
fn test_from_gnss_have_extra_value() {
    use std::collections::HashMap;

    use convert_macro::FromGnss;
    use rinex::{
        observation::{LliFlags, ObservationData},
        prelude::Observable,
    };

    #[allow(unused)]
    #[derive(Default, FromGnss)]
    struct TestStruct {
        c1c: f64,
        l1c: f64,
        d1c: f64,
    }

    let mut data: HashMap<Observable, ObservationData> = HashMap::new();
    data.insert(
        Observable::PseudoRange("c1c".to_string()),
        ObservationData::new(
            1.0,
            Some(LliFlags::OK_OR_UNKNOWN),
            Some(rinex::observation::SNR::DbHz0),
        ),
    );

    data.insert(
        Observable::Phase("l1c".to_string()),
        ObservationData::new(
            2.0,
            Some(LliFlags::OK_OR_UNKNOWN),
            Some(rinex::observation::SNR::DbHz0),
        ),
    );

    data.insert(
        Observable::Doppler("d1c".to_string()),
        ObservationData::new(
            3.0,
            Some(LliFlags::OK_OR_UNKNOWN),
            Some(rinex::observation::SNR::DbHz0),
        ),
    );

    data.insert(
        Observable::SSI("s1c".to_string()),
        ObservationData::new(
            4.0,
            Some(LliFlags::OK_OR_UNKNOWN),
            Some(rinex::observation::SNR::DbHz0),
        ),
    );

    let test_struct: TestStruct = (&data).into();
    assert!(test_struct.c1c == 1.0);
    assert!(test_struct.l1c == 2.0);
    assert!(test_struct.d1c == 3.0);
}

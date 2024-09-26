#[cfg(feature = "gnss-ssc")]
#[test]
fn test_gnss_ssc_for_empty_struct() {
    use convert_macro::SSC;
    use ssc::SignalStrengthComparer;

    #[allow(dead_code)]
    #[derive(Default, SSC)]
    struct Gps {}

    let gps1 = Gps::default();
    let gps2 = Gps::default();

    assert!(gps1.ss_compare(&gps2).is_empty());
}

#[cfg(feature = "gnss-ssc")]
#[test]
fn test_gnss_ssc_for_valid_struct() {
    use convert_macro::SSC;
    use ssc::SignalStrengthComparer;

    #[allow(dead_code)]
    #[derive(SSC)]
    struct Gps {
        c1c: f64,
        c1l: f64,
        s1c: f64,
        s1l: f64,
    }

    let gps1 = Gps {
        c1c: 1.0,
        c1l: 2.0,
        s1c: 3.0,
        s1l: 4.0,
    };
    let gps2 = Gps {
        c1c: 1.0,
        c1l: 2.0,
        s1c: 5.0,
        s1l: 6.0,
    };

    assert_eq!(gps1.ss_compare(&gps2), vec![-2.0, -2.0]);
}

#[cfg(feature = "gnss-ssc")]
#[test]
fn test_ssc_for_bad_struct() {
    use convert_macro::SSC;
    use ssc::SignalStrengthComparer;
    #[allow(dead_code)]
    #[derive(SSC)]
    struct Gps {
        c1c: f64,
        s1c: f64,
        s1l: f64,
        c1l: f64,
        s2p: f64,
    }

    let gps1 = Gps {
        c1c: 1.0,
        c1l: 2.0,
        s1c: 3.0,
        s1l: 4.0,
        s2p: 5.0,
    };

    let gps2 = Gps {
        c1c: 1.0,
        c1l: 2.0,
        s1c: 5.0,
        s1l: 6.0,
        s2p: 7.0,
    };

    assert_eq!(gps1.ss_compare(&gps2), vec![-2.0, -2.0, -2.0]);
}

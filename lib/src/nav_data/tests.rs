#[cfg(test)]
mod tests {
    use rinex::{prelude::Constellation, Rinex};

    use crate::nav_data::QZSSNavData;

    #[test]
    fn test_from_ephemeris_for_qzss_nav_data() {
        let rinex = Rinex::from_file("/mnt/d/GNSS_Data/Data/Nav/2020/brdm0010.20p").unwrap();
        let ephemeris = rinex
            .navigation()
            .into_iter()
            .find(|(_, frames)| {
                frames.iter().any(|frame| {
                    if let Some((_, sv, _)) = frame.as_eph() {
                        sv.constellation == Constellation::QZSS
                    } else {
                        false
                    }
                })
            })
            .unwrap()
            .1[0]
            .as_eph()
            .unwrap()
            .2;
        let qzss_nav_data = QZSSNavData::from(ephemeris);
        assert_eq!(qzss_nav_data.iode, 6.100000000000E+01);
    }
}

#[cfg(test)]
mod tests {
    use hifitime::{Epoch, TimeScale};
    use rinex::{navigation::Ephemeris, prelude::Constellation, Rinex};

    use crate::nav_data::{BeiDouNavData, GPSNavData, QZSSNavData};

    #[test]
    fn test_from_ephemeris_for_qzss_nav_data() {
        let rinex = Rinex::from_file("d:/Data/Nav/2020/brdm0010.20p").unwrap();
        let ephemeris = rinex
            .navigation()
            .into_iter()
            .find(|(epoch, frames)| {
                **epoch == Epoch::from_gregorian(2020, 1, 1, 0, 0, 0, 0, TimeScale::GPST)
            })
            .unwrap()
            .1
            .iter()
            .find(|frame| {
                if let Some((_, sv, _)) = frame.as_eph() {
                    sv.constellation == Constellation::QZSS && sv.prn == 1
                } else {
                    false
                }
            })
            .unwrap()
            .as_eph()
            .unwrap()
            .2;
        let qzss_nav_data = QZSSNavData::from(ephemeris);
        let expect = QZSSNavData {
            clock_bias: -3.263065591455E-04,
            clock_drift: 7.958078640513E-13,
            iode: 6.100000000000E+01,
            crs: -4.926875000000E+02,
            delta_n: 1.697927868368E-09,
            m0: -3.102037450940E+00,
            cuc: -1.518800854683E-05,
            e: 7.525891624391E-02,
            cus: 1.844204962254E-05,
            sqrt_a: 6.493624935150E+03,
            toe: 2.592000000000E+05,
            cic: 2.142041921616E-07,
            omega_0: 8.527570677262E-01,
            cis: -2.607703208923E-07,
            i0: 7.239225494342E-01,
            crc: -4.702187500000E+02,
            omega: -1.561414084558E+00,
            omega_dot: -2.013655305398E-09,
        };
        assert_eq!(qzss_nav_data, expect);
    }

    #[test]
    fn test_from_ephemeris_for_gps_nav_data() {
        let rinex = Rinex::from_file("d:/Data/Nav/2020/brdm0010.20p").unwrap();
        let ephemeris = rinex
            .navigation()
            .into_iter()
            .find(|(epoch, _)| {
                **epoch == Epoch::from_gregorian(2020, 1, 1, 0, 0, 0, 0, TimeScale::GPST)
            })
            .unwrap()
            .1
            .iter()
            .find(|frame| {
                if let Some((_, sv, _)) = frame.as_eph() {
                    sv.constellation == Constellation::GPS && sv.prn == 1
                } else {
                    false
                }
            })
            .unwrap()
            .as_eph()
            .unwrap()
            .2;

        let nav_data = GPSNavData::from(ephemeris);
        let expected = GPSNavData {
            clock_bias: -2.479013055563E-04,
            clock_drift: -1.216449163621E-11,
            iode: 7.200000000000E+01,
            crs: -1.956250000000E+01,
            delta_n: 4.218032840856E-09,
            m0: -1.483804704890E+00,
            cuc: -9.573996067047E-07,
            e: 9.235781384632E-03,
            cus: 3.913417458534E-06,
            sqrt_a: 5.153638690948E+03,
            toe: 2.592000000000E+05,
            cic: 2.067536115646E-07,
            omega_0: -5.786517456821E-01,
            cis: -1.005828380585E-07,
            i0: 9.785100924501E-01,
            crc: 3.135937500000E+02,
            omega: 7.594713900033E-01,
            omega_dot: -8.066050269084E-09,
        };
        assert_eq!(nav_data, expected);
    }

    #[test]
    fn test_from_ephemris_for_beidou_nav_data() {
        let rinex = Rinex::from_file("d:/Data/Nav/2020/brdm0010.20p").unwrap();
        let ephemeris = rinex
            .navigation()
            .into_iter()
            .find(|(epoch, _)| {
                **epoch == Epoch::from_gregorian(2020, 1, 1, 0, 0, 0, 0, TimeScale::GPST)
            })
            .unwrap()
            .1
            .iter()
            .find(|frame| {
                if let Some((_, sv, _)) = frame.as_eph() {
                    sv.constellation == Constellation::BeiDou && sv.prn == 1
                } else {
                    false
                }
            })
            .unwrap()
            .as_eph()
            .unwrap()
            .2;

        let nav_data = BeiDouNavData::from(ephemeris);
        let expected = BeiDouNavData {
            clock_bias: 3.270966699347E-04,
            clock_drift: 4.532019204362E-11,
            // age of data
            aode: 0.000000000000E+00,
            crs: -3.326250000000E+02,
            delta_n: 2.868690921063E-09,
            m0: 1.363339855457E+00,
            cuc: -1.106690615416E-05,
            e: 2.870542230085E-04,
            cus: 9.210780262947E-07,
            sqrt_a: 6.493465633392E+03,
            toe: 2.592000000000E+05,
            cic: 0.000000000000E+00,
            omega_0: -2.814852716703E+00,
            cis: -1.816079020500E-08,
            i0: 8.926346638738E-02,
            crc: -2.510937500000E+01,
            omega: -2.335007303661E+00,
            omega_dot: -1.726500487104E-09,
        };
        assert_eq!(nav_data, expected);
    }
}

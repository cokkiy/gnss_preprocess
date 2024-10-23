#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use gnss_rs::sv::{self, SV};
    use hifitime::{Epoch, TimeScale};
    use rinex::{prelude::Constellation, Rinex};

    use crate::nav_data::{
        BeiDouNavData, GPSNavData, GalileoNavData, GlonassNavData, QZSSNavData, SBASNavData,
    };

    #[test]
    fn test_from_ephemeris_for_qzss_nav_data() {
        let rinex = Rinex::from_file("/mnt/d/GNSS_Data/Data/Nav/2020/brdm0010.20p").unwrap();
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
            i_dot: 2.342954736326E-10,
        };
        assert_eq!(qzss_nav_data, expect);
    }

    #[test]
    fn test_from_ephemeris_for_gps_nav_data() {
        let rinex = Rinex::from_file("/mnt/d/GNSS_Data/Data/Nav/2020/brdm0010.20p").unwrap();
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
            i_dot: 5.714523747137E-12,
        };
        assert_eq!(nav_data, expected);
    }

    #[test]
    fn test_from_ephemeris_for_beidou_nav_data() {
        let rinex = Rinex::from_file("/mnt/d/GNSS_Data/Data/Nav/2020/brdm0010.20p").unwrap();
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
            i_dot: -3.000124967247E-10,
        };
        assert_eq!(nav_data, expected);
    }

    #[test]
    fn test_from_ephemeris_for_galileo_nav_data() {
        let rinex = Rinex::from_file("/mnt/d/GNSS_Data/Data/Nav/2020/brdm0010.20p").unwrap();
        let ephemeris = rinex
            .navigation()
            .into_iter()
            .find(|(epoch, _)| {
                **epoch == Epoch::from_gregorian(2020, 1, 1, 0, 30, 0, 0, TimeScale::GPST)
            })
            .unwrap()
            .1
            .iter()
            .find(|frame| {
                if let Some((_, sv, _)) = frame.as_eph() {
                    sv.constellation == Constellation::Galileo && sv.prn == 1
                } else {
                    false
                }
            })
            .unwrap()
            .as_eph()
            .unwrap()
            .2;

        let nav_data = GalileoNavData::from(ephemeris);
        let expected = GalileoNavData {
            clock_bias: -7.641562260687E-04,
            clock_drift: -7.887024366937E-12,
            iodnav: 5.100000000000E+01,
            crs: 1.628125000000E+01,
            delta_n: 2.929050578142E-09,
            m0: 2.400327625795E+00,
            cuc: 7.972121238708E-07,
            e: 1.714224927127E-04,
            cus: 7.882714271545E-06,
            sqrt_a: 5.440614101410E+03,
            toe: 2.610000000000E+05,
            cic: -3.911554813385E-08,
            omega_0: -2.976173319051E+00,
            cis: 2.793967723846E-08,
            i0: 9.852146869691E-01,
            crc: 1.827500000000E+02,
            omega: -5.489551362649E-01,
            omega_dot: -5.396653363703E-09,
            i_dot: -3.353711124101E-10,
        };
        assert_eq!(nav_data, expected);
    }

    #[test]
    fn test_from_ephemeris_for_glonass_nav_data() {
        let rinex = Rinex::from_file("/mnt/d/GNSS_Data/Data/Nav/2020/brdm0010.20p").unwrap();
        let ephemeris = rinex
            .navigation()
            .into_iter()
            .find(|(epoch, _)| {
                **epoch == Epoch::from_gregorian(2020, 1, 1, 0, 15, 0, 0, TimeScale::UTC)
            })
            .unwrap()
            .1
            .iter()
            .find(|frame| {
                if let Some((_, sv, _)) = frame.as_eph() {
                    sv.constellation == Constellation::Glonass && sv.prn == 1
                } else {
                    false
                }
            })
            .unwrap()
            .as_eph()
            .unwrap()
            .2;

        let nav_data = GlonassNavData::from(ephemeris);
        let expected = GlonassNavData {
            clock_bias: 5.495641380548E-05,
            clock_drift: 0.000000000000E+00,
            // message frame time
            mrt: 0.0,
            x: -1.242258300781E+04,
            vel_x: -2.059366226196E+00,
            accel_x: 9.313225746155E-10,
            health: 0.0,
            y: 2.215789062500E+03,
            vel_y: -2.229665756226E+00,
            accel_y: 0.000000000000E+00,
            z: 2.216454687500E+04,
            vel_z: -9.299058914185E-01,
            accel_z: -2.793967723846E-09,
            age: 0.0,
        };
        assert_eq!(nav_data, expected);
    }

    #[test]
    fn test_from_ephemeris_for_sbsa_nav_data() {
        let rinex = Rinex::from_file("/mnt/d/GNSS_Data/Data/Nav/2020/brdm0010.20p").unwrap();
        let _sv = SV::from_str("S22").unwrap();
        let ephemeris = rinex
            .navigation()
            .into_iter()
            .find(|(epoch, _)| {
                **epoch == Epoch::from_gregorian(2020, 1, 1, 0, 2, 08, 0, TimeScale::GPST)
            })
            .unwrap()
            .1
            .iter()
            .find(|frame| {
                if let Some((_, sv, _)) = frame.as_eph() {
                    sv == _sv
                } else {
                    false
                }
            })
            .unwrap()
            .as_eph()
            .unwrap()
            .2;

        let nav_data = SBASNavData::from(ephemeris);
        let expected = SBASNavData {
            clock_bias: 0.000000000000E+00,
            clock_drift: 0.000000000000E+00,
            // time of message
            tom: 2.592340000000E+05,
            x: -3.389392800000E+04,
            vel_x: 0.000000000000E+00,
            accel_x: 0.000000000000E+00,
            health: 6.300000000000E+01,
            y: 2.508018800000E+04,
            vel_y: 0.000000000000E+00,
            accel_y: 0.000000000000E+00,
            ura: 3.276700000000E+04,
            z: 0.000000000000E+00,
            vel_z: 0.000000000000E+00,
            accel_z: 0.000000000000E+00,
            // issue of data navigation
            iodn: 9.700000000000E+01,
        };
        assert_eq!(nav_data, expected);
    }
}

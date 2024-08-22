use std::{
    collections::HashMap,
    io::{Error, ErrorKind},
    path::PathBuf,
    vec,
};

use rinex::{
    observation::ObservationData,
    prelude::{Constellation, Observable, SV},
    Rinex,
};

pub(crate) struct ObsDataProvider {
    obs_file: Rinex,
    index: usize,
}

impl ObsDataProvider {
    pub(crate) fn new(filename: PathBuf) -> Result<Self, rinex::Error> {
        let obs_file = Rinex::from_file(
            filename
                .to_str()
                .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "Invalid filename"))?,
        )?;
        Ok(Self { obs_file, index: 0 })
    }

    fn sv_to_u16(sv: &SV) -> u16 {
        let leading: u16 = match sv.constellation {
            Constellation::GPS => 1,
            Constellation::Glonass => 2,
            Constellation::Galileo => 3,
            Constellation::BeiDou => 4,
            Constellation::QZSS => 5,
            Constellation::IRNSS => 6,
            _ => 7,
        };
        leading * 100 + sv.prn as u16
    }

    fn gps_data(observations: &HashMap<Observable, ObservationData>) -> Vec<f64> {
        // implementation of the gps_data method
        for (observable, observation_data) in observations {
            match observable {
                Observable::Phase(phase) => {
                    // implementation of the C1C observable
                }
                Observable::Doppler(doppler) => {
                    // implementation of the L1C observable
                }
                Observable::PseudoRange(pseudo_range) => {
                    // implementation of the P1C observable
                }
                Observable::SSI(ssi) => {}
                Observable::ChannelNumber(channel_number) => {}
                _ => {}
            }
        }
        vec![0.0; 24]
    }

    fn glonass_data(observations: &HashMap<Observable, ObservationData>) -> Vec<f64> {
        // implementation of the glonass_data method
        vec![0.0; 24]
    }

    fn galileo_data(observations: &HashMap<Observable, ObservationData>) -> Vec<f64> {
        // implementation of the galileo_data method
        vec![0.0; 24]
    }

    fn beidou_data(observations: &HashMap<Observable, ObservationData>) -> Vec<f64> {
        // implementation of the beidou_data method
        vec![0.0; 24]
    }

    fn qzss_data(observations: &HashMap<Observable, ObservationData>) -> Vec<f64> {
        // implementation of the qzss_data method
        vec![0.0; 24]
    }

    fn irnss_data(observations: &HashMap<Observable, ObservationData>) -> Vec<f64> {
        // implementation of the irnss_data method
        vec![0.0; 24]
    }

    fn sbas_data(observations: &HashMap<Observable, ObservationData>) -> Vec<f64> {
        // implementation of the irnss_data method
        vec![0.0; 24]
    }
}

impl Iterator for ObsDataProvider {
    type Item = (u16, Vec<f64>);

    fn next(&mut self) -> Option<Self::Item> {
        let ((epoch, flag), (_, vehicles)) = self.obs_file.observation().nth(self.index)?;
        if flag.is_ok() {
            for (sv, observations) in vehicles {
                let sv_id = ObsDataProvider::sv_to_u16(sv);
                let data = match sv.constellation {
                    Constellation::GPS => ObsDataProvider::gps_data(observations),
                    Constellation::Glonass => ObsDataProvider::glonass_data(observations),
                    Constellation::Galileo => ObsDataProvider::galileo_data(observations),
                    Constellation::BeiDou => ObsDataProvider::beidou_data(observations),
                    Constellation::QZSS => ObsDataProvider::qzss_data(observations),
                    Constellation::IRNSS => ObsDataProvider::irnss_data(observations),
                    _ => ObsDataProvider::sbas_data(observations),
                };
            }
        } else {
        }
        self.index += 1;

        Some((0_u16, vec![0.0; 24]))
    }
}

#[cfg(test)]
mod tests {
    use rinex::prelude::Epoch;

    use super::*;

    #[test]
    fn test_sv_to_u16() {
        let sv_gps = SV {
            constellation: Constellation::GPS,
            prn: 1,
        };
        assert_eq!(ObsDataProvider::sv_to_u16(&sv_gps), 101);

        let sv_galileo = SV {
            constellation: Constellation::Galileo,
            prn: 2,
        };
        assert_eq!(ObsDataProvider::sv_to_u16(&sv_galileo), 302);

        // Add more test cases for other constellations
        let sv_nsas = SV {
            constellation: Constellation::NSAS,
            prn: 24,
        };

        assert_eq!(ObsDataProvider::sv_to_u16(&sv_nsas), 724);

        let sv_compass = SV {
            constellation: Constellation::BeiDou,
            prn: 28,
        };
        assert_eq!(ObsDataProvider::sv_to_u16(&sv_compass), 428);

        let sv_irnss = SV {
            constellation: Constellation::IRNSS,
            prn: 7,
        };
        assert_eq!(ObsDataProvider::sv_to_u16(&sv_irnss), 607);

        let span = SV {
            constellation: Constellation::SPAN,
            prn: 9,
        };
        assert_eq!(ObsDataProvider::sv_to_u16(&span), 709);
    }

    #[test]
    fn test_epoch_time_gpst() {
        let epoch = Epoch::from_gregorian_hms(2020, 1, 1, 0, 0, 0, TimeScale::GPST);

        assert_eq!(
            epoch.to_gregorian_str(TimeScale::UTC),
            "2019-12-31T23:59:23 UTC"
        );

        assert_eq!(
            epoch.to_gregorian_str(TimeScale::GPST),
            "2020-01-01T00:00:00 GPST"
        );
    }

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

    // Add more tests for other methods and functionalities of ObsDataProvider
}

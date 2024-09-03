use std::{
    collections::HashMap,
    io::{Error, ErrorKind},
    path::PathBuf,
    vec,
};

use rinex::{
    observation::ObservationData,
    prelude::{Constellation, Epoch, Observable, TimeScale, SV},
    Rinex,
};

use crate::tna_fields::{
    BEIDOU_FIELDS, GALILEO_FIELDS, GLONASS_FIELDS, GPS_FIELDS, IRNSS_FIELDS, MAX_FIELDS_COUNT,
    QZSS_FIELDS, SBAS_FIELDS,
};

/// Maximum number of fields in a RINEX observation record
const DATA_VEC_SIZE: usize = MAX_FIELDS_COUNT * 2 + 6;

#[derive(Clone)]
pub(crate) struct ObsDataProvider {
    obs_file: Rinex,
    index: usize,
    inner_index: usize,
    gps_fields: HashMap<&'static str, usize>,
    glonass_fields: HashMap<&'static str, usize>,
    galileo_fields: HashMap<&'static str, usize>,
    beidou_fields: HashMap<&'static str, usize>,
    qzss_fields: HashMap<&'static str, usize>,
    irnss_fields: HashMap<&'static str, usize>,
    sbas_fields: HashMap<&'static str, usize>,
}

impl ObsDataProvider {
    /// Converts a vector of strings to a hash map which maps the string to its index*2+4 in the vector.
    fn vec_to_hash(vec: &Vec<&'static str>) -> HashMap<&'static str, usize> {
        vec.iter()
            .cloned()
            .enumerate()
            .map(|(i, s)| (s, i * 2 + 6))
            .collect()
    }

    pub(crate) fn new(filename: PathBuf) -> Result<Self, rinex::Error> {
        let obs_file = Rinex::from_file(
            filename
                .to_str()
                .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "Invalid filename"))?,
        )
        .map_err(|e| rinex::Error::from(e))?; // Handle the error returned by Rinex::from_file

        Ok(Self {
            obs_file,
            index: 0,
            inner_index: 0,
            gps_fields: Self::vec_to_hash(&GPS_FIELDS),
            glonass_fields: Self::vec_to_hash(&GLONASS_FIELDS),
            galileo_fields: Self::vec_to_hash(&GALILEO_FIELDS),
            beidou_fields: Self::vec_to_hash(&BEIDOU_FIELDS),
            qzss_fields: Self::vec_to_hash(&QZSS_FIELDS),
            irnss_fields: Self::vec_to_hash(&IRNSS_FIELDS),
            sbas_fields: Self::vec_to_hash(&SBAS_FIELDS),
        })
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

    #[inline]
    fn get_observable_field_name(observable: &Observable) -> Option<&str> {
        match observable {
            Observable::Phase(name) => Some(name),
            Observable::Doppler(name) => Some(name),
            Observable::SSI(name) => Some(name),
            Observable::PseudoRange(name) => Some(name),
            Observable::ChannelNumber(name) => Some(name),
            _ => None,
        }
    }

    /// Converts the observation data to a vector of f64 values.
    fn get_data(
        &self,
        observations: &HashMap<Observable, ObservationData>,
        fields: &HashMap<&str, usize>,
    ) -> Vec<f64> {
        let mut data = vec![0.0; DATA_VEC_SIZE];
        // implementation of the gps_data method
        for (observable, observation_data) in observations {
            let field_name = Self::get_observable_field_name(observable);
            if let Some(field_name) = field_name {
                if let Some(index) = fields.get(field_name) {
                    data[*index] = observation_data.obs;
                    if let Some(snr) = observation_data.snr {
                        data[*index + 1] = f64::from(snr);
                    }
                }
            }
        }
        data
    }

    #[inline(always)]
    fn gps_data(&self, observations: &HashMap<Observable, ObservationData>) -> Vec<f64> {
        self.get_data(observations, &self.gps_fields)
    }

    #[inline(always)]
    fn glonass_data(&self, observations: &HashMap<Observable, ObservationData>) -> Vec<f64> {
        self.get_data(observations, &self.glonass_fields)
    }

    #[inline(always)]
    fn galileo_data(&self, observations: &HashMap<Observable, ObservationData>) -> Vec<f64> {
        self.get_data(observations, &self.galileo_fields)
    }

    #[inline(always)]
    fn beidou_data(&self, observations: &HashMap<Observable, ObservationData>) -> Vec<f64> {
        self.get_data(observations, &self.beidou_fields)
    }

    #[inline(always)]
    fn qzss_data(&self, observations: &HashMap<Observable, ObservationData>) -> Vec<f64> {
        self.get_data(observations, &self.qzss_fields)
    }

    #[inline(always)]
    fn irnss_data(&self, observations: &HashMap<Observable, ObservationData>) -> Vec<f64> {
        self.get_data(observations, &self.irnss_fields)
    }
    #[inline(always)]
    fn sbas_data(&self, observations: &HashMap<Observable, ObservationData>) -> Vec<f64> {
        self.get_data(observations, &self.sbas_fields)
    }
}

use lazy_static::lazy_static;

lazy_static! {
    /// The epoch time at J2000 in GPST seconds
    static ref EPOCH_TIME_AT_J2000: f64 =
        Epoch::from_gregorian(2000, 1, 1, 0, 0, 0, 0, TimeScale::GPST).to_gpst_seconds();
}

impl Iterator for ObsDataProvider {
    type Item = (SV, Epoch, Vec<f64>);

    /// Returns the next observation data in the RINEX file.
    /// The first element of the tuple is the epoch, the second is the SV, and the third is the observation data.
    /// The first byte of the observation data is the satellite id which is converted from the SV by `sv_to_u16`.
    /// The second byte of the observation data is the epoch time divided by J2000.
    /// The next 3 bytes of the observation data is the ground position in ECEF coordinates.
    fn next(&mut self) -> Option<Self::Item> {
        let ((epoch, flag), (_, vehicles)) = self.obs_file.observation().nth(self.index)?;
        if flag.is_ok() {
            if let Some((sv, observations)) = vehicles.iter().nth(self.inner_index) {
                let sv_id = Self::sv_to_u16(sv);
                let mut data = match sv.constellation {
                    Constellation::GPS => self.gps_data(observations),
                    Constellation::Glonass => self.glonass_data(observations),
                    Constellation::Galileo => self.galileo_data(observations),
                    Constellation::BeiDou => self.beidou_data(observations),
                    Constellation::QZSS => self.qzss_data(observations),
                    Constellation::IRNSS => self.irnss_data(observations),
                    _ => self.sbas_data(observations),
                };
                data[0] = f64::from(sv_id);
                data[1] = epoch.to_gpst_seconds() / *EPOCH_TIME_AT_J2000;
                if let Some(ground_position) = self.obs_file.header.ground_position {
                    data[2] = ground_position.to_ecef_wgs84().0;
                    data[3] = ground_position.to_ecef_wgs84().1;
                    data[4] = ground_position.to_ecef_wgs84().2;
                }
                // move to the next vehicle
                self.inner_index += 1;
                Some((sv.clone(), epoch.clone(), data))
            } else {
                // move to the next epoch if there are no more vehicles in this epoch
                self.index += 1;
                self.inner_index = 0;
                self.next()
            }
        } else {
            // move to the next epoch if this epoch is not valid
            self.index += 1;
            self.inner_index = 0;
            self.next()
        }
    }
}

#[cfg(test)]
mod tests {
    use rinex::{
        observation::LliFlags,
        prelude::{Epoch, TimeScale},
    };

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
}

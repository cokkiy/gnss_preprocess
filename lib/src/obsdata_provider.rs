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

use crate::{
    common::sv_to_u16,
    tna_fields::{
        BEIDOU_FIELDS, GALILEO_FIELDS, GLONASS_FIELDS, GPS_FIELDS, IRNSS_FIELDS, MAX_FIELDS_COUNT,
        QZSS_FIELDS, SBAS_FIELDS,
    },
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

#[allow(dead_code)]
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

    /// Retrieves all space vehicles (SV) from the observation file.
    ///
    /// # Returns
    ///
    /// A vector containing all the space vehicles (SV) present in the observation file.
    ///
    /// # Example
    ///
    /// ```
    /// let obs_data_provider = ObsDataProvider::new(filename).unwrap();
    /// let all_sv = obs_data_provider.get_all_sv();
    /// for sv in all_sv {
    ///     println!("{:?}", sv);
    /// }
    /// ```
    pub(crate) fn get_all_sv(&self) -> Vec<SV> {
        self.obs_file
            .observation()
            .map(|((_, _), (_, vehicles))| vehicles.keys().cloned())
            .flatten()
            .collect()
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
                let sv_id = sv_to_u16(sv);
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
mod tests;

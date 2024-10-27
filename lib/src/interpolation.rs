mod beidou_nav_data_interpolation;
mod galileo_nav_data_interpolation;
mod glonass_nav_data_interpolation;
mod gps_nav_interpolation;
mod irnss_nav_data_interpolation;
mod qzss_nav_data_interpolation;
mod sbas_nav_data_interpolation;

use hifitime::Epoch;

use crate::nav_data::{
    BeiDouNavData, GPSNavData, GalileoNavData, GlonassNavData, IRNSSNavData, NavData, QZSSNavData,
    SBASNavData,
};

/// Defines the interpolation trait
pub trait Interpolation {
    /// Defines the output type
    type Output;
    /// Defines the interpolate method
    fn interpolate(&self, epoch: &Epoch) -> Self::Output;
}

impl Interpolation for Vec<NavData> {
    type Output = NavData;

    fn interpolate(&self, epoch: &Epoch) -> Self::Output {
        if self.is_empty() {
            panic!("Cannot interpolate an empty vector");
        }
        self.iter()
            .all(|nav_data| nav_data.is_gps_nav_data())
            .then(|| {
                let gps_data: Vec<(&Epoch, &GPSNavData)> = self
                    .iter()
                    .map(|nav: &NavData| Into::<Option<(&Epoch, &GPSNavData)>>::into(nav).unwrap())
                    .collect();
                NavData::GPSNavData((*epoch, gps_data.interpolate(epoch)))
            })
            .or_else(|| {
                self.iter()
                    .all(|nav_data| nav_data.is_beidou_nav_data())
                    .then(|| {
                        let beidou_data: Vec<_> = self
                            .iter()
                            .map(|nav: &NavData| {
                                Into::<Option<(&Epoch, &BeiDouNavData)>>::into(nav).unwrap()
                            })
                            .collect();
                        NavData::BeiDouNavData((*epoch, beidou_data.interpolate(epoch)))
                    })
            })
            .or_else(|| {
                self.iter()
                    .all(|nav_data| nav_data.is_galileo_nav_data())
                    .then(|| {
                        let galileo_data: Vec<_> = self
                            .iter()
                            .map(|nav| {
                                Into::<Option<(&Epoch, &GalileoNavData)>>::into(nav).unwrap()
                            })
                            .collect();
                        NavData::GalileoNavData((*epoch, galileo_data.interpolate(epoch)))
                    })
            })
            .or_else(|| {
                self.iter()
                    .all(|nav_data| nav_data.is_glonass_nav_data())
                    .then(|| {
                        let glonass_data: Vec<_> = self
                            .iter()
                            .map(|nav| {
                                Into::<Option<(&Epoch, &GlonassNavData)>>::into(nav).unwrap()
                            })
                            .collect();
                        NavData::GlonassNavData((*epoch, glonass_data.interpolate(epoch)))
                    })
            })
            .or_else(|| {
                self.iter()
                    .all(|nav_data| nav_data.is_qzss_nav_data())
                    .then(|| {
                        let qzss_data: Vec<_> = self
                            .iter()
                            .map(|nav| Into::<Option<(&Epoch, &QZSSNavData)>>::into(nav).unwrap())
                            .collect();
                        NavData::QZSSNavData((*epoch, qzss_data.interpolate(epoch)))
                    })
            })
            .or_else(|| {
                self.iter()
                    .all(|nav_data| nav_data.is_irnss_nav_data())
                    .then(|| {
                        let irnss_data: Vec<_> = self
                            .iter()
                            .map(|nav| Into::<Option<(&Epoch, &IRNSSNavData)>>::into(nav).unwrap())
                            .collect();
                        NavData::IRNSSNavData((*epoch, irnss_data.interpolate(epoch)))
                    })
            })
            .or_else(|| {
                self.iter()
                    .all(|nav_data| nav_data.is_sbas_nav_data())
                    .then(|| {
                        let sbas_data: Vec<_> = self
                            .iter()
                            .map(|nav| Into::<Option<(&Epoch, &SBASNavData)>>::into(nav).unwrap())
                            .collect();
                        NavData::SBASNavData((*epoch, sbas_data.interpolate(epoch)))
                    })
            })
            .unwrap()
    }
}

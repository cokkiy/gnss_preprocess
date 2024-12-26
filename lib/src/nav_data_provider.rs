use hifitime::Epoch;
use rinex::prelude::SV;

use crate::{
    gnss_epoch_data::GnssEpochData, interpolation::Interpolation, nav_data::NavData,
    nearest_points_finder::NearestPointsFinder,
};

/// NavDataProvider provide GNSS satellite vehicle
/// navigation data to the application.
struct NavDataProvider {
    base_path: String,
    nearest_points_finder: Box<dyn NearestPointsFinder>,
}
impl NavDataProvider {
    fn new(base_path: String, points_finder: Box<dyn NearestPointsFinder>) -> Self {
        Self {
            base_path,
            nearest_points_finder: points_finder,
        }
    }

    pub fn sample(&self, epoch_data: &GnssEpochData) -> Vec<NavData> {
        let epoch = epoch_data.get_epoch();
        epoch_data
            .iter()
            .map_while(|sv_data| {
                let sv = sv_data.get_sv();
                let nav_data = self.get_nav_data(&sv, &epoch);
                nav_data
            })
            .collect()
    }

    fn get_nav_data(&self, sv: &SV, epoch: &Epoch) -> Option<NavData> {
        let nearest_points = self.nearest_points_finder.find_nearest_points(sv, epoch);
        if let Some(points) = nearest_points {
            let nav_data = points.interpolate(epoch);
            Some(nav_data)
        } else {
            None
        }
    }
}

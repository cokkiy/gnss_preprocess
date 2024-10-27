use hifitime::Epoch;
use itertools::Itertools;
use rinex::{prelude::SV, Rinex};

use crate::{gnss_epoch_data::GnssEpochData, interpolation::Interpolation, nav_data::NavData};

/// NavDataProvider provide GNSS satellite vehicle
/// navigation data to the application.
struct NavDataProvider {
    base_path: String,
    cur_rinex: Option<Rinex>,
    nxt_rinex: Option<Rinex>,
}
impl NavDataProvider {
    fn new(base_path: String) -> Self {
        Self {
            base_path,
            cur_rinex: None,
            nxt_rinex: None,
        }
    }

    pub fn sample(&self, epoch_data: &GnssEpochData) -> Vec<NavData> {
        let epoch = epoch_data.get_epoch();
        epoch_data
            .iter()
            .map(|sv_data| {
                let sv = sv_data.get_sv();
                let nav_data = self.get_nav_data(&epoch, &sv);
                nav_data
            })
            .collect()
    }

    fn get_nav_data(&self, epoch: &Epoch, sv: &SV) -> NavData {
        if self.check_is_cur_rinex(epoch) {
            if let Some(ref nav_rinex) = self.cur_rinex {
                let nav_data = self.find_nearest_five_points(epoch, nav_rinex);
                nav_data.interpolate(epoch)
            }
        } else if self.check_is_nxt_rinex(epoch) {
            if let Some(ref nav_rinex) = self.nxt_rinex {
                let nav_data = nav_rinex.get_nav_data(sv);
                return nav_data;
            }
        }
    }

    fn find_nearest_five_points(&self, epoch: &Epoch, rinex: &Rinex) -> Vec<NavData> {
        let mut epoch_list = Vec::new();
        if let Some(ref nav_rinex) = self.cur_rinex {
            let epoch_list_cur = nav_rinex.epoch().collect_vec();
            epoch_list.extend(epoch_list_cur);
        }
        if let Some(ref nav_rinex) = self.nxt_rinex {
            let epoch_list_nxt = nav_rinex.epoch().collect_vec();
            epoch_list.extend(epoch_list_nxt);
        }
        epoch_list.sort_by(|a, b| {
            let a_diff = a - epoch;
            let b_diff = b - epoch;
            a_diff.partial_cmp(&b_diff).unwrap()
        });
        epoch_list.into_iter().take(5).collect()
    }

    fn check_is_cur_rinex(&self, epoch: &Epoch) -> bool {
        if let Some(ref nav_rinex) = self.cur_rinex {
            if let Some(first_epoch) = nav_rinex.epoch().nth(0) {
                let epoch_year = first_epoch.year();
                let epoch_day_of_year = first_epoch.day_of_year();
                if epoch.year() == epoch_year && epoch.day_of_year() == epoch_day_of_year {
                    return true;
                }
            }
        }
        return false;
    }
}

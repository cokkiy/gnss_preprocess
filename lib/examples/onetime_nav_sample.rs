use std::str::FromStr;

use gnss_preprocess::NavDataProvider;
use rinex::prelude::{Epoch, TimeScale, SV};
fn main() {
    let nav_files_path = "/mnt/d/GNSS_Data/Data/Nav";
    let mut navdata_store = NavDataProvider::new(nav_files_path);
    let sv = SV::from_str("S34").unwrap();
    let epoch = Epoch::from_gregorian(2023, 4, 10, 22, 10, 0, 0, TimeScale::GPST);
    let results = navdata_store.sample(23, 100, &sv, &epoch);
    println!("{:?}", results);
}

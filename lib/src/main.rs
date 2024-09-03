mod constellation_keys;
mod navdata_interpolation;
mod navdata_provider;
mod navigation_data;

use gnss_preprocess::GNSSDataProvider;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let nav_files_path = "/mnt/d/GNSS_Data/Data";
    let mut gnss_data_provider = GNSSDataProvider::new(nav_files_path, None);
    let mut iter = gnss_data_provider.train_iter();
    for _ in 0..10 {
        let data = iter.next().unwrap();
        println!("{:?}", data);
    }

    Ok(())
}

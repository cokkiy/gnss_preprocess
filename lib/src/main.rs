mod constellation_keys;
mod navdata_interpolation;
mod navdata_provider;
mod navigation_data;

use navdata_provider::NavDataProvider;
use rinex::prelude::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let nav_files_path = "/mnt/d/GNSS_Data/Data/Nav";
    //let mut nav_data_store = NavDataProvider::new(nav_files_path);
    // 读取观测文件
    let observation_file_path = "/mnt/d/GNSS_Data/Data/Obs/2020/001/daily/abmf0010.20o"; // 示例观测文件路径
    let obs = Rinex::from_file(observation_file_path)?;
    //println!("{:?}", obs.header.obs.unwrap().time_of_first_obs.unwrap());
    if let Some(obs) = obs.header.obs {
        println!("{:?}", obs.time_of_first_obs.unwrap());
        obs.codes
            .iter()
            .for_each(|(c, v)| println!("{} {:?}", c, v));
    }
    // 按行读取观测文件，进行插值和对齐
    // for ((epoch, _), (_, vehicles)) in obs.observation() {
    //     for (sv, observations) in vehicles {
    //         let timestamp = epoch.to_gregorian_utc();

    //         let nav_data = nav_data_store.sample(20, 1, sv, epoch);

    //         println!(
    //             "{} {}-{}-{}T{}:{}:{}.{} {:?} {:?}",
    //             sv,
    //             timestamp.0,
    //             timestamp.1,
    //             timestamp.2,
    //             timestamp.3,
    //             timestamp.4,
    //             timestamp.5,
    //             timestamp.6,
    //             nav_data,
    //             observations
    //         );
    //     }
    // }

    Ok(())
}

use chrono::{NaiveTime, Timelike};
use rinex::prelude::*;
use std::{
    collections::{BTreeMap, HashMap},
    error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    // 读取导航文件
    let navigation_file_path = "/mnt/d/GNSS_Data/Data/Nav/2020/brdm0010.20p"; // 示例导航文件路径
    let nav = Rinex::from_file(navigation_file_path)?;

    // 读取观测文件
    //let observation_file_path = "observation.21o"; // 示例观测文件路径
    //let obs = Rinex::from_file(observation_file_path)?;

    // 提取导航中的卫星轨迹信息
    let mut multi_navigation_data: HashMap<SV, BTreeMap<u64, &rinex::navigation::Ephemeris>> =
        HashMap::new();

    for (epoch, nav_frames) in nav.navigation() {
        let timestamp = epoch.to_gpst_nanoseconds()?;
        for frame in nav_frames {
            if let Some((_, sv, eph)) = frame.as_eph() {
                // 假设pos是我们感兴趣的导航数据
                if let Some(data) = multi_navigation_data.get_mut(&sv) {
                    data.insert(timestamp, eph);
                } else {
                    let mut navigation_data = BTreeMap::new();
                    navigation_data.insert(timestamp, eph);
                    multi_navigation_data.insert(sv, navigation_data);
                }
            }
        }
    }
    let sv_key = SV::new(Constellation::GPS, 01);
    for (timestamp, eph) in &multi_navigation_data[&sv_key] {
        println!("{} {:?}", timestamp, eph);
    }

    // let mut i = 0;
    // for (k, v) in multi_navigation_data {
    //     println!("{:?} {:?}", k, v);
    //     i = i + 1;
    //     if i > 2 {
    //         break;
    //     }
    // }

    // 按行读取观测文件，进行插值和对齐
    // if let Some(observation_records) = obs.observation() {
    //     for (epoch, obs_data) in observation_records {
    //         let time = epoch.time.time();
    //         let obs_value = obs_data.values().next().unwrap(); // 示例获取观测值

    //         // 查找比当前时间早和晚的导航数据点
    //         let mut points = Vec::new();
    //         for (&timestamp, &position) in &navigation_data {
    //             points.push((timestamp, position));
    //         }

    //         // 找到最近的四个点进行拉格朗日插值
    //         let mut closest_points = find_closest_points(&points, time, 4);

    //         // 进行拉格朗日插值
    //         let position = lagrange_interpolation(time, &closest_points);

    //         // 打印输出对齐后的数据
    //         println!(
    //             "Time: {}, Interpolated Position: {:?}, Observation Data: {}",
    //             time.format("%H:%M:%S").to_string(),
    //             position,
    //             obs_value
    //         );
    //     }
    // }

    Ok(())
}

// 拉格朗日插值函数
fn lagrange_interpolation(target_time: NaiveTime, points: &[(NaiveTime, [f64; 3])]) -> [f64; 3] {
    let mut result = [0.0, 0.0, 0.0];
    for i in 0..3 {
        for (j, &(time_j, value_j)) in points.iter().enumerate() {
            let mut term = value_j[i];
            for (k, &(time_k, _)) in points.iter().enumerate() {
                if j != k {
                    let time_j_seconds = time_j.num_seconds_from_midnight() as f64;
                    let time_k_seconds = time_k.num_seconds_from_midnight() as f64;
                    let target_seconds = target_time.num_seconds_from_midnight() as f64;
                    term *= (target_seconds - time_k_seconds) / (time_j_seconds - time_k_seconds);
                }
            }
            result[i] += term;
        }
    }
    result
}

// 查找最近的n个点
fn find_closest_points(
    data: &[(NaiveTime, [f64; 3])],
    target_time: NaiveTime,
    n: usize,
) -> Vec<(NaiveTime, [f64; 3])> {
    let mut closest_points = Vec::new();
    for &(time, pos) in data {
        closest_points.push((time, pos));
        if closest_points.len() == n {
            break;
        }
    }
    closest_points
}

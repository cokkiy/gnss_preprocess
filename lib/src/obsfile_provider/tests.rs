use std::collections::HashMap;

use super::*;

#[test]
fn test_get_total_days() {
    let obs_data_tree = HashMap::from([
        (
            20,
            HashMap::from([
                (1, vec!["a", "b", "c"]),
                (2, vec!["d", "e", "f"]),
                (3, vec!["g", "h", "i"]),
            ]),
        ),
        (
            21,
            HashMap::from([(1, vec!["a", "b", "c"]), (2, vec!["d", "e", "f"])]),
        ),
    ]);
    let obs_data_provider = ObsFileProvider::from_data(obs_data_tree);
    assert_eq!(obs_data_provider.get_day_numbers(), 5);
}

#[test]
fn test_get_total() {
    let obs_data_tree = HashMap::from([
        (
            20,
            HashMap::from([
                (1, vec!["a", "b", "c"]),
                (2, vec!["d", "e", "f"]),
                (3, vec!["g", "h", "i"]),
            ]),
        ),
        (
            21,
            HashMap::from([
                (1, vec!["a", "b", "c"]),
                (2, vec!["d", "e"]),
                (3, vec!["g", "h", "i", "o"]),
            ]),
        ),
    ]);
    let obs_data_provider = ObsFileProvider::from_data(obs_data_tree);
    assert_eq!(obs_data_provider.get_total_count(), 18);
}

#[test]
fn test_build_obs_tree() {
    let obs_files_path = "/mnt/d/GNSS_Data/Data/Obs";
    let obs_data_tree = build_obs_tree(obs_files_path);

    // Assert that the returned tree is not empty
    assert_ne!(!obs_data_tree.get_obs_files().count(), 0);

    // Assert that the tree contains the expected years
    assert!(obs_data_tree.get_obs_files().any(|f| f.starts_with("2020")));
    assert!(obs_data_tree.get_obs_files().any(|f| f.starts_with("2021")));

    // Assert that the tree contains the expected files
    assert!(obs_data_tree
        .get_obs_files()
        .any(|f| f.starts_with("2020/001/daily")));
    assert!(obs_data_tree
        .get_obs_files()
        .any(|f| f.starts_with("2020/002/daily")));
    assert!(obs_data_tree
        .get_obs_files()
        .any(|f| f.starts_with("2020/003/daily")));

    assert!(obs_data_tree
        .get_obs_files()
        .any(|f| f.starts_with("2021/266/daily")));
    assert!(obs_data_tree
        .get_obs_files()
        .any(|f| f.starts_with("2021/284/daily")));
}

#[test]
fn test_obs_file_provider_find_next_file() {
    let obs_files_path = "/mnt/d/GNSS_Data/Data/Obs";
    let obs_data_tree = build_obs_tree(obs_files_path);
    let p = obs_data_tree.find_next_file("abmf", 2020, 1);
    assert!(p.is_some());
    assert_eq!(p.unwrap().to_str().unwrap(), "2020/002/daily/abmf0020.20o");
}

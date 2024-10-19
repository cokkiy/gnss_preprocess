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

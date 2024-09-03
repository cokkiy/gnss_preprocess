#[cfg(test)]
use std::collections::HashMap;
use std::path::PathBuf;

use crate::obs_files_tree::{ObsFilesInDay, ObsFilesInYear, ObsFilesTree};

/// `ObsFileProvider` is a struct that represents a provider of observation data file.
/// With this struct, you can get the total count of observation files, the number of unique days,
/// and split the observation files into two parts based on a given percentage to get training and testing files.
/// The struct also provides an iterator over the observation file paths. Using the iterator, you can get the year,
/// day of the year, and the corresponding observation file path.
#[derive(Clone)]
#[allow(dead_code)]
pub struct ObsFileProvider {
    obs_files_path: String,
    obs_files_tree: ObsFilesTree,
}

#[allow(dead_code)]
impl ObsFileProvider {
    /// Creates a new `ObsFileProvider` instance.
    ///
    /// # Arguments
    ///
    /// * `obs_files_path` - The path to the observation files.
    ///
    /// # Returns
    ///
    /// A new `ObsFileProvider` instance.
    pub fn new(obs_files_path: &str) -> Self {
        Self {
            obs_files_path: obs_files_path.to_string(),
            obs_files_tree: build_obs_tree(obs_files_path),
        }
    }

    /// Returns the total count of observation files in the `ObsFileProvider`.
    ///
    /// # Returns
    ///
    /// The total count of observation files.
    pub fn get_total_count(&self) -> usize {
        self.obs_files_tree.get_obs_files().count()
    }

    /// Returns the number of unique days represented in the `ObsFileProvider`.
    ///
    /// # Returns
    ///
    /// The number of unique days.
    pub fn get_day_numbers(&self) -> usize {
        self.obs_files_tree.get_day_numbers()
    }

    /// Splits the `ObsFileProvider` into two instances based on the given percentage
    /// which count all days in the `ObsFileProvider` and split them into two parts.
    ///
    /// # Arguments
    ///
    /// * `percent` - The percentage at which to split the `ObsFileProvider`.
    ///
    /// # Returns
    ///
    /// A tuple containing two `ObsFileProvider` instances, where the first instance contains
    /// the left portion of the split based on days and the second instance contains the right portion of the split.
    pub fn split_by_percent(&self, percent: u8) -> (Self, Self) {
        let (left, right) = self.obs_files_tree.split_by_percent(percent);
        (
            Self {
                obs_files_path: self.obs_files_path.clone(),
                obs_files_tree: left,
            },
            Self {
                obs_files_path: self.obs_files_path.clone(),
                obs_files_tree: right,
            },
        )
    }

    /// Returns an iterator over the observation file paths in the `ObsFileProvider`.
    ///
    /// # Returns
    ///
    /// An iterator over the observation file paths, which yields tuples containing
    ///  the year, day of the year and the corresponding observation file path.
    pub fn iter(&self) -> impl Iterator<Item = (u16, u16, PathBuf)> + '_ {
        self.obs_files_tree.get_files()
    }

    #[cfg(test)]
    /// from_data is used for testing purposes.
    fn from_data(obs_data: HashMap<u16, HashMap<u16, Vec<&'static str>>>) -> Self {
        Self {
            obs_files_path: "".to_string(),
            obs_files_tree: ObsFilesTree::from_data(obs_data),
        }
    }
}

/// Builds an observation files tree from the given observation files path.
fn build_obs_tree(obs_files_path: &str) -> ObsFilesTree {
    let mut obs_data_tree = ObsFilesTree::new();
    if let Ok(root_dir) = std::fs::read_dir(obs_files_path) {
        root_dir
            .map(|year_entries| year_entries.unwrap())
            .for_each(|entry| {
                let year = entry.file_name().to_string_lossy().parse::<u16>().unwrap();
                let mut obs_files_in_year = ObsFilesInYear::create_empty(year);
                if let Ok(day_of_years) = std::fs::read_dir(entry.path()) {
                    day_of_years
                        .map(|entries| entries.unwrap())
                        .for_each(|day_entry| {
                            let day_of_year = day_entry
                                .file_name()
                                .to_string_lossy()
                                .parse::<u16>()
                                .unwrap();
                            let mut obs_files_in_days = Vec::new();
                            if let Ok(files) = std::fs::read_dir(day_entry.path().join("daily")) {
                                files.map(|file| file.unwrap()).for_each(|file| {
                                    obs_files_in_days
                                        .push(file.file_name().to_string_lossy().to_string());
                                });
                            }
                            let obs_file_item = ObsFilesInDay::new(day_of_year, obs_files_in_days);
                            obs_files_in_year.add_item(obs_file_item);
                        });
                }
                obs_data_tree.add_item(obs_files_in_year);
            });
    };

    obs_data_tree
}

#[cfg(test)]
mod tests {

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
}

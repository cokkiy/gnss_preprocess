#[cfg(test)]
use std::collections::HashMap;
use std::path::PathBuf;

use crate::obs_files_tree::ObsFilesTree;

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
            obs_files_tree: ObsFilesTree::create_obs_tree(obs_files_path),
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

    /// Returns the next day observation file path for the given station name.
    /// If the observation file is not found in the next day of given year and day of the year,
    /// it returns `None`.
    pub fn find_next_file(&self, name: &str, year: u16, day_of_year: u16) -> Option<PathBuf> {
        self.obs_files_tree.find_next_file(name, year, day_of_year)
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

#[cfg(test)]
mod tests;

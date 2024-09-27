/// This module contains the implementation of the `ObsFilesTree` struct and related types.
#[cfg(test)]
use std::collections::HashMap;
use std::path::PathBuf;

use crate::common::get_next_day;

/// The `ObsFilesInDay` struct contains the day of year and a list of observation file names
/// which observed in that day.
/// It also provides an iterator to iterate over the observation file paths.
/// # Fields
///
/// - `day_of_year`: The day of the year.
/// - `obs_files`: A list of observation file names.
///
/// # Examples
///
/// ```
/// use std::path::PathBuf;
///
/// let obs_files = vec!["file1.obs".to_string(), "file2.obs".to_string()];
/// let obs_file_item = ObsFilesInDay::new(123, obs_files);
///
/// let mut iter = obs_file_item.iter();
/// assert_eq!(iter.next(), Some(PathBuf::from("123/daily/file1.obs")));
/// assert_eq!(iter.next(), Some(PathBuf::from("123/daily/file2.obs")));
/// assert_eq!(iter.next(), None);
/// ```
#[derive(Clone, Eq, Debug)]
pub(crate) struct ObsFilesInDay {
    day_of_year: u16,
    obs_files: Vec<String>,
}

impl PartialEq for ObsFilesInDay {
    fn eq(&self, other: &Self) -> bool {
        self.day_of_year == other.day_of_year
    }
}

impl ObsFilesInDay {
    /// Creates a new `ObsFilesInDay` with the specified `day_of_year` and `obs_files`.
    ///
    /// # Arguments
    ///
    /// * `day_of_year` - The day of the year.
    /// * `obs_files` - The vector of observation files.
    ///
    /// # Returns
    ///
    /// A new `ObsFilesInDay` instance.
    ///
    /// # Example
    ///
    /// ```
    /// use gnss_preprocess::ObsFilesInDay;
    ///
    /// let day_of_year = 123;
    /// let obs_files = vec!["file1.obs".to_string(), "file2.obs".to_string()];
    /// let obs_file_item = ObsFilesInDay::new(day_of_year, obs_files);
    /// ```
    pub(crate) fn new(day_of_year: u16, obs_files: Vec<String>) -> Self {
        Self {
            day_of_year,
            obs_files,
        }
    }

    /// Returns an iterator over the paths of the observation files in the `ObsFilesInDay`.
    ///
    /// # Arguments
    ///
    /// * `self` - A reference to the `ObsFilesInDay`.
    ///
    /// # Returns
    ///
    /// An iterator over the paths of the observation files.
    ///
    /// # Example
    ///
    /// ```
    /// use gnss_preprocess::ObsFilesInDay;
    /// use std::path::PathBuf;
    ///
    /// let day_of_year = 123;
    /// let obs_files = vec!["file1.obs".to_string(), "file2.obs".to_string()];
    /// let obs_file_item = ObsFilesInDay::new(day_of_year, obs_files);
    ///
    /// let paths: Vec<PathBuf> = obs_file_item.iter().collect();
    /// for path in paths {
    ///     println!("Path: {:?}", path);
    /// }
    /// ```
    pub(crate) fn iter(&self) -> impl Iterator<Item = PathBuf> + '_ {
        self.obs_files.iter().map(|file_name| {
            PathBuf::from(format!("{:03}", self.day_of_year))
                .join("daily")
                .join(file_name)
        })
    }
}

/// The `ObsFilesInYear` struct represents an item in the `ObsFilesTree`, containing the year and a list of `ObsFilesInDay` objects
/// which observed in that year.
/// It also provides an iterator to iterate over the observation file paths.
///
/// # Fields
///
/// - `year`: The year of the observation files.
/// - `obs_file_items`: A list of `ObsFilesInDay` objects representing the observation files for the year.
///
/// # Examples
///
/// ```
/// use std::path::PathBuf;
///
/// let obs_files = vec!["file1.obs".to_string(), "file2.obs".to_string()];
/// let obs_file_item = ObsFilesInDay::new(123, obs_files);
///
/// let mut iter = obs_file_item.iter();
/// assert_eq!(iter.next(), Some(PathBuf::from("123/daily/file1.obs")));
/// assert_eq!(iter.next(), Some(PathBuf::from("123/daily/file2.obs")));
/// assert_eq!(iter.next(), None);
/// ```
#[derive(Clone, Eq, Debug)]
pub(crate) struct ObsFilesInYear {
    year: u16,
    obs_file_items: Vec<ObsFilesInDay>,
}

#[allow(dead_code)]
impl ObsFilesInYear {
    /// Creates a new `ObsFilesInYear` object.
    /// # Arguments
    /// - `year`: The year of the observation files.
    /// - `obs_file_items`: A list of `ObsFilesInDay` objects representing the observation files for the year.
    ///
    /// # Returns
    /// A new `ObsFilesInYear` object.
    ///
    /// # Examples
    /// ```
    /// use std::path::PathBuf;
    /// use gnss_preprocess::obs_files_tree::{ObsFilesInDay, ObsFilesInYear};
    /// let obs_files = vec!["file1.obs".to_string(), "file2.obs".to_string()];
    /// let obs_file_item = ObsFilesInDay::new(123, obs_files);
    /// let obs_files_tree_item = ObsFilesInYear::new(2023, vec![obs_file_item]);
    /// ```
    pub(crate) fn new(year: u16, obs_file_items: Vec<ObsFilesInDay>) -> Self {
        Self {
            year,
            obs_file_items,
        }
    }

    /// Creates an empty `ObsFilesInYear` object for the specified year.
    ///
    /// # Arguments
    /// - `year`: The year of the observation files.
    ///
    /// # Returns
    /// A new `ObsFilesInYear` object with no observation files.
    ///
    /// # Examples
    /// ```
    /// use gnss_preprocess::obs_files_tree::ObsFilesInYear;
    /// let obs_files_tree_item = ObsFilesInYear::create_empty(2023);
    /// ```
    pub(crate) fn create_empty(year: u16) -> Self {
        Self {
            year,
            obs_file_items: Vec::new(),
        }
    }

    /// Returns how much days in the `ObsFilesInYear`.
    pub(crate) fn days(&self) -> usize {
        self.obs_file_items.len()
    }

    /// Adds an `ObsFilesInDay` to the `ObsFilesInYear`.
    ///
    /// # Arguments
    /// - `obs_file_item`: The `ObsFilesInDay` to be added.
    ///
    /// # Examples
    /// ```
    /// use gnss_preprocess::obs_files_tree::{ObsFilesInDay, ObsFilesInYear};
    /// let obs_files = vec!["file1.obs".to_string(), "file2.obs".to_string()];
    /// let obs_file_item = ObsFilesInDay::new(123, obs_files);
    /// let mut obs_files_tree_item = ObsFilesInYear::new(2023, vec![obs_file_item]);
    /// obs_files_tree_item.add_item(obs_file_item);
    /// ```
    pub(crate) fn add_item(&mut self, obs_file_item: ObsFilesInDay) {
        self.obs_file_items.push(obs_file_item);
    }

    /// Returns an iterator over the observation file paths.
    /// # Returns
    /// An iterator over the observation file paths.
    /// # Examples
    /// ```
    /// use std::path::PathBuf;
    /// use gnss_preprocess::obs_files_tree::{ObsFilesInDay, ObsFilesInYear};
    /// let obs_files = vec!["file1.obs".to_string(), "file2.obs".to_string()];
    /// let obs_file_item = ObsFilesInDay::new(123, obs_files);
    /// let obs_files_tree_item = ObsFilesInYear::new(2023, vec![obs_file_item]);
    /// let mut iter = obs_files_tree_item.iter();
    /// assert_eq!(iter.next(), Some(PathBuf::from("2023/123/daily/file1.obs")));
    /// assert_eq!(iter.next(), Some(PathBuf::from("2023/123/daily/file2.obs")));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub(crate) fn iter(&self) -> impl Iterator<Item = PathBuf> + '_ {
        self.obs_file_items.iter().flat_map(|obs_item| {
            obs_item
                .iter()
                .map(|path| PathBuf::from(self.year.to_string()).join(path))
        })
    }

    /// Returns an iterator over the observation file paths for each day in the `ObsFilesInYear`.
    ///
    /// # Returns
    /// An iterator yielding tuples containing the year, day of the year and the corresponding observation file path.
    ///
    /// # Examples
    /// ```
    /// use std::path::PathBuf;
    /// use gnss_preprocess::obs_files_tree::{ObsFilesInDay, ObsFilesInYear};
    /// let obs_files = vec!["file1.obs".to_string(), "file2.obs".to_string()];
    /// let obs_file_item = ObsFilesInDay::new(123, obs_files);
    /// let obs_files_tree_item = ObsFilesInYear::new(2023, vec![obs_file_item]);
    /// let mut iter = obs_files_tree_item.iter_day_paths();
    /// assert_eq!(iter.next(), Some((123, PathBuf::from("2023/123/daily/file1.obs"))));
    /// assert_eq!(iter.next(), Some((123, PathBuf::from("2023/123/daily/file2.obs"))));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub(crate) fn iter_paths(&self) -> impl Iterator<Item = (u16, u16, PathBuf)> + '_ {
        self.obs_file_items.iter().flat_map(|obs_item| {
            obs_item.iter().map(|path| {
                (
                    self.year,
                    obs_item.day_of_year,
                    PathBuf::from(self.year.to_string()).join(path),
                )
            })
        })
    }

    /// Returns a reference to the observation files for each day in the `ObsFilesInYear`.
    ///
    /// # Returns
    /// A reference to a slice of `ObsFilesInDay` objects representing the observation files for each day in the year.
    ///
    /// # Examples
    /// ```
    /// use gnss_preprocess::obs_files_tree::{ObsFilesInDay, ObsFilesInYear};
    /// let obs_files = vec!["file1.obs".to_string(), "file2.obs".to_string()];
    /// let obs_file_item = ObsFilesInDay::new(123, obs_files);
    /// let obs_files_tree_item = ObsFilesInYear::new(2023, vec![obs_file_item]);
    /// let day_files = obs_files_tree_item.get_day_files();
    /// ```
    pub(crate) fn get_day_files(&self) -> &[ObsFilesInDay] {
        &self.obs_file_items
    }

    /// Sorts the observation files in the `ObsFilesInYear` by the day of the year
    /// in ascending order.
    /// # Examples
    /// ```
    /// use gnss_preprocess::obs_files_tree::{ObsFilesInDay, ObsFilesInYear};
    /// let obs_files = vec!["file1.obs".to_string(), "file2.obs".to_string()];
    /// let obs_file_item = ObsFilesInDay::new(123, obs_files);
    /// let mut obs_files_tree_item = ObsFilesInYear::new(2023, vec![obs_file_item]);
    /// obs_files_tree_item.sort();
    /// ```
    pub(crate) fn sort(&mut self) {
        self.obs_file_items.sort_by_key(|item| item.day_of_year);
    }
}

impl PartialEq for ObsFilesInYear {
    fn eq(&self, other: &Self) -> bool {
        self.year == other.year
    }
}

impl PartialOrd for ObsFilesInYear {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.year.partial_cmp(&other.year)
    }
}

impl Ord for ObsFilesInYear {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.year.cmp(&other.year)
    }
}

/// The `ObsFilesTree` struct contains a collection of `ObsFilesInYear` objects and provides methods to iterate over the observation file paths.
///
/// # Examples
///
/// ```
/// use gnss_preprocess::obs_files_tree::ObsFilesTree;
///
/// let obs_files_tree = ObsFilesTree::new();
/// let obs_files = obs_files_tree.get_obs_files();
/// for obs_file in obs_files {
///     println!("Observation file: {:?}", obs_file);
/// }
/// ```
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub(crate) struct ObsFilesTree {
    items: Vec<ObsFilesInYear>,
}

#[allow(dead_code)]
impl ObsFilesTree {
    /// Creates a new `ObsFilesTree` object.
    ///
    /// # Returns
    ///
    /// A new `ObsFilesTree` instance.
    pub(crate) fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// Adds an `ObsFilesInYear` to the `ObsFilesTree`
    /// and sorts the observation files in the `ObsFilesInYear` by the day of the year.
    ///
    /// # Arguments
    ///
    /// * `item` - The `ObsFilesInYear` to add.
    pub(crate) fn add_item(&mut self, mut item: ObsFilesInYear) {
        item.sort();
        let index = self.items.binary_search(&item).unwrap_or_else(|x| x);
        self.items.insert(index, item);
    }

    /// Returns the total number of days in the `ObsFilesTree`.
    /// # Returns
    /// The total number of days in the `ObsFilesTree`.
    pub(crate) fn get_day_numbers(&self) -> usize {
        self.items
            .iter()
            .map(|item| item.obs_file_items.len())
            .sum()
    }

    /// Returns an iterator over the observation file paths in the `ObsFilesTree`.
    ///
    /// # Returns
    ///
    /// An iterator over the observation file paths.
    pub(crate) fn get_obs_files(&self) -> impl Iterator<Item = PathBuf> + '_ {
        self.items.iter().flat_map(|item| item.iter())
    }

    /// Returns an iterator over the observation file paths in the `ObsFilesTree`.
    ///
    /// # Returns
    ///
    /// An iterator over the observation file paths, which yields tuples containing
    ///  the year, day of the year and the corresponding observation file path.
    pub(crate) fn get_files(&self) -> impl Iterator<Item = (u16, u16, PathBuf)> + '_ {
        self.items.iter().flat_map(|item| item.iter_paths())
    }

    /// Finds the next observation file with the specified name, year and day of the year.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the observation file.
    /// * `year` - The year of the observation file.
    /// * `day_of_year` - The day of the year of the observation file.
    ///
    /// # Returns
    ///
    /// The path of the next observation file with the specified name, year and day of the year.
    ///
    pub(crate) fn find_next_file(
        &self,
        name: &str,
        year: u16,
        day_of_year: u16,
    ) -> Option<PathBuf> {
        let next_day = get_next_day(year, day_of_year);
        self.items.iter().find_map(|item| {
            if item.year == next_day.0 {
                item.obs_file_items.iter().find_map(|obs_item| {
                    if obs_item.day_of_year == next_day.1 {
                        obs_item
                            .obs_files
                            .iter()
                            .find(|file_name| file_name.starts_with(name))
                            .map(|file_name| {
                                PathBuf::from(format!("{}", next_day.0))
                                    .join(format!("{:03}", next_day.1))
                                    .join("daily")
                                    .join(file_name)
                            })
                    } else {
                        None
                    }
                })
            } else {
                None
            }
        })
    }

    /// Splits the `ObsFilesTree` into two parts based on the given percentage
    /// which counts the number in days not in files.
    ///
    /// # Arguments
    ///
    /// * `percent` - The percentage at which to split the `ObsFilesTree`.
    ///
    /// # Returns
    ///
    /// A tuple containing two `ObsFilesTree` objects, representing the left and right parts of the split.
    pub(crate) fn split_by_percent(&self, percent: u8) -> (Self, Self) {
        let total_count = self.get_day_numbers();
        let left_count = (total_count as f64 * percent as f64 / 100.0).round() as usize;
        let mut left = Vec::new();
        let mut right = Vec::new();
        let mut _count = 0;
        for year_files in &self.items {
            if _count < left_count {
                if _count + year_files.days() < left_count {
                    left.push(year_files.clone());
                    _count += year_files.days();
                } else {
                    let left_part_year = ObsFilesInYear::new(
                        year_files.year,
                        year_files.get_day_files()[..(left_count - _count)].to_vec(),
                    );
                    let right_part_year = ObsFilesInYear::new(
                        year_files.year,
                        year_files.get_day_files()[(left_count - _count)..].to_vec(),
                    );
                    _count += left_part_year.days();
                    left.push(left_part_year);
                    right.push(right_part_year);
                }
            } else {
                right.push(year_files.clone());
            }
        }
        (ObsFilesTree { items: left }, ObsFilesTree { items: right })
    }

    /// Creates an `ObsFilesTree` object from the specified observation data.
    /// This method is used for testing purposes.
    #[cfg(test)]
    pub(super) fn from_data(obs_data: HashMap<u16, HashMap<u16, Vec<&'static str>>>) -> Self {
        let mut obs_files_tree = ObsFilesTree::new();
        for (year, day_files) in obs_data {
            let mut obs_file_items = Vec::new();
            for (day, files) in day_files {
                let obs_file_item =
                    ObsFilesInDay::new(day, files.iter().map(|f| f.to_string()).collect());
                obs_file_items.push(obs_file_item);
            }
            let obs_files_tree_item = ObsFilesInYear::new(year, obs_file_items);
            obs_files_tree.add_item(obs_files_tree_item);
        }
        obs_files_tree
    }
}

#[cfg(test)]
mod tests;

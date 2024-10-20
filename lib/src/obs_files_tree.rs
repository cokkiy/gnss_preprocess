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
    /// // Output:
    /// // Path: 123/daily/file1.obs
    /// // Path: 123/daily/file2.obs
    /// ```
    pub(crate) fn iter(&self) -> impl Iterator<Item = PathBuf> + '_ {
        self.obs_files.iter().map(|file_name| {
            PathBuf::from(format!("{:03}", self.day_of_year))
                .join("daily")
                .join(file_name)
        })
    }

    /// Iterates over the observation file names in the `ObsFilesInDay` and get the day_of_year
    /// and station name.
    /// # Returns
    /// An iterator yielding tuples containing the day of the year and the station name.
    /// # Examples
    /// ```
    /// use gnss_preprocess::ObsFilesInDay;
    /// let obs_files = vec!["nreq1230.obs".to_string(), "hewq1230.obs".to_string()];
    /// let obs_file_item = ObsFilesInDay::new(123, obs_files);
    /// let mut iter = obs_file_item.station_iter();
    /// assert_eq!(iter.next(), Some((123, "nreq".to_string())));
    /// assert_eq!(iter.next(), Some((123, "hewq".to_string())));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub(crate) fn station_iter(&self) -> impl Iterator<Item = (u16, String)> + '_ {
        self.obs_files.iter().map(|file_name| {
            (
                self.day_of_year,
                // The station name is the first four characters of the observation file name.
                file_name.split('.').next().unwrap()[..4].to_string(),
            )
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

    /// Iterates over the `ObsFilesInYear` and get the year, day_of_year and station name.
    /// # Returns
    /// An iterator yielding tuples containing the year, day of the year and the station name.
    /// # Examples
    /// ```
    /// use gnss_preprocess::obs_files_tree::{ObsFilesInDay, ObsFilesInYear};
    /// let obs_files = vec!["abmf1230.23o".to_string(), "abpo1230.23o".to_string()];
    /// let obs_file_item = ObsFilesInDay::new(123, obs_files);
    /// let obs_files_tree_item = ObsFilesInYear::new(2023, vec![obs_file_item]);
    /// let mut iter = obs_files_tree_item.iter_stations();
    /// assert_eq!(iter.next(), Some((2023, 123, "abmf".to_string())));
    /// assert_eq!(iter.next(), Some((2023, 123, "abpo".to_string())));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub(crate) fn iter_stations(&self) -> impl Iterator<Item = (u16, u16, String)> + '_ {
        self.obs_file_items.iter().flat_map(|obs_item| {
            obs_item
                .station_iter()
                .map(|(day, station)| (self.year, day, station))
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
    base_path: String,
    items: Vec<ObsFilesInYear>,
}

#[allow(dead_code)]
impl ObsFilesTree {
    /// Creates a new `ObsFilesTree` object.
    ///
    /// # Arguments
    /// * `base_path` - The base path of the observation files.
    ///
    /// # Returns
    ///
    /// A new `ObsFilesTree` instance.
    pub(crate) fn new(base_path: &str) -> Self {
        Self {
            base_path: base_path.to_string(),
            items: Vec::new(),
        }
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

    /// Finds an observation file which observed by the `name` specified station at the given `year` and `day_of_year`.
    /// # Arguments
    /// * `year` - The year of the observation.
    /// * `day_of_year` - The day of the year of the observation.
    /// * `name` - The observation station name.
    /// # Returns
    /// The full path of the observation file which observed by the specified station at the given year and day of the year.
    /// If the observation file is not found, it returns `None`.
    ///
    /// # Note
    /// The observation file name should start with the `name` specified station name.
    pub(crate) fn find_file(&self, year: u16, day_of_year: u16, name: &str) -> Option<PathBuf> {
        self.items.iter().find_map(|item| {
            if item.year == year {
                item.obs_file_items.iter().find_map(|obs_item| {
                    if obs_item.day_of_year == day_of_year {
                        obs_item
                            .obs_files
                            .iter()
                            .find(|file_name| file_name.starts_with(name))
                            .map(|file_name| {
                                PathBuf::from(format!("{}/{}", self.base_path, year))
                                    .join(format!("{:03}", day_of_year))
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
        (
            ObsFilesTree {
                base_path: self.base_path.clone(),
                items: left,
            },
            ObsFilesTree {
                base_path: self.base_path.clone(),
                items: right,
            },
        )
    }

    /// Returns an iterator over this `ObsFilesTree` and get the year, day_of_year and station name.
    /// # Returns
    /// An iterator yielding tuples containing the year, day of the year and the station name.
    /// # Examples
    /// ```
    /// use gnss_preprocess::obs_files_tree::ObsFilesTree;
    /// let obs_files = vec!["file1.obs".to_string(), "file2.obs".to_string()];
    /// let obs_file_item = ObsFilesInDay::new(123, obs_files);
    /// let mut obs_files_tree = ObsFilesTree::new("");
    /// obs_files_tree.add_item(ObsFilesInYear::new(2023, vec![obs_file_item]));
    /// let mut iter = obs_files_tree.iter();
    /// assert_eq!(iter.next(), Some((2023, 123, "file1".to_string())));
    /// assert_eq!(iter.next(), Some((2023, 123, "file2".to_string())));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub(crate) fn iter(&self) -> impl Iterator<Item = (u16, u16, String)> + '_ {
        self.items.iter().flat_map(|item| item.iter_stations())
    }

    /// Creates an `ObsFilesTree` object Iterates over the specified observation files path.
    /// # Arguments
    /// * `obs_files_path` - The path of the observation files.
    /// # Returns
    /// A new `ObsFilesTree` object.
    /// # Examples
    /// ```
    /// use gnss_preprocess::obs_files_tree::ObsFilesTree;
    /// let obs_files_tree = ObsFilesTree::create_obs_tree("path/to/obs_files");
    /// ```
    /// # Panics
    /// This method panics if the observation files path is not found.
    /// # Note
    /// Iterates over the observation files and creates an `ObsFilesTree` object.
    ///
    /// The observation files should be organized in the following structure:
    /// ```text
    /// obs_files_path
    /// ├── year1
    /// │   ├── day1
    /// │   │   └── daily
    /// │   │       ├── file1.obs
    /// │   │       └── file2.obs
    /// │   └── day2
    /// │       └── daily
    /// │           ├── file1.obs
    /// │           └── file2.obs
    /// └── year2
    ///    ├── day1
    ///    │   └── daily
    ///    │       ├── file1.obs
    ///    │       └── file2.obs
    /// ```
    pub fn create_obs_tree(obs_files_path: &str) -> ObsFilesTree {
        let mut obs_data_tree = ObsFilesTree::new(obs_files_path);
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
                                    .expect(
                                        format!("Failed to parse day of year: {:?}", day_entry)
                                            .as_str(),
                                    );
                                let mut obs_files_in_days = Vec::new();
                                if let Ok(files) = std::fs::read_dir(day_entry.path().join("daily"))
                                {
                                    files.map(|file| file.unwrap()).for_each(|file| {
                                        obs_files_in_days
                                            .push(file.file_name().to_string_lossy().to_string());
                                    });
                                }
                                let obs_file_item =
                                    ObsFilesInDay::new(day_of_year, obs_files_in_days);
                                obs_files_in_year.add_item(obs_file_item);
                            });
                    }
                    obs_data_tree.add_item(obs_files_in_year);
                });
        };

        obs_data_tree
    }

    /// Creates an `ObsFilesTree` object from the specified observation data.
    /// This method is used for testing purposes.
    #[cfg(test)]
    pub(super) fn from_data(obs_data: HashMap<u16, HashMap<u16, Vec<&'static str>>>) -> Self {
        let mut obs_files_tree = ObsFilesTree::new("");
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

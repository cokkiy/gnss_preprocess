/// This module contains the implementation of the `ObsFilesTree` struct and related types.
///

///
/// The `ObsFilesTreeItem` struct represents an item in the `ObsFilesTree`, containing the year and a list of `ObsFileItem` objects.
/// It also provides an iterator, `ObsFilesTreeItemIter`, to iterate over the observation file paths.
///
/// The `ObsFilesTree` struct contains a collection of `ObsFilesTreeItem` objects and provides methods to iterate over the observation file paths.
use std::path::PathBuf;

use chrono::format::Item;

/// The `ObsFileItem` struct represents a single item in the tree, containing the day of year and a list of observation file names
/// which observed in that day.
/// It also provides an iterator, `ObsFileItemIter`, to iterate over the observation file paths.
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
/// let obs_file_item = ObsFileItem::new(123, obs_files);
///
/// let mut iter = obs_file_item.iter();
/// assert_eq!(iter.next(), Some(PathBuf::from("123/daily/file1.obs")));
/// assert_eq!(iter.next(), Some(PathBuf::from("123/daily/file2.obs")));
/// assert_eq!(iter.next(), None);
/// ```
#[derive(Clone, Eq, Debug)]
pub(crate) struct ObsFileItem {
    day_of_year: u16,
    obs_files: Vec<String>,
}

impl PartialEq for ObsFileItem {
    fn eq(&self, other: &Self) -> bool {
        self.day_of_year == other.day_of_year
    }
}

impl ObsFileItem {
    /// Creates a new `ObsFileItem` with the specified `day_of_year` and `obs_files`.
    ///
    /// # Arguments
    ///
    /// * `day_of_year` - The day of the year.
    /// * `obs_files` - The vector of observation files.
    ///
    /// # Returns
    ///
    /// A new `ObsFileItem` instance.
    ///
    /// # Example
    ///
    /// ```
    /// use gnss_preprocess::ObsFileItem;
    ///
    /// let day_of_year = 123;
    /// let obs_files = vec!["file1.obs".to_string(), "file2.obs".to_string()];
    /// let obs_file_item = ObsFileItem::new(day_of_year, obs_files);
    /// ```
    pub(crate) fn new(day_of_year: u16, obs_files: Vec<String>) -> Self {
        Self {
            day_of_year,
            obs_files,
        }
    }

    /// Returns an iterator over the `ObsFileItem`.
    ///
    /// # Arguments
    ///
    /// * `self` - A reference to the `ObsFileItem`.
    ///
    /// # Returns
    ///
    /// An `ObsFileItemIter` iterator.
    pub(crate) fn iter<'a>(self: &'a Self) -> impl Iterator<Item = PathBuf> + 'a {
        self.obs_files.iter().map(|file_name| {
            PathBuf::from(self.day_of_year.to_string())
                .join("daily")
                .join(file_name)
        })
    }
}

/// The `ObsFilesTreeItem` struct represents an item in the `ObsFilesTree`, containing the year and a list of `ObsFileItem` objects.
/// It also provides an iterator, `ObsFilesTreeItemIter`, to iterate over the observation file paths.
///
/// # Fields
///
/// - `year`: The year of the observation files.
/// - `obs_file_items`: A list of `ObsFileItem` objects representing the observation files for the year.
///
/// # Examples
///
/// ```
/// use std::path::PathBuf;
///
/// let obs_files = vec!["file1.obs".to_string(), "file2.obs".to_string()];
/// let obs_file_item = ObsFileItem::new(123, obs_files);
///
/// let mut iter = obs_file_item.iter();
/// assert_eq!(iter.next(), Some(PathBuf::from("123/daily/file1.obs")));
/// assert_eq!(iter.next(), Some(PathBuf::from("123/daily/file2.obs")));
/// assert_eq!(iter.next(), None);
/// ```
#[derive(Clone, Eq, Debug)]
pub struct ObsFilesTreeItem {
    year: u16,
    obs_file_items: Vec<ObsFileItem>,
}

impl ObsFilesTreeItem {
    /// Creates a new `ObsFilesTreeItem` object.
    /// # Arguments
    /// - `year`: The year of the observation files.
    /// - `obs_file_items`: A list of `ObsFileItem` objects representing the observation files for the year.
    ///
    /// # Returns
    /// A new `ObsFilesTreeItem` object.
    ///
    /// # Examples
    /// ```
    /// use std::path::PathBuf;
    /// use gnss_preprocess::obs_files_tree::{ObsFileItem, ObsFilesTreeItem};
    /// let obs_files = vec!["file1.obs".to_string(), "file2.obs".to_string()];
    /// let obs_file_item = ObsFileItem::new(123, obs_files);
    /// let obs_files_tree_item = ObsFilesTreeItem::new(2023, vec![obs_file_item]);
    /// ```
    pub fn new(year: u16, obs_file_items: Vec<ObsFileItem>) -> Self {
        Self {
            year,
            obs_file_items,
        }
    }

    /// Returns an iterator over the observation file paths.
    /// # Returns
    /// An iterator over the observation file paths.
    /// # Examples
    /// ```
    /// use std::path::PathBuf;
    /// use gnss_preprocess::obs_files_tree::{ObsFileItem, ObsFilesTreeItem};
    /// let obs_files = vec!["file1.obs".to_string(), "file2.obs".to_string()];
    /// let obs_file_item = ObsFileItem::new(123, obs_files);
    /// let obs_files_tree_item = ObsFilesTreeItem::new(2023, vec![obs_file_item]);
    /// let mut iter = obs_files_tree_item.iter();
    /// assert_eq!(iter.next(), Some(PathBuf::from("2023/123/daily/file1.obs")));
    /// assert_eq!(iter.next(), Some(PathBuf::from("2023/123/daily/file2.obs")));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter<'a>(self: &'a Self) -> impl Iterator<Item = PathBuf> + 'a {
        self.obs_file_items.iter().flat_map(|obs_item| {
            obs_item
                .iter()
                .map(|path| PathBuf::from(self.year.to_string()).join(path))
        })
    }
}

impl PartialEq for ObsFilesTreeItem {
    fn eq(&self, other: &Self) -> bool {
        self.year == other.year
    }
}

/// The `ObsFilesTree` struct contains a collection of `ObsFilesTreeItem` objects and provides methods to iterate over the observation file paths.
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
pub(crate) struct ObsFilesTree {
    items: Vec<ObsFilesTreeItem>,
}

impl ObsFilesTree {
    /// Creates a new `ObsFilesTree` object.
    ///
    /// # Returns
    ///
    /// A new `ObsFilesTree` instance.
    pub(crate) fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// Adds an `ObsFilesTreeItem` to the `ObsFilesTree`.
    ///
    /// # Arguments
    ///
    /// * `item` - The `ObsFilesTreeItem` to add.
    pub(crate) fn add_item(&mut self, item: ObsFilesTreeItem) {
        self.items.push(item);
    }

    /// Returns an iterator over the observation file paths in the `ObsFilesTree`.
    ///
    /// # Returns
    ///
    /// An iterator over the observation file paths.
    pub(crate) fn get_obs_files<'a>(&'a self) -> impl Iterator<Item = PathBuf> + 'a {
        self.items.iter().flat_map(|item| item.iter())
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_obs_file_item_iter() {
        let obs_files = vec!["file1.obs".to_string(), "file2.obs".to_string()];
        let obs_file_item = ObsFileItem::new(123, obs_files);

        let mut iter = obs_file_item.iter();
        assert_eq!(iter.next(), Some(PathBuf::from("123/daily/file1.obs")));
        assert_eq!(iter.next(), Some(PathBuf::from("123/daily/file2.obs")));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_obs_file_item_iter_empty() {
        let obs_files = Vec::new();
        let obs_file_item = ObsFileItem::new(123, obs_files);

        let mut iter = obs_file_item.iter();
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_obs_file_item_iter_multiple_items() {
        let obs_files1 = vec!["file1.obs".to_string(), "file2.obs".to_string()];
        let obs_file_item1 = ObsFileItem::new(123, obs_files1);

        let obs_files2 = vec!["file3.obs".to_string(), "file4.obs".to_string()];
        let obs_file_item2 = ObsFileItem::new(456, obs_files2);

        let mut iter = obs_file_item1.iter().chain(obs_file_item2.iter());
        assert_eq!(iter.next(), Some(PathBuf::from("123/daily/file1.obs")));
        assert_eq!(iter.next(), Some(PathBuf::from("123/daily/file2.obs")));
        assert_eq!(iter.next(), Some(PathBuf::from("456/daily/file3.obs")));
        assert_eq!(iter.next(), Some(PathBuf::from("456/daily/file4.obs")));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_obs_files_tree_item_iter() {
        let obs_files = vec!["file1.obs".to_string(), "file2.obs".to_string()];
        let obs_file_item = ObsFileItem::new(123, obs_files);
        let obs_files_tree_item = ObsFilesTreeItem::new(2023, vec![obs_file_item]);

        let mut iter = obs_files_tree_item.iter();
        assert_eq!(iter.next(), Some(PathBuf::from("2023/123/daily/file1.obs")));
        assert_eq!(iter.next(), Some(PathBuf::from("2023/123/daily/file2.obs")));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_obs_files_tree_item_iter_empty() {
        let obs_files_tree_item = ObsFilesTreeItem::new(2023, Vec::new());

        let mut iter = obs_files_tree_item.iter();
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_obs_files_tree_item_iter_multiple_items() {
        let obs_files1 = vec!["file1.obs".to_string(), "file2.obs".to_string()];
        let obs_file_item1 = ObsFileItem::new(123, obs_files1);

        let obs_files2 = vec!["file3.obs".to_string(), "file4.obs".to_string()];
        let obs_file_item2 = ObsFileItem::new(456, obs_files2);

        let obs_files_tree_item = ObsFilesTreeItem::new(2023, vec![obs_file_item1, obs_file_item2]);

        let mut iter = obs_files_tree_item.iter();
        assert_eq!(iter.next(), Some(PathBuf::from("2023/123/daily/file1.obs")));
        assert_eq!(iter.next(), Some(PathBuf::from("2023/123/daily/file2.obs")));
        assert_eq!(iter.next(), Some(PathBuf::from("2023/456/daily/file3.obs")));
        assert_eq!(iter.next(), Some(PathBuf::from("2023/456/daily/file4.obs")));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_obs_files_tree_get_obs_files() {
        let obs_files1 = vec!["file1.obs".to_string(), "file2.obs".to_string()];
        let obs_file_item1 = ObsFileItem::new(123, obs_files1);

        let obs_files2 = vec!["file3.obs".to_string(), "file4.obs".to_string()];
        let obs_file_item2 = ObsFileItem::new(456, obs_files2);

        let obs_files_tree_item1 = ObsFilesTreeItem::new(2023, vec![obs_file_item1]);
        let obs_files_tree_item2 = ObsFilesTreeItem::new(2024, vec![obs_file_item2]);

        let mut obs_files_tree = ObsFilesTree::new();
        obs_files_tree.add_item(obs_files_tree_item1);
        obs_files_tree.add_item(obs_files_tree_item2);

        let mut iter = obs_files_tree.get_obs_files();
        assert_eq!(iter.next(), Some(PathBuf::from("2023/123/daily/file1.obs")));
        assert_eq!(iter.next(), Some(PathBuf::from("2023/123/daily/file2.obs")));
        assert_eq!(iter.next(), Some(PathBuf::from("2024/456/daily/file3.obs")));
        assert_eq!(iter.next(), Some(PathBuf::from("2024/456/daily/file4.obs")));
        assert_eq!(iter.next(), None);
    }
}

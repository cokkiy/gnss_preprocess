use pyo3::prelude::*;
use std::path::PathBuf;
use std::thread;

use crate::obsdata_provider::ObsDataProvider;
use crate::NavDataProvider;
use crate::ObsFileProvider;

/// The `GNSSDataProvider` struct provides GNSS data.
/// It reads GNSS observation data from the GNSS files path and provides interpolation for
/// the GNSS navigation data for any valid time.
#[allow(dead_code)]
#[pyclass]
pub struct GNSSDataProvider {
    gnss_data_path: String,
    training_data_files: ObsFileProvider,
    testing_data_files: ObsFileProvider,
    nav_data_provider: NavDataProvider,
}

#[pymethods]
impl GNSSDataProvider {
    #[new]
    #[pyo3(signature = (gnss_files_path, percent=None))]
    pub fn new(gnss_files_path: &str, percent: Option<u8>) -> Self {
        let obs_data_provider = ObsFileProvider::new(
            PathBuf::from(gnss_files_path)
                .join("Obs")
                .to_str()
                .expect("Invalid UTF-8 sequence in path"),
        );
        let (training_data_files, testing_data_files) =
            obs_data_provider.split_by_percent(percent.unwrap_or(80));
        Self {
            gnss_data_path: gnss_files_path.to_string(),
            training_data_files,
            testing_data_files,
            nav_data_provider: NavDataProvider::new(
                PathBuf::from(gnss_files_path).join("Nav").to_str().unwrap(),
            ),
        }
    }

    /// Get the training data iterator.
    ///
    /// This function returns an iterator over the training data.
    /// It uses the current year and day to load the navigation data provider.
    ///
    /// # Returns
    ///
    /// Returns an iterator over the training data.
    pub fn train_iter(&mut self) -> DataIter {
        DataIter::new(
            self.gnss_data_path.clone(),
            self.training_data_files.clone(),
            self.nav_data_provider.clone(),
        )
    }

    /// Get the testing data iterator.
    ///
    /// This function returns an iterator over the testing data.
    /// It uses the current year and day to load the navigation data provider.
    ///
    /// # Returns
    ///
    /// Returns an iterator over the testing data.
    pub fn test_iter(&mut self) -> DataIter {
        DataIter::new(
            self.gnss_data_path.clone(),
            self.testing_data_files.clone(),
            self.nav_data_provider.clone(),
        )
    }
}

/// The `ObsDataProviderManager` struct manages the observation data providers.
/// It provides methods to iterate through the observation data providers and load the next one if necessary.
struct ObsDataProviderManager {
    cur_provider: Option<ObsDataProvider>,
    cur_obs_file_index: usize,
    data_files: ObsFileProvider,
    base_path: String,
    current_year: u16,
    current_day: u16,
    handle: Option<thread::JoinHandle<Option<(u16, u16, ObsDataProvider, usize)>>>,
}

/// The `ObsDataProviderManager` struct manages the observation data providers.
/// It provides methods to iterate through the observation data providers and load the next one if necessary.
impl ObsDataProviderManager {
    /// Creates a new `ObsDataProviderManager`.
    ///
    /// # Arguments
    ///
    /// * `base_path` - The base path for the observation data files.
    /// * `data_files` - The observation data files to manage.
    fn new(base_path: String, data_files: ObsFileProvider) -> Self {
        Self {
            cur_provider: None,
            cur_obs_file_index: 0,
            data_files,
            base_path,
            current_day: 0,
            current_year: 0,
            handle: None,
        }
    }

    /// Get the next observation data provider.
    ///
    /// This function returns the next observation data provider in the sequence.
    /// It updates the current year and day, and loads the next provider if necessary.
    ///
    /// # Returns
    ///
    /// Returns an `Option` containing a tuple of the year, day, and the next observation data provider.
    /// If there are no more providers, it returns `None`.
    ///
    fn next(&mut self) -> Option<(u16, u16, ObsDataProvider)> {
        if self.handle.is_none() {
            self.handle = self.load_next_provider();
        }
        if let Some(handle) = self.handle.take() {
            if let Ok(Some((year, day, obs_data_provider, index))) = handle.join() {
                self.cur_obs_file_index = index;
                self.current_year = year;
                self.current_day = day;
                self.cur_provider = Some(obs_data_provider);
                self.handle = self.load_next_provider();
                return Some((year, day, self.cur_provider.as_ref().unwrap().clone()));
            }
        }
        None
    }

    fn load_next_provider(
        &self,
    ) -> Option<thread::JoinHandle<Option<(u16, u16, ObsDataProvider, usize)>>> {
        let base_path = self.base_path.clone();
        let data_files = self.data_files.clone();
        let mut cur_obs_file_index = self.cur_obs_file_index;

        let handle = thread::spawn(move || {
            while let Some((y, d, file_name)) = data_files.iter().nth(cur_obs_file_index) {
                let obs_data_provider =
                    ObsDataProvider::new(PathBuf::from(&base_path).join("Obs").join(file_name));

                if let Ok(obs_data_provider) = obs_data_provider {
                    return Some((y, d, obs_data_provider, cur_obs_file_index));
                }
                cur_obs_file_index += 1;
            }
            None
        });
        Some(handle)
    }
}

/// The `DataIter` struct is an iterator over the GNSS data.
#[pyclass]
pub struct DataIter {
    obs_provider_manager: ObsDataProviderManager,
    nav_data_provider: NavDataProvider,
    current: Option<(u16, u16, ObsDataProvider)>,
}

impl DataIter {
    /// Creates a new `DataIter`.
    ///
    /// # Arguments
    ///
    /// * `base_path` - The base path for the observation data files.
    /// * `data_files` - The observation data files to manage.
    /// * `nav_data_provider` - The navigation data provider.
    fn new(
        base_path: String,
        data_files: ObsFileProvider,
        nav_data_provider: NavDataProvider,
    ) -> Self {
        Self {
            obs_provider_manager: ObsDataProviderManager::new(base_path, data_files),
            nav_data_provider,
            current: None,
        }
    }
}

#[pymethods]
impl DataIter {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<Vec<f64>> {
        slf.next()
    }
}

impl Iterator for DataIter {
    type Item = Vec<f64>;

    /// Get the next item in the iterator.
    ///
    /// This function returns the next item in the iterator.
    /// It updates the current year and day, and loads the next provider if necessary.
    ///
    /// # Returns
    ///
    /// Returns the next item in the iterator.
    /// If there are no more items, it returns `None`.
    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_none() {
            self.current = self.obs_provider_manager.next();
        }
        if let Some((y, d, obs_data_provider)) = &mut self.current {
            if let Some((sv, epoch, data)) = obs_data_provider.next() {
                let nav_data = self.nav_data_provider.sample(*y, *d, &sv, &epoch);
                let mut result = vec![];
                result.extend(data);
                result.extend(nav_data.unwrap_or(vec![0.0; 20]));
                Some(result)
            } else {
                self.current = self.obs_provider_manager.next();
                self.next()
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests;

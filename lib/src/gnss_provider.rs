use std::path::PathBuf;

use crate::NavDataProvider;
use crate::ObsFileProvider;

#[allow(dead_code)]
pub struct GNSSDataProvider {
    gnss_data_path: String,
    training_data: ObsFileProvider,
    testing_data: ObsFileProvider,
    nav_data_provider: NavDataProvider,
}

#[allow(dead_code)]
impl GNSSDataProvider {
    /// Creates a new instance of `GNSSDataProvider`.
    ///
    /// # Arguments
    ///
    /// * `gnss_files_path` - The path to the GNSS files.
    /// * `percent` - An optional percentage value (0-100) to set the percent field which used to
    ///               split the data into training set and testing set. If not provided, the default value is 80.
    ///
    /// # Returns
    ///
    /// A new instance of `GNSSDataProvider`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::PathBuf;
    /// use gnss_preprocess::GNSSDataProvider;
    ///
    /// let gnss_files_path = "/path/to/gnss/files";
    /// let percent = Some(90);
    ///
    /// let provider = GNSSDataProvider::new(gnss_files_path, percent);
    /// ```
    pub fn new(gnss_files_path: &str, percent: Option<u8>) -> Self {
        let obs_data_provider = ObsFileProvider::new(
            PathBuf::from(gnss_files_path)
                .join("Obs")
                .to_str()
                .expect("Invalid UTF-8 sequence in path"),
        );
        let (training_data, testing_data) =
            obs_data_provider.split_by_percent(percent.unwrap_or(80));
        Self {
            gnss_data_path: gnss_files_path.to_string(),
            training_data,
            testing_data,
            nav_data_provider: NavDataProvider::new(
                PathBuf::from(gnss_files_path).join("Obs").to_str().unwrap(),
            ),
        }
    }

    // pub fn train_data_iter(&self) -> impl Iterator<Item = (u8, Vec<f64>)> {
    //     self.training_data.iter().map(|(y, d, file_name)| {
    //         let obs_data = get_obs_data(
    //             PathBuf::from(self.gnss_data_path)
    //                 .join(file_name)
    //                 .to_str()
    //                 .expect("Invalid UTF-8 sequence in path"),
    //         );
    //         let nav_data = self.nav_data_provider.get_nav_data(y, d);
    //         (
    //             y as u8,
    //             obs_data.into_iter().chain(nav_data.into_iter()).collect(),
    //         )
    //     })
    // }
}

use pyo3::prelude::*;
mod beidou_data;
mod common;
mod constellation_keys;
mod galileo_data;
mod glonass_data;
mod gnss_data;
mod gnss_epoch_data;
mod gnss_provider;
mod gps_data;
mod irnss_data;
mod navdata_interpolation;
mod navdata_provider;
mod navigation_data;
mod obs_files_tree;
mod obsdata_provider;
mod obsfile_provider;
mod qzss_data;
mod sbas_data;
mod tna_fields;
pub use beidou_data::BeidouData;
pub use galileo_data::GalileoData;
pub use gnss_data::GnssData;
pub use gnss_provider::GNSSDataProvider;
pub use gps_data::GPSData;
pub use irnss_data::IRNSSData;
pub use navdata_provider::NavDataProvider;
pub use obsfile_provider::ObsFileProvider;
pub use qzss_data::QZSSData;
pub use sbas_data::SBASData;

/// A Python module implemented in Rust.
#[pymodule]
fn gnss_preprocess(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<GNSSDataProvider>()?;
    Ok(())
}

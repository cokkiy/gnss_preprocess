use pyo3::prelude::*;
mod constellation_keys;
mod gnss_provider;
mod navdata_interpolation;
mod navdata_provider;
mod navigation_data;
mod obs_files_tree;
mod obsdata_provider;
mod obsfile_provider;
mod tna_fields;
mod common;
pub use gnss_provider::GNSSDataProvider;
pub use navdata_provider::NavDataProvider;
pub use obsfile_provider::ObsFileProvider;

/// A Python module implemented in Rust.
#[pymodule]
fn gnss_preprocess(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<GNSSDataProvider>()?;
    Ok(())
}

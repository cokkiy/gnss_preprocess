use convert_macro::{FieldsPos, FromGnss, FromSlice, FromVec, ToSlice, ToVec, SSC};

/// data for SBAS constellation
#[derive(Clone, Debug, Default, FieldsPos, ToSlice, FromSlice, ToVec, FromVec, FromGnss, SSC)]
pub struct SBASData {
    c1c: f64,
    c5i: f64,
    c5x: f64,
    d1c: f64,
    d5i: f64,
    d5x: f64,
    l1c: f64,
    l5i: f64,
    l5x: f64,
    s1c: f64,
    s5i: f64,
    s5x: f64,
}

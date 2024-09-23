use convert_macro::{FieldsPos, FromGnss, FromSlice, FromVec, ToSlice, ToVec};

#[derive(Clone, Debug, Default, FieldsPos, ToSlice, FromSlice, ToVec, FromVec, FromGnss)]
pub struct GlonassData {
    c1c: f64,
    c1p: f64,
    c2c: f64,
    c2p: f64,
    c3q: f64,
    c3x: f64,
    d1c: f64,
    d1p: f64,
    d2c: f64,
    d2p: f64,
    d3q: f64,
    d3x: f64,
    l1c: f64,
    l1p: f64,
    l2c: f64,
    l2p: f64,
    l3q: f64,
    l3x: f64,
    s1c: f64,
    s1p: f64,
    s2c: f64,
    s2p: f64,
    s3q: f64,
    s3x: f64,
}

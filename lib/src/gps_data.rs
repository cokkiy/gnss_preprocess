use convert_macro::{
    FieldsCount, FieldsPos, FromGnss, FromSlice, FromVec, SSFieldsCount, ToSlice, ToVec, SSC,
};

#[derive(
    Clone,
    Debug,
    Default,
    FieldsPos,
    ToSlice,
    FromSlice,
    ToVec,
    FromVec,
    FromGnss,
    SSC,
    FieldsCount,
    SSFieldsCount,
)]
pub struct GPSData {
    c1c: f64,
    c1l: f64,
    c1p: f64,
    c1w: f64,
    c1x: f64,
    c2c: f64,
    c2l: f64,
    c2p: f64,
    c2s: f64,
    c2w: f64,
    c2x: f64,
    c2y: f64,
    c5i: f64,
    c5q: f64,
    c5x: f64,
    d1c: f64,
    d1l: f64,
    d1p: f64,
    d1w: f64,
    d1x: f64,
    d2c: f64,
    d2l: f64,
    d2p: f64,
    d2s: f64,
    d2w: f64,
    d2x: f64,
    d2y: f64,
    d5i: f64,
    d5q: f64,
    d5x: f64,
    l1c: f64,
    l1l: f64,
    l1p: f64,
    l1w: f64,
    l1x: f64,
    l2c: f64,
    l2l: f64,
    l2p: f64,
    l2s: f64,
    l2w: f64,
    l2x: f64,
    l2y: f64,
    l5i: f64,
    l5q: f64,
    l5x: f64,
    s1c: f64,
    s1l: f64,
    s1p: f64,
    s1w: f64,
    s1x: f64,
    s2c: f64,
    s2l: f64,
    s2p: f64,
    s2s: f64,
    s2w: f64,
    s2x: f64,
    s2y: f64,
    s5i: f64,
    s5q: f64,
    s5x: f64,
}

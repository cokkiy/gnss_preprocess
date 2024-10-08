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
pub struct IRNSSData {
    c5a: f64,
    c5b: f64,
    c5c: f64,
    c5x: f64,
    c9a: f64,
    c9b: f64,
    c9c: f64,
    c9x: f64,
    d5a: f64,
    d5b: f64,
    d5c: f64,
    d5x: f64,
    d9a: f64,
    d9b: f64,
    d9c: f64,
    d9x: f64,
    l5a: f64,
    l5b: f64,
    l5c: f64,
    l5x: f64,
    l9a: f64,
    l9b: f64,
    l9c: f64,
    l9x: f64,
    s5a: f64,
    s5b: f64,
    s5c: f64,
    s5x: f64,
    s9a: f64,
    s9b: f64,
    s9c: f64,
    s9x: f64,
}

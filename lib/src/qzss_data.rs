use convert_macro::{FieldsPos, FromGnss, FromSlice, FromVec, ToSlice, ToVec, SSC};

#[derive(Clone, Debug, Default, FieldsPos, ToSlice, FromSlice, ToVec, FromVec, FromGnss, SSC)]
pub struct QZSSData {
    c1b: f64,
    c1c: f64,
    c1l: f64,
    c1x: f64,
    c1z: f64,
    c2l: f64,
    c2s: f64,
    c2x: f64,
    c5p: f64,
    c5q: f64,
    c5x: f64,
    c6l: f64,
    c6x: f64,
    c6z: f64,
    d1c: f64,
    d1l: f64,
    d1x: f64,
    d1z: f64,
    d2l: f64,
    d2s: f64,
    d2x: f64,
    d5p: f64,
    d5q: f64,
    d5x: f64,
    d6x: f64,
    d6z: f64,
    l1b: f64,
    l1c: f64,
    l1l: f64,
    l1x: f64,
    l1z: f64,
    l2l: f64,
    l2s: f64,
    l2x: f64,
    l5p: f64,
    l5q: f64,
    l5x: f64,
    l6l: f64,
    l6x: f64,
    l6z: f64,
    s1b: f64,
    s1c: f64,
    s1l: f64,
    s1x: f64,
    s1z: f64,
    s2l: f64,
    s2s: f64,
    s2x: f64,
    s5p: f64,
    s5q: f64,
    s5x: f64,
    s6l: f64,
    s6x: f64,
    s6z: f64,
}

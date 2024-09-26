/// Signal Strength Comparer
///
/// This trait is used to compare the signal strength of gnss items.
///
/// The `compare` method takes a reference to another item and returns a vector of `f64`.
/// The vector should contain the signal strength of the item compared to the other item.
#[allow(dead_code)]
pub trait SignalStrengthComparer {
    /// Compare the signal strength of the item with another item.
    ///
    /// Returns a vector of `f64` value representing the signal strength of the item compared to the other item.
    /// The value represents the signal strength of the item subtract to the other item.
    fn ss_compare(&self, other: &Self) -> Vec<f64>;
}

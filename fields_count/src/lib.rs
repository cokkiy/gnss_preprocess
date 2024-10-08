/// A trait to count the number of fields in a struct.
///
/// # Example
///
/// ```
/// struct MyStruct;
/// impl FieldsCount for MyStruct {
///     fn get_fields_count() -> usize {
///         3
///     }
/// }
///
/// let count = MyStruct::get_fields_count();
/// assert_eq!(count, 3);
/// ```
pub trait FieldsCount {
    /// Returns the number of fields in the struct.
    fn get_fields_count() -> usize;
}

/// A trait to count the number of signal strength fields in a struct.
///
/// # Example
///
/// ```
/// struct MyStruct;
/// impl SignalStrengthFieldsCount for MyStruct {
///     fn get_ss_fields_count() -> usize {
///         2
///     }
/// }
///
/// let count = MyStruct::get_ss_fields_count();
/// assert_eq!(count, 2);
/// ```
pub trait SignalStrengthFieldsCount {
    /// Returns the number of signal strength fields in the struct.
    fn get_ss_fields_count() -> usize;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        struct TestStruct;
        impl FieldsCount for TestStruct {
            fn get_fields_count() -> usize {
                0
            }
        }

        impl SignalStrengthFieldsCount for TestStruct {
            fn get_ss_fields_count() -> usize {
                2
            }
        }
        let result = TestStruct::get_fields_count();
        assert_eq!(result, 0);
        assert_eq!(TestStruct::get_ss_fields_count(), 2);
    }
}

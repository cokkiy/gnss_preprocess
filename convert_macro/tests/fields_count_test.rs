#[cfg(feature = "fields-count")]
#[test]
fn test_fields_count() {
    use convert_macro::FieldsCount;
    use fields_count::AllFieldsCount;
    #[allow(unused)]
    #[derive(FieldsCount)]
    struct TestStruct {
        field1: u32,
        field2: u32,
        field3: u32,
    }
    let result = TestStruct::get_fields_count();
    assert_eq!(result, 3);
}

#[cfg(feature = "fields-count")]
#[test]
fn test_ss_fields_count() {
    use convert_macro::{FieldsCount, SSFieldsCount};
    use fields_count::{AllFieldsCount, SignalStrengthFieldsCount};

    #[allow(unused)]
    #[derive(SSFieldsCount, FieldsCount)]
    struct TestStruct {
        field1: u32,
        field2: u32,
        field3: u32,
        sfield1: f64,
        sfield2: usize,
    }
    let result = TestStruct::get_ss_fields_count();
    assert_eq!(result, 2);
    assert_eq!(TestStruct::get_fields_count(), 5);
}

use convert_macro::FieldsPos;

#[test]
fn test_field_map() {
    #[allow(unused)]
    #[derive(FieldsPos)]
    struct TestStruct {
        field1: u32,
        field2: u32,
        field3: u32,
    }
    assert_eq!(TestStruct::fields_pos().get("field1"), Some(&0));
    assert_eq!(TestStruct::fields_pos().get("field2"), Some(&1));
    assert_eq!(TestStruct::fields_pos().get("field3"), Some(&2));
}

#[test]
fn test_field_pos() {
    #[allow(unused)]
    #[derive(FieldsPos)]
    struct TestStruct {
        crc: f64,
        src: f64,
        dst: f64,
        port: u8,
    }
    assert_eq!(TestStruct::fields_pos().get("crc"), Some(&0));
    assert_eq!(TestStruct::fields_pos().get("src"), Some(&1));
    assert_eq!(TestStruct::fields_pos().get("dst"), Some(&2));
    assert_eq!(TestStruct::fields_pos().get("port"), Some(&3));
}

use convert_macro::{to_vec, FieldsPos};
#[test]
fn test_to_vec_f64() {
    #[allow(unused)]
    #[derive(FieldsPos)]
    #[to_vec(f64)]
    struct TestStruct {
        field1: u32,
        field2: u32,
        field3: u32,
    }

    let instance = TestStruct {
        field1: 1,
        field2: 2,
        field3: 3,
    };

    let vec = Vec::<f64>::from(&instance);
    assert_eq!(vec, vec![1.0, 2.0, 3.0]);
}

#[test]
fn test_to_vec_u32() {
    #[allow(unused)]
    #[derive(FieldsPos)]
    #[to_vec(u32)]
    struct TestStruct {
        field1: u32,
        field2: u32,
        field3: u32,
    }

    let instance = TestStruct {
        field1: 1,
        field2: 2,
        field3: 3,
    };

    let vec = Vec::<u32>::from(&instance);
    assert_eq!(vec, vec![1, 2, 3]);
}

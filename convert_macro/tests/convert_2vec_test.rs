use convert_macro::{FieldsPos, ToVec};

#[test]
fn test_convert_2vec() {
    #[allow(unused)]
    #[derive(FieldsPos, ToVec)]
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
fn test_convert_2vec_2() {
    #[allow(unused)]
    #[derive(FieldsPos, ToVec)]
    struct TestStruct {
        field1: f64,
        field2: f64,
        field3: f64,
        field4: f64,
    }

    let instance = TestStruct {
        field1: 4.0,
        field2: 5.0,
        field3: 6.0,
        field4: 7.0,
    };

    let vec = Vec::<f64>::from(&instance);
    assert_eq!(vec, vec![4.0, 5.0, 6.0, 7.0]);
}

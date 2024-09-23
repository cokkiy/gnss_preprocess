use convert_macro::{to_slice, FieldsPos, ToSlice};

#[test]
fn test_to_slice() {
    #[allow(unused)]
    #[derive(FieldsPos, ToSlice)]
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

    let vec = <[f64; 3]>::from(&instance);
    assert_eq!(&vec[..3], &[1.0, 2.0, 3.0]);
}
#[test]
fn test_to_slice_2() {
    #[allow(unused)]
    #[derive(FieldsPos, ToSlice)]
    struct TestStruct {
        field1: u32,
        field2: u32,
        field3: u32,
        field4: f64,
    }

    let instance = TestStruct {
        field1: 1,
        field2: 2,
        field3: 3,
        field4: 10.00,
    };

    let vec = <[f64; 4]>::from(&instance);
    assert_eq!(&vec[..4], &[1.0, 2.0, 3.0, 10.0]);
}

#[test]
fn test_to_slice_f64() {
    #[allow(unused)]
    #[derive(FieldsPos)]
    #[to_slice(f64)]
    struct TestStruct {
        field1: f64,
        field2: f64,
        field3: f64,
    }

    let instance = TestStruct {
        field1: 1.0,
        field2: 2.0,
        field3: 3.0,
    };

    let vec = <[f64; 3]>::from(&instance);
    assert_eq!(&vec[..3], &[1.0, 2.0, 3.0]);
}

#[test]
fn test_to_slice_u32() {
    #[allow(unused)]
    #[derive(FieldsPos)]
    #[to_slice(u32)]
    struct TestStruct {
        field1: f64,
        field2: f64,
        field3: f64,
    }

    let instance = TestStruct {
        field1: 1.0,
        field2: 2.0,
        field3: 3.0,
    };

    let vec = <[u32; 3]>::from(&instance);
    assert_eq!(&vec[..3], &[1, 2, 3]);
}

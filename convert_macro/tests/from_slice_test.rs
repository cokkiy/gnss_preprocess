#[test]
fn test_from_slice() {
    use convert_macro::{FieldsPos, FromSlice};

    #[derive(Default, FieldsPos, FromSlice)]
    struct Test {
        a: f64,
        b: f64,
    }

    let vec = [1.0, 2.0];
    let test = Test::from(&vec);
    assert_eq!(test.a, 1.0);
    assert_eq!(test.b, 2.0);
}

#[test]
fn test_from_slice2() {
    use convert_macro::{FieldsPos, FromSlice};

    #[derive(Default, FieldsPos, FromSlice)]
    struct Test {
        a: f64,
        b: f64,
        c: f64,
    }

    let vec = [1.0, 2.0, 5.0];
    let test = Test::from(&vec);
    assert_eq!(test.a, 1.0);
    assert_eq!(test.b, 2.0);
    assert_eq!(test.c, 5.0);
}

#[test]
fn test_from_f64() {
    use convert_macro::{from_slice, FieldsPos};

    #[derive(Default, FieldsPos)]
    #[from_slice(f64)]
    struct Test {
        a: f64,
        b: f64,
        c: f64,
    }

    let vec = [1.0, 2.0, 5.0];
    let test = Test::from(&vec);
    assert_eq!(test.a, 1.0);
    assert_eq!(test.b, 2.0);
    assert_eq!(test.c, 5.0);
}

#[test]
fn test_from_i32() {
    use convert_macro::{from_slice, FieldsPos};

    #[derive(Default, FieldsPos)]
    #[from_slice(i32)]
    struct Test {
        a: f64,
        b: f64,
        c: f64,
    }

    let vec = [1, 2, 5];
    let test = Test::from(&vec);
    assert_eq!(test.a, 1.0);
    assert_eq!(test.b, 2.0);
    assert_eq!(test.c, 5.0);
}

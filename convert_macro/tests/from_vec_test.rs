#[test]
fn test_from_vec() {
    use convert_macro::{FieldsPos, FromVec};

    #[derive(Default, FieldsPos, FromVec)]
    struct Test {
        a: f64,
        b: f64,
    }

    let vec = vec![1.0, 2.0];
    let test = Test::from(&vec);
    assert_eq!(test.a, 1.0);
    assert_eq!(test.b, 2.0);
}

#[test]
fn test_from_vec2() {
    use convert_macro::{FieldsPos, FromVec};

    #[derive(Default, FieldsPos, FromVec)]
    struct Test {
        a: f64,
        b: f64,
        c: f64,
    }

    let vec = vec![1.0, 2.0, 5.0];
    let test = Test::from(&vec);
    assert_eq!(test.a, 1.0);
    assert_eq!(test.b, 2.0);
    assert_eq!(test.c, 5.0);
}

#[test]
fn test_from_vec3() {
    use convert_macro::{FieldsPos, FromVec};

    #[derive(Default, FieldsPos, FromVec)]
    struct Test {
        a: f64,
        b: u8,
        c: f64,
        d: u32,
    }

    let vec = vec![1.0, 2.0, 5.0, 7.0];
    let test = Test::from(&vec);
    assert_eq!(test.a, 1.0);
    assert_eq!(test.b, 2);
    assert_eq!(test.c, 5.0);
    assert_eq!(test.d, 7);
}

#[test]
fn test_from_vec_u32() {
    use convert_macro::{from_vec, FieldsPos};

    #[from_vec(u32)]
    #[derive(Default, FieldsPos)]
    struct Test {
        a: u32,
        b: u32,
        c: u32,
    }

    let vec = vec![1, 2, 5];
    let test = Test::from(&vec);
    assert_eq!(test.a, 1);
    assert_eq!(test.b, 2);
    assert_eq!(test.c, 5);
}

#[test]
fn test_from_vec_u32_for_f64() {
    use convert_macro::{from_vec, FieldsPos};

    #[from_vec(u32)]
    #[derive(Default, FieldsPos)]
    struct Test {
        a: f64,
        b: f64,
        c: f64,
    }

    let vec = vec![1, 2, 5];
    let test = Test::from(&vec);
    assert_eq!(test.a, 1.0);
    assert_eq!(test.b, 2.0);
    assert_eq!(test.c, 5.0);
}

#[doc = r#"This crate provides a set of macros to convert structs with named fields into vectors or slices.
The macros can be derived for structs with named fields and generate implementations of the `From` trait
to convert a reference to the struct into a `Vec<f64>`, `Vec<ty>`, `[f64,*]` or `[#ty,*]` slice, where each
field's value is converted to `f64` or type `ty` and placed in the vector or slice according to the field's
position.
Also, the `From` trait can be implemented to convert a reference to a `Vec<f64>` or `[#ty,*]` slice into the struct,
where each field's value is converted to the field's type and placed in the struct according to the field's position.
The slice len must be equal to the field's number.
Additionally, if feature "gnss" enabled, the `From` trait can be implemented to convert a reference to a `HashMap<Observable, ObservationData>`
into the struct, where each field's value is converted to the field's type and placed in the struct according to the
field's name matches the Observable name."#]
mod check_derive;
mod slice;
mod vec;

use proc_macro::TokenStream;
use quote::quote;
use slice::*;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields, FieldsNamed};
use vec::*;

/// # Macros
///
/// ## `FieldsPos`
///
/// This macro can be derived for structs with named fields. It generates an implementation
/// of a method `fields_pos` that returns a `HashMap` mapping field names to their positions
/// within the struct.
///
/// ### Example
///
/// ```rust
/// use convert_macro::FieldsPos;
/// use std::collections::HashMap;
///
/// #[derive(FieldsPos)]
/// struct MyStruct {
///     field1: i32,
///     field2: f64,
/// }
///
/// let positions: HashMap<&'static str, usize> = MyStruct::fields_pos();
/// assert_eq!(positions["field1"], 0);
/// assert_eq!(positions["field2"], 1);
/// ```
///
#[proc_macro_derive(FieldsPos)]
pub fn derive_fields_pos(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let fields = match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { named, .. }),
            ..
        }) => named,
        _ => {
            return TokenStream::from(quote! {
                compile_error!("FieldsPos can only be derived for structs with named fields");
            });
        }
    };

    let field_map = fields.iter().enumerate().map(|(index, field)| {
        let field_name = field.ident.as_ref().unwrap();
        quote! {
            map.insert(stringify!(#field_name), #index);
        }
    });

    let expanded = quote! {
        impl #name {
            pub fn fields_pos() -> std::collections::HashMap<&'static str, usize> {
                let mut map = std::collections::HashMap::new();
                #(#field_map)*
                map
            }
        }
    };

    TokenStream::from(expanded)
}

/// ## `ToVec`
///
/// This macro can be derived for structs with named fields. It generates an implementation
/// of the `From` trait to convert a reference to the struct into a `Vec<f64>`, where each
/// field's value is converted to `f64` and placed in the vector according to the field's
/// position.
///
/// ### Example
///
/// ```rust
/// use convert_macro::{FieldsPos, ToVec};
///
/// #[derive(FieldsPos, ToVec)]
/// struct MyStruct {
///     field1: i32,
///     field2: f64,
/// }
///
/// let my_struct = MyStruct { field1: 42, field2: 3.14 };
/// let vec: Vec<f64> = (&my_struct).into();
/// assert_eq!(vec, vec![42.0, 3.14]);
/// ```
#[proc_macro_derive(ToVec)]
pub fn derive_to_vec(input: TokenStream) -> TokenStream {
    _internal_to_vec(quote! {f64}.into(), input)
}

/// ## `FromVec`
///
/// This macro can be derived for structs with named fields. It generates an implementation
/// of the `From` trait to convert a reference to a `Vec<f64>` into the struct, where each
/// field's value is converted to the field's type and placed in the struct according to the
/// field's position.
///
/// ### Example
///
/// ```rust
/// use convert_macro::{FieldsPos, FromVec};
///
/// #[derive(Default, FieldsPos, FromVec)]
/// struct Test {
///    a: f64,
///    b: f64,
///    c: u32
///   }
///
/// let vec = vec![1.0, 2.0, 5.0];
/// let test = Test::from(&vec);
/// assert_eq!(test.a, 1.0);
/// assert_eq!(test.b, 2.0);
/// assert_eq!(test.c, 5);
/// ```
///
/// ## Note
///
/// The `FromVec` macro can only be derived for structs with named fields and has implemented `Default` trait.
/// Also, the field's type must implement the `From<f64>` trait and the field's number must be equal to the vector's length.
/// The struct need to be derived from `FieldsPos` macro too.
///
#[proc_macro_derive(FromVec)]
pub fn derive_from_vec(input: TokenStream) -> TokenStream {
    _internal_from_vec(quote! {f64}.into(), input)
}

/// ## to_vec(ty)
///
/// This macro can be used for structs with named fields. It generates an implementation
/// of the `From` trait to convert a reference to the struct into a `Vec<ty>`, where each
/// field's value is converted to type `ty` and placed in the vector according to the field's
/// position.
///
/// ### Example
///
/// ```rust
/// use convert_macro::{FieldsPos, to_vec};
///
/// #[derive(FieldsPos)]
/// #[to_vec(f64)]
/// struct MyStruct {
///     field1: i32,
///     field2: f64,
/// }
///
/// let my_struct = MyStruct { field1: 42, field2: 3.14 };
/// let vec: Vec<f64> = (&my_struct).into();
/// assert_eq!(vec, vec![42.0, 3.14]);
/// ```
#[proc_macro_attribute]
pub fn to_vec(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let append = _internal_to_vec(_attr, item.clone());

    let mut result = TokenStream::from(item);
    result.extend(TokenStream::from(append));
    result
}

/// ## from_vec(ty)
///
/// This macro can be used for structs with named fields. It generates an implementation
/// of the `From` trait to convert a reference to a `Vec<#ty>` into the struct, where each
/// field's value is converted to the field's type and placed in the struct according to the
/// field's position.
///
/// ### Example
///
/// ```rust
/// use convert_macro::{FieldsPos, from_vec};
///
/// #[derive(Default, FieldsPos)]
/// #[from_vec(f64)]
/// struct Test {
///     a: f64,
///     b: f64,
///     c: u32
///     }
///
/// let vec = vec![1.0, 2.0, 5.0];
/// let test = Test::from(&vec);
/// assert_eq!(test.a, 1.0);
/// assert_eq!(test.b, 2.0);
/// assert_eq!(test.c, 5);
/// ```
#[proc_macro_attribute]
pub fn from_vec(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let append = _internal_from_vec(_attr, item.clone());

    let mut result = TokenStream::from(item);
    result.extend(TokenStream::from(append));
    result
}

/// ## `ToSlice`
///
/// This macro can be derived for structs with named fields. It generates an implementation
/// of the `From` trait to convert a reference to the struct into a `[f64,*]` slice, where each
/// field's value is converted to `f64` and placed in the vector according to the field's
/// position. The slice len equals to the field's number.
///
/// ### Example
///
/// ```rust
/// use convert_macro::{FieldsPos, ToSlice};
///
/// #[derive(FieldsPos, ToSlice)]
/// struct MyStruct {
///     field1: i32,
///     field2: f64,
/// }
///
/// let my_struct = MyStruct { field1: 42, field2: 3.14 };
/// let vec: <[f64,2]> = (&my_struct).into();
/// assert_eq!(&vec[..2], &[42.0, 3.14]);
/// ```
#[proc_macro_derive(ToSlice)]
pub fn derive_to_slice(input: TokenStream) -> TokenStream {
    _internal_to_slice(quote! {f64}.into(), input)
}

/// ## `FromSlice`
/// This macro can be derived for structs with named fields. It generates an implementation
/// of the `From` trait to convert a reference to a `[f64,*]` slice into the struct, where each
/// field's value is converted to the field's type and placed in the struct according to the
/// field's position. The slice len must be equal to the field's number.
/// ### Example
/// ```rust
/// use convert_macro::{FieldsPos, FromSlice};
/// #[derive(Default, FieldsPos, FromSlice)]
/// struct Test {
///     a: f64,
///     b: f64,
///     }
/// let vec = [1.0, 2.0];
/// let test = Test::from(&vec);
/// assert_eq!(test.a, 1.0);
/// assert_eq!(test.b, 2.0);
/// ```
#[proc_macro_derive(FromSlice)]
pub fn derive_from_slice(input: TokenStream) -> TokenStream {
    _internal_from_slice(quote! {f64}.into(), input)
}

/// ## to_slice(ty)
/// This macro can be used for structs with named fields. It generates an implementation
/// of the `From` trait to convert a reference to the struct into a `[#ty,*]` slice, where each
/// field's value is converted to type `#ty` and placed in the vector according to the field's
/// position. The slice len equals to the field's number.
/// ### Example
/// ```rust
/// use convert_macro::{FieldsPos, to_slice};
/// #[derive(FieldsPos)]
/// #[to_slice(f64)]
/// struct MyStruct {
///    field1: i32,
///    field2: f64,
///    }
/// let my_struct = MyStruct { field1: 42, field2: 3.14 };
/// let vec = <[f64; 2]>::from(&my_struct);
/// assert_eq!(&vec[..2], &[42.0, 3.14]);
/// ```
#[proc_macro_attribute]
pub fn to_slice(attr: TokenStream, item: TokenStream) -> TokenStream {
    let append = _internal_to_slice(attr, item.clone());

    let mut result = TokenStream::from(item);
    result.extend(TokenStream::from(append));
    result
}

/// ## from_slice(ty)
/// This macro can be used for structs with named fields. It generates an implementation
/// of the `From` trait to convert a reference to a `[#ty,*]` slice into the struct, where each
/// field's value is converted to the field's type and placed in the struct according to the
/// field's position. The slice len must be equal to the field's number.
/// ### Example
/// ```rust
/// use convert_macro::{FieldsPos, from_slice};
/// #[derive(Default, FieldsPos)]
/// #[from_slice(f64)]
/// struct Test {
///    a: f64,
///    b: f64,
///    }
/// let vec = [1.0, 2.0];
/// let test = Test::from(&vec);
/// assert_eq!(test.a, 1.0);
/// assert_eq!(test.b, 2.0);
/// ```
#[proc_macro_attribute]
pub fn from_slice(attr: TokenStream, item: TokenStream) -> TokenStream {
    let append = _internal_from_slice(attr, item.clone());

    let mut result = TokenStream::from(item);
    result.extend(TokenStream::from(append));
    result
}

/// ## `FromGnss`
/// This macro can be derived for structs with named fields. It generates an implementation
/// of the `From` trait to convert a reference to a `HashMap<Observable, ObservationData>`
/// into the struct, where each field's value is converted to the field's type and placed in the struct according to the
/// field's name matches the Observable name.
/// ### Example
/// ```rust
/// use convert_macro::FromGnss;
/// use std::collections::HashMap;
/// use rinex::{
///     observation::{LliFlags, ObservationData},
///     prelude::Observable,
///     };
/// #[derive(Default, FromGnss)]
/// struct TestStruct {
///     c1c: f64,
///     l1c: f64,
///     d1c: f64,
///     s1c: f64,
///     }
/// let mut data: HashMap<Observable, ObservationData> = HashMap::new();
/// data.insert(
///    Observable::PseudoRange("c1c".to_string()),
///    ObservationData::new(
///         1.0,
///         Some(LliFlags::OK_OR_UNKNOWN),
///         Some(rinex::observation::SNR::DbHz0),
///         ),
///     );
/// data.insert(
///     Observable::Phase("l1c".to_string()),
///     ObservationData::new(
///         2.0,
///         Some(LliFlags::OK_OR_UNKNOWN),
///         Some(rinex::observation::SNR::DbHz0),
///         ),
///     );
/// data.insert(
///     Observable::Doppler("d1c".to_string()),
///     ObservationData::new(
///         3.0,
///         Some(LliFlags::OK_OR_UNKNOWN),
///         Some(rinex::observation::SNR::DbHz0),
///         ),
///     );
/// data.insert(
///      Observable::SSI("s1c".to_string()),
///      ObservationData::new(
///         4.0,
///         Some(LliFlags::OK_OR_UNKNOWN),
///         Some(rinex::observation::SNR::DbHz0),
///         ),
///     );
/// let test_struct: TestStruct = (&data).into();
/// assert!(test_struct.c1c == 1.0);
/// assert!(test_struct.l1c == 2.0);
/// assert!(test_struct.d1c == 3.0);
/// assert!(test_struct.s1c == 4.0);
/// ```
/// ## Note
/// The `FromGnss` macro can only be derived for structs with named fields and has implemented `Default` trait.
///
#[cfg(feature = "gnss")]
#[proc_macro_derive(FromGnss)]
pub fn derive_from_hashmap(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let fields = match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { named, .. }),
            ..
        }) => named,
        _ => {
            return TokenStream::from(quote! {
                compile_error!("This macro can only be derived for structs with named fields");
            });
        }
    };

    let field_idents: Vec<_> = fields.iter().map(|f| f.ident.as_ref().unwrap()).collect();
    let field_types: Vec<_> = fields.iter().map(|f| &f.ty).collect();
    let expanded = quote! {
        impl From<&std::collections::HashMap<
                rinex::prelude::Observable,
                rinex::observation::ObservationData,
            >> for #name {
            fn from(value: &std::collections::HashMap<
                rinex::prelude::Observable,
                rinex::observation::ObservationData,
            >) -> Self {
                fn get_observable_field_name(observable: &rinex::prelude::Observable) -> Option<&str> {
                    match observable {
                        rinex::prelude::Observable::Phase(name) => Some(name),
                        rinex::prelude::Observable::Doppler(name) => Some(name),
                        rinex::prelude::Observable::SSI(name) => Some(name),
                        rinex::prelude::Observable::PseudoRange(name) => Some(name),
                        _ => None,
                    }
                }
                let mut _self= Self::default();
                #(
                    let v = value
                        .iter()
                        .find(|(obs, _)| get_observable_field_name(obs) == Some(stringify!(#field_idents)));
                    if let Some((_, data)) = v {
                        _self.#field_idents = data.obs as #field_types;
                    }
                )*
                _self
            }
        }
    };

    TokenStream::from(expanded)
}

/// ## `SSC`
/// This macro can be derived for structs with named fields. It generates an implementation
/// of the `SignalStrengthComparer` trait to compare the signal strength of two structs.
/// ### Example
/// ```rust
/// use convert_macro::SSC;
/// use ssc::SignalStrengthComparer;
/// #[derive(SSC)]
/// struct TestStruct {
///     c1c: f64,
///     c1l: f64,
///     s1c: f64,
///     s1l: f64,
///     s1p: f64,
///     }
/// let test1 = TestStruct {
///     c1c: 1.0,
///     c1l: 3.0,
///     s1c: 2.0,
///     s1l: 4.0,
///     s1p: 5.0,
///     };
/// let test2 = TestStruct {
///     c1c: 2.0,
///     c1l: 4.0,
///     s1c: 3.0,
///     s1l: 5.0,
///     s1p: 6.0,
///     };
/// let result = test1.ss_compare(&test2);
/// assert_eq!(result, vec![1.0, 1.0, 1.0, 1.0, 1.0]);
/// ```
/// ## Note
/// The `SSC` macro in feature "gnss-ssc".
#[cfg(feature = "gnss-ssc")]
#[proc_macro_derive(SSC)]
pub fn derive_ssc(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let fields = match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { named, .. }),
            ..
        }) => named,
        _ => {
            return TokenStream::from(quote! {
                compile_error!("This macro can only be derived for structs with named fields");
            });
        }
    };

    let field_idents: Vec<_> = fields
        .iter()
        .filter(|f| f.ident.as_ref().unwrap().to_string().starts_with("s"))
        .map(|f| f.ident.as_ref().unwrap())
        .collect();
    let len = field_idents.len();
    let expanded = quote! {

        impl ssc::SignalStrengthComparer for #name {
            fn ss_compare(&self, other: &Self) -> Vec<f64> {
                let mut result = Vec::with_capacity(#len);
                #(
                    result.push((self.#field_idents - other.#field_idents).round() as f64);
                )*

                result
            }
        }
    };

    TokenStream::from(expanded)
}

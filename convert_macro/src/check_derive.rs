use syn::DeriveInput;
#[allow(unused)]
pub(super) fn check_macro_derived(input: &DeriveInput, macro_name: &str) -> bool {
    // Prepare to store the result.
    let mut derives_another_macro = false;
    // Inspect the `derive` attributes to see if it contains the target derive macro.
    if let Some(attr) = input.attrs.iter().find(|a| a.path().is_ident("derive")) {
        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident(macro_name) {
                derives_another_macro = true;
            }
            Ok(())
        });
    }
    derives_another_macro
}

#[cfg(test)]
mod tests {
    use quote::quote;
    use syn::DeriveInput;

    use crate::check_derive::check_macro_derived;

    #[test]
    fn test_check_macro_derived_found() {
        let input = quote! {
            #[allow(unused)]
            #[derive(FieldsPos, ToSlice)]
            struct TestStruct {
                field1: u32,
                field2: u32,
                field3: u32,
                field4: f64,
            }
        };

        let input = syn::parse2::<DeriveInput>(input);
        assert!(input.is_ok());
        let found = check_macro_derived(input.as_ref().unwrap(), "FieldsPos");
        assert!(found);
    }

    #[test]
    fn test_check_macro_derived_no_found() {
        let input = quote! {
            #[allow(unused)]
            #[derive(ToSlice)]
            struct TestStruct {
                field1: u32,
                field2: u32,
                field3: u32,
                field4: f64,
            }
        };

        let input = syn::parse2::<DeriveInput>(input);
        assert!(input.is_ok());
        let found = check_macro_derived(input.as_ref().unwrap(), "FieldsPos");
        assert!(!found);
    }
}

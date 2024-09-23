use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields, FieldsNamed};

pub(super) fn _internal_to_slice(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let ty = parse_macro_input!(_attr as syn::Type);
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
    let len = field_idents.len();
    let expanded = quote! {
        impl From<&#name> for [#ty;#len] {
            fn from(value: &#name) -> Self {
                let mut vec = [0.0 as #ty; #len];
                #(
                    vec[#name::fields_pos()[stringify!(#field_idents)]] = value.#field_idents as #ty;
                )*
                vec
            }
        }
    };

    TokenStream::from(expanded)
}

pub(super) fn _internal_from_slice(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let ty = parse_macro_input!(_attr as syn::Type);
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
    let len = field_idents.len();
    let expanded = quote! {
        impl From<&[#ty;#len]> for #name {
            fn from(value: &[#ty;#len]) -> Self {
                let mut _self= Self::default();
                #(
                    _self.#field_idents= value[#name::fields_pos()[stringify!(#field_idents)]] as #field_types;
                )*
                _self
            }
        }
    };

    TokenStream::from(expanded)
}

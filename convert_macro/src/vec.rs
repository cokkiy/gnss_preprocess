use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields, FieldsNamed};

#[inline]
pub(super) fn _internal_from_vec(_attr: TokenStream, input: TokenStream) -> TokenStream {
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
    let expanded = quote! {
        impl From<&Vec<#ty>> for #name {
            fn from(value: &Vec<#ty>) -> Self {
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

#[inline]
pub(super) fn _internal_to_vec(_attr: TokenStream, input: TokenStream) -> TokenStream {
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
    let expanded = quote! {
        impl From<&#name> for Vec<#ty> {
            fn from(value: &#name) -> Self {
                let len = #name::fields_pos().len();
                let mut vec = vec![0.0 as #ty; len];
                #(
                    vec[#name::fields_pos()[stringify!(#field_idents)]] = value.#field_idents as #ty;
                )*
                vec
            }
        }
    };

    TokenStream::from(expanded)
}

pub(super) fn _internal_to_compact(_attr: TokenStream, input: TokenStream) -> TokenStream {
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
    let expanded = quote! {
        impl From<&#name> for Vec<#ty> {
            fn from(value: &#name) -> Self {
                let len = #name::fields_pos().len();
                let mut vec = Vec::<#ty>::new();
                #(
                    if value.#field_idents != 0{
                        vec.push(value.#field_idents as #ty);
                    }
                )*
                vec
            }
        }
    };

    TokenStream::from(expanded)
}

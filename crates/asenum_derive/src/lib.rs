//! Small derive macro to safely convert the numerical value of an enum to the enum.

#![no_std]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

/// Generates an implementation of `asenum::Convert`.
///
/// Requires each enum variant to have an explicit value.
///
/// Example of generated code:
///
/// ```
/// enum Foo {
///     Bar = 1,
///     Baz = 2,
/// }
///
/// impl Convert for Foo {
///     fn from_value(value: usize) -> Option<Self> {
///         match value {
///             1 => Some(Self::Bar),
///             2 => Some(Self::Baz),
///             _ => None,
///         }
///     }
/// }
/// ```
#[proc_macro_derive(Convert)]
pub fn convert(s: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(s as DeriveInput);
    let data = match data {
        Data::Enum(data) => data,
        _ => panic!("ToEnum can only be applied to enums"),
    };
    let arms = data.variants.into_iter().map(|variant| {
        let loc_ident = variant.ident;

        let expr = variant
            .discriminant
            .expect("ToEnum field variants must have a discriminant")
            .1;

        quote! {
            #expr => Some(#ident::#loc_ident),
        }
    });

    quote! {
        impl ::asenum::Convert for #ident {
            fn from_value(value: usize) -> Option<Self> {
                match value {
                    #(#arms)*
                    _ => None,
                }
            }
        }
    }
    .into()
}

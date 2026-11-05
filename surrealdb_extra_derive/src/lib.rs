mod common;
#[cfg(feature = "repr")]
mod repr;
#[cfg(feature = "table")]
mod table;

#[cfg(feature = "table")]
use crate::table::name::{get_crate_path, get_table_name};

use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[cfg(feature = "table")]
#[proc_macro_derive(Table, attributes(table))]
pub fn table(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;
    let (table_name, krate) =
        match get_table_name(&input).and_then(|n| Ok((n, get_crate_path(&input)?))) {
            Ok(v) => v,
            Err(e) => return e.into_compile_error().into(),
        };

    let expanded = quote! {
        #[automatically_derived]
        impl #krate::table::Table for #struct_name {
            const TABLE_NAME: &'static str = #table_name;
        }
    };

    TokenStream::from(expanded)
}

#[cfg(feature = "repr")]
#[proc_macro_derive(SurrealValueRepr, attributes(surreal_repr))]
pub fn surreal_value_repr(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    repr::expand(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

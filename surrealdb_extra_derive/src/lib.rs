mod table;
mod common;
mod surreal_value_json;

use crate::surreal_value_json::kind::get_kind;
use crate::table::name::get_table_name;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[cfg(feature = "table")]
#[proc_macro_derive(Table, attributes(table))]
pub fn table(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;
    let table_name = get_table_name(&input).unwrap();

    let expanded = quote! {
        impl Table for #struct_name {
            const TABLE_NAME: &'static str = #table_name;
        }
    };

    TokenStream::from(expanded)
}

#[cfg(feature = "surreal_value_json")]
#[proc_macro_derive(SurrealValueJson, attributes(surreal_value_json))]
pub fn surreal_json(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // let root = crate
    let struct_name = &input.ident;
    let kind_str = get_kind(&input).unwrap_or("Object".to_string());

    // Parse the string as actual Rust code
    let kind_tokens: proc_macro2::TokenStream = kind_str.parse().expect("Invalid Rust syntax in kind");

    let expanded = quote! {
        impl SurrealValue for #struct_name {
            fn kind_of() -> surrealdb_extra::surreal_value_json::Kind {
                surrealdb_extra::surreal_value_json::Kind::#kind_tokens
            }

            fn into_value(self) -> surrealdb_extra::surreal_value_json::Value {
                surrealdb_extra::surreal_value_json::json!(self).into_value()
            }

            fn from_value(value: surrealdb_extra::surreal_value_json::Value) -> surrealdb_extra::surreal_value_json::anyhow::Result<Self>
            where
                Self: Sized
            {
                let val = surrealdb_extra::surreal_value_json::from_value(value.into_json_value())?;
                Ok(val)
            }
        }
    };

    TokenStream::from(expanded)
}

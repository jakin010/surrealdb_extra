mod table_name;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};
use crate::table_name::get_table_name;

#[proc_macro_derive(Table, attributes(table))]
pub fn table(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;
    let table_name = get_table_name(&input).unwrap();

    let expanded = quote! {
        impl Table for #struct_name {
            const TABLE_NAME: &'static str = #table_name;

            fn get_id(&self) -> &Option<::surrealdb::opt::RecordId> {
                &self.id
            }

            fn set_id(&mut self, id: impl Into<String>) {
                self.id = Some(::surrealdb::opt::RecordId::from((Self::TABLE_NAME.to_string(), id.into())));
            }
        }
    };

    TokenStream::from(expanded)
}

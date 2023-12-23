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

            fn table_name() -> String {
                #table_name.to_string()
            }

            fn get_id(&self) -> &Option<::surrealdb::sql::Thing> {
                &self.id
            }

            fn set_id(&mut self, id: impl Into<String>) {
                self.id = Some(::surrealdb::sql::Thing::from((Self::TABLE_NAME.to_string(), id.into())));
            }
        }
    };

    TokenStream::from(expanded)
}

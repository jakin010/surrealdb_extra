mod table_name;
mod field_names;


extern crate proc_macro;

use ::proc_macro::TokenStream;
use ::quote::quote;
use ::syn::{parse_macro_input, DeriveInput};
use crate::field_names::get_field_names;
use crate::table_name::get_table_name;

#[proc_macro_derive(Table, attributes(table))]
pub fn table(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;
    let table_name = get_table_name(&input).unwrap();
    let field_names = get_field_names(&input).unwrap();

    let expanded = quote! {
        impl Table for #struct_name {
            fn table_name() -> String {
                #table_name.to_string()
            }

            fn get_id(&self) -> &Option<::surrealdb::sql::Thing> {
               &self.id
            }

            fn set_id(&mut self, id: ::surrealdb::sql::Thing) {
                self.id = Some(id);
            }

            fn fields() -> Vec<&'static str> {
                #field_names.split(",").collect()
            }
        }
    };

    TokenStream::from(expanded)
}

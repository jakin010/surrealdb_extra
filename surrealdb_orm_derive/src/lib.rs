extern crate proc_macro;


use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Meta, Token};
use syn::punctuated::Punctuated;

#[proc_macro_derive(Table, attributes(table))]
pub fn table(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // println!("input: {:#?}", input);

    let struct_name = &input.ident;
    let collection_name = get_table_name(&input).unwrap();

    let expanded = quote! {
        impl Table for #struct_name {
            fn name() -> String {
                #collection_name.to_string()
            }
        }
    };

    TokenStream::from(expanded)
}

fn get_table_name(input: &DeriveInput) -> Option<String> {
    for attr in &input.attrs {
        if attr.path().is_ident("table") {
            let nested = attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated).unwrap();

            for meta in nested {
                match meta {
                    Meta::NameValue(nv) if nv.path.is_ident("name") => {
                        match nv.value {

                            _ => { return None; }
                        }
                    }
                    _ => { return None; }
                }
            }
        }
    }

    Some("test".to_string())
}
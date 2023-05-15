extern crate proc_macro;


use ::proc_macro::TokenStream;
use ::quote::quote;
use ::syn::{parse_macro_input, DeriveInput, Meta, Token, Expr, Lit};
use ::syn::punctuated::Punctuated;

#[proc_macro_derive(Table, attributes(table))]
pub fn table(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // println!("input: {:#?}", input);

    let struct_name = &input.ident;
    let table_name = get_table_name(&input).unwrap();
//    let table_id = get_table_id(&input).unwrap();

    let expanded = quote! {
        impl Table for #struct_name {
            fn name() -> String {
                #table_name.to_string()
            }

            fn id(&self) -> ::surrealdb::sql::Thing {
                ::surrealdb::sql::Thing::from(("test", "test"))
            }
        }
    };

    TokenStream::from(expanded)
}

fn get_table_name(input: &DeriveInput) -> Option<String> {
    let attr_name = "name";
    for attr in &input.attrs {
        if attr.path().is_ident("table") {
            let nested = attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated).unwrap();
            for meta in nested {
                match meta {
                    Meta::NameValue(nv) if nv.path.is_ident(attr_name) => {
                        match nv.value {
                            Expr::Lit(expr_lit) => {
                                match expr_lit.lit {
                                    Lit::Str(lit) => {
                                        let v = lit.value();

                                        if !v.chars().nth(0).unwrap().is_alphabetic() {
                                            panic!("table({}) attribute first character must be alphabetic", attr_name);
                                        }

                                        if v.contains(char::is_whitespace) {
                                            panic!("table({}) attribute must not have any whitespace", attr_name);
                                        }

                                        if !v.chars().all(|x| x.is_alphanumeric() || "_".contains(x)) {
                                            panic!("table({}) attribute can only have alphanumeric and/or `_` characters", attr_name);
                                        }

                                        return Some(v);
                                    }
                                    _ => { return None; }
                                }
                            }
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
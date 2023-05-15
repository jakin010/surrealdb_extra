use ::syn::{DeriveInput, Meta, Token, Expr, Lit};use ::syn::punctuated::Punctuated;
use syn::__private::Span;
use syn::Error;

pub(crate) fn get_table_name(input: &DeriveInput) -> Result<String, Error> {
    let attr_name = "name";
    for attr in &input.attrs {
        if attr.path().is_ident("table") {
            let nested = attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated).unwrap();
            for meta in nested {
                let v = meta.require_name_value().and_then(|mnv| {

                    if !mnv.path.is_ident(attr_name) {
                        return Err(Error::new(Span::call_site(), "name attribute is missing"));
                    }

                     let expr_lit = match &mnv.value {
                         Expr::Lit(expr_lit) => expr_lit,
                         _ => { return Err(Error::new(Span::call_site(), "Wrong expression")); }
                     };

                    let lit = match &expr_lit.lit {
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

                            v
                        }
                        _ => { return Err(Error::new(Span::call_site(), "Wrong type")); }
                    };

                    Ok(lit)
                })?;

                return Ok(v);
            }
        }
    }

    Err(Error::new(Span::call_site(), "Something went wrong"))
}

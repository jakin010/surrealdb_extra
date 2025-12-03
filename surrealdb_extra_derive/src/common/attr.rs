use syn::{
    spanned::Spanned, Attribute, Error, Expr, Lit, Meta, Token,
    punctuated::Punctuated,
};

pub(crate) fn get_attr_value(
    attrs: &[Attribute],
    attr_ident: &str,
    key_ident: &str
) -> Result<Option<String>, Error> {
    for attr in attrs {
        if attr.path().is_ident(attr_ident) {

            let nested = attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;

            for meta in nested {
                if let Meta::NameValue(mnv) = meta {
                    if mnv.path.is_ident(key_ident) {
                        if let Expr::Lit(expr_lit) = &mnv.value {
                            if let Lit::Str(lit_str) = &expr_lit.lit {
                                return Ok(Some(lit_str.value()));
                            }
                        }

                        return Err(Error::new(
                            mnv.value.span(),
                            format!("Expected a string literal for `{}`", key_ident)
                        ));
                    }
                }
            }
        }
    }

    Ok(None)
}

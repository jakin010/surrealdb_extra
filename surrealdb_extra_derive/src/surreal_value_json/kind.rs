use crate::common::attr::get_attr_value;
use syn::spanned::Spanned;
use syn::{DeriveInput, Error};

pub(crate) fn get_kind(input: &DeriveInput) -> Result<String, Error> {
    let kind = match get_attr_value(&input.attrs, "surreal_value_json", "kind")? {
        Some(n) => n,
        None => {
            return Err(Error::new(
                input.span(),
                "Missing #[surreal_value_json(kind = \"...\")]",
            ));
        }
    };

    Ok(kind)
}

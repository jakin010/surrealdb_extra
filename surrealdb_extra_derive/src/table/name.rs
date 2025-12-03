use crate::common::attr::get_attr_value;
use syn::spanned::Spanned;
use syn::{DeriveInput, Error};

pub(crate) fn get_table_name(input: &DeriveInput) -> Result<String, Error> {
    let table_name = match get_attr_value(&input.attrs, "table", "name")? {
        Some(n) => n,
        None => return Err(Error::new(input.span(), "Missing #[table(name = \"...\")]")),
    };

    Ok(table_name)
}

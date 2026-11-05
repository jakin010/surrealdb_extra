use proc_macro_crate::{FoundCrate, crate_name};
use proc_macro2::Span;
use syn::{Ident, Path};

/// Path to the `surrealdb_extra` crate as the calling crate names it.
///
/// Reads the caller's `Cargo.toml`, so a renamed dependency
/// (`extra = { package = "surrealdb_extra" }`) resolves to `::extra`.
/// Falls back to `::surrealdb_extra` when it can't be found, for example when the
/// derive is reached through another crate's re-export.
pub(crate) fn surrealdb_extra_path() -> Path {
    let name = match crate_name("surrealdb_extra") {
        Ok(FoundCrate::Name(name)) => name,
        // `Itself` also covers this crate's own tests and doctests, which link it by name.
        Ok(FoundCrate::Itself) | Err(_) => "surrealdb_extra".to_string(),
    };
    let ident = Ident::new(&name, Span::call_site());
    syn::parse_quote!(::#ident)
}

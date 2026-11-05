//! Implementation of `#[derive(SurrealValueRepr)]`.
//!
//! Implements `SurrealValue` for a fieldless `#[repr(int)]` enum, storing it as its
//! integer discriminant (like `serde_repr`, but without serde).

use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Error, Fields, Ident, LitStr, Path, Result};

#[derive(Clone, Copy, PartialEq)]
enum KindMode {
    Literal,
    Int,
    Number,
}

struct Container {
    krate: Path,
    kind: Option<KindMode>,
}

/// Reprs that have a `SurrealValue` impl upstream. `i128`/`u128` do not.
const SUPPORTED_REPRS: &[&str] = &[
    "u8", "u16", "u32", "u64", "usize", "i8", "i16", "i32", "i64", "isize",
];

fn parse_container(input: &DeriveInput) -> Result<Container> {
    let mut krate: Path = syn::parse_quote!(::surrealdb::types);
    let mut kind = None;
    for attr in input
        .attrs
        .iter()
        .filter(|a| a.path().is_ident("surreal_repr"))
    {
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("crate") {
                let s: LitStr = meta.value()?.parse()?;
                krate = s.parse()?;
                Ok(())
            } else if meta.path.is_ident("kind") {
                let s: LitStr = meta.value()?.parse()?;
                kind = Some(match s.value().as_str() {
                    "literal" => KindMode::Literal,
                    "int" => KindMode::Int,
                    "number" => KindMode::Number,
                    _ => {
                        return Err(Error::new_spanned(
                            s,
                            "expected one of \"literal\", \"int\", \"number\"",
                        ));
                    }
                });
                Ok(())
            } else if meta.path.is_ident("other") {
                Err(meta.error("#[surreal_repr(other)] belongs on a variant, not the enum"))
            } else {
                Err(meta
                    .error("unknown surreal_repr container attribute; expected `crate` or `kind`"))
            }
        })?;
    }
    Ok(Container { krate, kind })
}

/// Finds the integer repr, like serde_repr does.
fn parse_repr(input: &DeriveInput) -> Result<Ident> {
    let mut found: Option<Ident> = None;
    for attr in input.attrs.iter().filter(|a| a.path().is_ident("repr")) {
        attr.parse_nested_meta(|meta| {
            let Some(ident) = meta.path.get_ident().cloned() else {
                return Err(meta.error("unsupported repr"));
            };
            let name = ident.to_string();
            if SUPPORTED_REPRS.contains(&name.as_str()) {
                found = Some(ident);
                Ok(())
            } else if name == "i128" || name == "u128" {
                Err(meta.error(
                    "#[repr(i128)] / #[repr(u128)] is not supported: surrealdb has no SurrealValue impl for 128-bit integers",
                ))
            } else if name == "C" || name == "transparent" {
                // Ignored (rustc itself rejects `repr(C, u8)` on fieldless enums, E0566); plain `repr(C)` is caught below as "missing".
                Ok(())
            } else if name == "align" || name == "packed" {
                if meta.input.peek(syn::token::Paren) {
                    let _content;
                    syn::parenthesized!(_content in meta.input);
                    let _ = _content.parse::<TokenStream2>()?;
                }
                Ok(())
            } else {
                Err(meta.error("unsupported repr for SurrealValueRepr"))
            }
        })?;
    }
    found.ok_or_else(|| {
        Error::new(
            Span::call_site(),
            "SurrealValueRepr requires an integer repr, e.g. #[repr(u8)] or #[repr(i32)]",
        )
    })
}

fn is_other(variant: &syn::Variant) -> Result<bool> {
    let mut other = false;
    for attr in variant
        .attrs
        .iter()
        .filter(|a| a.path().is_ident("surreal_repr"))
    {
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("other") {
                other = true;
                Ok(())
            } else {
                Err(meta.error("unknown surreal_repr variant attribute; expected `other`"))
            }
        })?;
    }
    Ok(other)
}

pub(crate) fn expand(input: DeriveInput) -> Result<TokenStream2> {
    let Data::Enum(data) = &input.data else {
        return Err(Error::new(
            Span::call_site(),
            "SurrealValueRepr can only be derived for enums",
        ));
    };
    if !input.generics.params.is_empty() || input.generics.where_clause.is_some() {
        return Err(Error::new_spanned(
            &input.generics,
            "SurrealValueRepr does not support generic enums",
        ));
    }
    if data.variants.is_empty() {
        return Err(Error::new(
            Span::call_site(),
            "SurrealValueRepr requires at least one variant",
        ));
    }

    let container = parse_container(&input)?;
    let repr = parse_repr(&input)?;

    let mut variants = Vec::new();
    let mut other: Option<Ident> = None;
    for v in &data.variants {
        if !matches!(v.fields, Fields::Unit) {
            return Err(Error::new_spanned(
                v,
                "SurrealValueRepr requires unit variants (no fields)",
            ));
        }
        if is_other(v)? {
            if other.is_some() {
                return Err(Error::new_spanned(
                    v,
                    "only one variant can be #[surreal_repr(other)]",
                ));
            }
            other = Some(v.ident.clone());
        }
        variants.push(v.ident.clone());
    }

    let ident = &input.ident;
    let name = ident.to_string();
    let t = &container.krate;
    // u64/usize discriminants may exceed i64::MAX; those are stored as whole Decimal by the
    // upstream `u64::into_value`, so `Kind::Int` / `Literal(Integer)` would not describe them.
    let may_exceed_i64 = matches!(repr.to_string().as_str(), "u64" | "usize");

    if other.is_some() && container.kind == Some(KindMode::Literal) {
        return Err(Error::new(
            Span::call_site(),
            "#[surreal_repr(other)] cannot be combined with kind = \"literal\": the schema would reject the values `other` is meant to catch",
        ));
    }
    let mode = container.kind.unwrap_or(if other.is_some() {
        KindMode::Int
    } else {
        KindMode::Literal
    });

    // Hidden consts in the serde_repr style: rustc evaluates
    // `Enum::V as Repr`, so implicit, negative and const-expression discriminants all work.
    let cname: Vec<Ident> = (0..variants.len())
        .map(|i| format_ident!("__V{}", i))
        .collect();
    let consts = variants.iter().zip(&cname).map(|(v, c)| {
        quote! { const #c: #repr = #ident::#v as #repr; }
    });
    let disc = format_ident!("__SurrealValueReprDiscriminant");
    let disc_struct = quote! {
        #[allow(dead_code)]
        struct #disc;
        #[allow(dead_code)]
        impl #disc { #(#consts)* }
    };
    let arms = variants
        .iter()
        .zip(&cname)
        .map(|(v, c)| quote! { #disc::#c => ::core::result::Result::Ok(#ident::#v), });
    let disc_list = cname.iter().map(|c| quote! { #disc::#c });

    let kind_body = match mode {
        KindMode::Int if may_exceed_i64 => quote! { #t::Kind::Number },
        KindMode::Int => quote! { #t::Kind::Int },
        KindMode::Number => quote! { #t::Kind::Number },
        KindMode::Literal => {
            let lits = cname.iter().map(|c| {
                // Runtime-checked: a u64 discriminant > i64::MAX does not fit
                // KindLiteral::Integer(i64), so it degrades to Kind::Number.
                quote! {
                    match <i64 as ::core::convert::TryFrom<#repr>>::try_from(#disc::#c) {
                        ::core::result::Result::Ok(__i) => #t::Kind::Literal(#t::KindLiteral::Integer(__i)),
                        ::core::result::Result::Err(_) => #t::Kind::Number,
                    }
                }
            });
            if variants.len() == 1 {
                quote! { #(#lits)* }
            } else {
                quote! { #t::Kind::Either(::std::vec![#(#lits),*]) }
            }
        }
    };

    let fallback_arm = match &other {
        Some(o) => quote! { _ => ::core::result::Result::Ok(#ident::#o), },
        None => quote! {
            __other => {
                let __expected: ::std::vec::Vec<::std::string::String> =
                    [#(#disc_list),*].iter().map(|__d| ::std::string::ToString::to_string(__d)).collect();
                ::core::result::Result::Err(
                    #t::OutOfRangeError::new(__other, #name)
                        .with_context(::std::format!("expected one of: {}", __expected.join(", ")))
                        .into(),
                )
            }
        },
    };

    let into_arms = variants
        .iter()
        .zip(&cname)
        .map(|(v, c)| quote! { #ident::#v => #disc::#c, });

    Ok(quote! {
        #[automatically_derived]
        #[allow(deprecated)]
        impl #t::SurrealValue for #ident {
            fn kind_of() -> #t::Kind {
                #disc_struct
                #kind_body
            }

            fn is_value(__value: &#t::Value) -> bool {
                // Don't use the default `is_kind(kind_of())`: Literal(Integer(n)) also matches
                // Float(n.0), which from_value rejects. Cheap pre-check avoids cloning non-numbers.
                ::core::matches!(__value, #t::Value::Number(_))
                    && <Self as #t::SurrealValue>::from_value(::core::clone::Clone::clone(__value)).is_ok()
            }

            fn into_value(self) -> #t::Value {
                #disc_struct
                // Delegate to the repr: Number::Int when it fits in i64, whole Decimal otherwise
                // (u64/usize > i64::MAX), same as a plain integer field. `match` so no Copy bound.
                let __d: #repr = match self { #(#into_arms)* };
                <#repr as #t::SurrealValue>::into_value(__d)
            }

            fn from_value(__value: #t::Value) -> ::core::result::Result<Self, #t::Error> {
                #disc_struct
                // Pre-check so non-numbers and floats get "Expected 0 | 1 | 7, got number (Enum)"
                // instead of upstream's confusing "Expected number, got number".
                if !::core::matches!(
                    __value,
                    #t::Value::Number(ref __n) if !::core::matches!(
                        __n,
                        #t::Number::Float(_) | #t::Number::F32(_) | #t::Number::F16(_)
                    )
                ) {
                    return ::core::result::Result::Err(
                        #t::ConversionError::from_value(<Self as #t::SurrealValue>::kind_of(), &__value)
                            .with_context(#name)
                            .into(),
                    );
                }
                // Width / Decimal / Float / range handling comes from the repr's own impl.
                match <#repr as #t::SurrealValue>::from_value(__value)? {
                    #(#arms)*
                    #fallback_arm
                }
            }
        }
    })
}

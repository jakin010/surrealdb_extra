use crate::common::attr::get_attr_value;
use syn::{DeriveInput, Error};

/// Returns `#[table(name = "...")]` if set, otherwise the struct name in snake_case.
pub(crate) fn get_table_name(input: &DeriveInput) -> Result<String, Error> {
    match get_attr_value(&input.attrs, "table", "name")? {
        Some(name) => Ok(name),
        None => {
            // `r#type` becomes `type`.
            let ident = input.ident.to_string();
            let ident = ident.strip_prefix("r#").unwrap_or(&ident);
            Ok(to_snake_case(ident))
        }
    }
}

/// `UserProfile` -> `user_profile`, `HTTPServer` -> `http_server`, `User2FA` -> `user2_fa`.
///
/// A word starts at an uppercase letter that follows a lowercase letter or digit,
/// or at the last uppercase letter of an acronym when a lowercase letter follows.
fn to_snake_case(ident: &str) -> String {
    let chars: Vec<char> = ident.chars().collect();
    let mut out = String::with_capacity(ident.len() + 4);

    for (i, &c) in chars.iter().enumerate() {
        if c.is_uppercase() && i > 0 {
            let prev = chars[i - 1];
            let next_is_lower = chars.get(i + 1).is_some_and(|n| n.is_lowercase());
            let starts_word = prev.is_lowercase()
                || prev.is_ascii_digit()
                || (prev.is_uppercase() && next_is_lower);
            if starts_word && !out.ends_with('_') {
                out.push('_');
            }
        }
        out.extend(c.to_lowercase());
    }

    out
}

#[cfg(test)]
mod tests {
    use super::to_snake_case;

    #[test]
    fn snake_case() {
        for (input, expected) in [
            ("User", "user"),
            ("UserProfile", "user_profile"),
            ("HTTPServer", "http_server"),
            ("ApiV2", "api_v2"),
            ("User2FA", "user2_fa"),
            ("IOError", "io_error"),
            ("ABC", "abc"),
            ("already_snake", "already_snake"),
            ("Snake_Case", "snake_case"),
            ("X", "x"),
        ] {
            assert_eq!(to_snake_case(input), expected, "{input}");
        }
    }
}

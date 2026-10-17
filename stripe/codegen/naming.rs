//! Identifier construction for generated Rust code.

/// Rust keywords that cannot be used as identifiers.
const KEYWORDS: &[&str] = &[
    "as", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern", "false", "fn", "for", "if", "impl",
    "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return", "self", "Self", "static", "struct",
    "super", "trait", "true", "type", "unsafe", "use", "where", "while", "async", "await", "abstract", "become", "box",
    "do", "final", "macro", "override", "priv", "try", "typeof", "unsized", "virtual", "yield", "union", "gen",
];

/// Keywords that raw identifiers (`r#...`) do not permit.
const NO_RAW: &[&str] = &["self", "Self", "super", "crate", "extern"];

fn is_keyword(value: &str) -> bool {
    KEYWORDS.contains(&value)
}

/// Replaces every character that is not valid in an identifier with `_` and
/// ensures the result does not start with a digit.
fn sanitize(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() || ch == '_' {
            out.push(ch);
        } else {
            out.push('_');
        }
    }
    if out.is_empty() {
        out.push_str("unnamed");
    }
    if out.starts_with(|ch: char| ch.is_ascii_digit()) {
        out.insert(0, '_');
    }
    out
}

/// A safe Rust identifier for a field or binding.
pub fn field_ident(value: &str) -> String {
    let sanitized = sanitize(value);
    if is_keyword(&sanitized) {
        if NO_RAW.contains(&sanitized.as_str()) {
            format!("{sanitized}_")
        } else {
            format!("r#{sanitized}")
        }
    } else {
        sanitized
    }
}

/// A safe identifier that must not be a raw identifier (enum variants, modules).
pub fn plain_ident(value: &str) -> String {
    let sanitized = sanitize(value);
    if is_keyword(&sanitized) {
        format!("{sanitized}_")
    } else {
        sanitized
    }
}

/// Converts `value` to `PascalCase`, suitable for a type or variant name.
pub fn pascal_case(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    let mut capitalize = true;
    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() {
            if capitalize && ch.is_ascii_alphabetic() {
                out.extend(ch.to_uppercase());
            } else {
                out.push(ch);
            }
            capitalize = false;
        } else {
            capitalize = true;
        }
    }
    if out.is_empty() {
        out.push_str("Unnamed");
    }
    if out.starts_with(|ch: char| ch.is_ascii_digit()) {
        out.insert(0, 'N');
    }
    out
}

/// Derives a Rust type name from an OpenAPI schema key such as `billing.meter`.
pub fn type_name_from_schema(key: &str) -> String {
    pascal_case(key)
}

/// The English singular of a resource word (`customers` -> `customer`).
///
/// Only the forms Stripe uses in its specification are handled; anything else
/// is returned unchanged, which keeps identifiers stable.
pub fn singular(word: &str) -> String {
    const SUFFIXES: &[(&str, &str)] = &[
        ("ies", "y"),
        ("sses", "ss"),
        ("xes", "x"),
        ("zes", "z"),
        ("ches", "ch"),
        ("shes", "sh"),
        ("ses", "s"),
    ];
    for (suffix, replacement) in SUFFIXES {
        if let Some(stem) = word.strip_suffix(suffix) {
            return format!("{stem}{replacement}");
        }
    }
    if let Some(stem) = word.strip_suffix('s') {
        // `status`, `radius`, `analysis`, `address` and friends are not plurals.
        if !stem.ends_with(['s', 'u', 'i']) {
            return stem.to_string();
        }
    }
    word.to_string()
}

/// The English plural of a resource word (`customer` -> `customers`).
pub fn plural(word: &str) -> String {
    if let Some(stem) = word.strip_suffix('y')
        && !stem.ends_with(['a', 'e', 'o', 'u'])
    {
        return format!("{stem}ies");
    }
    if word.ends_with(['s', 'x', 'z']) || word.ends_with("ch") || word.ends_with("sh") {
        return format!("{word}es");
    }
    format!("{word}s")
}

/// Returns true when `segment` is an OpenAPI path parameter such as `{customer}`.
pub fn is_placeholder(segment: &str) -> bool {
    segment.starts_with('{') && segment.ends_with('}')
}

/// The path segments of `path`, excluding the empty leading segment.
pub fn path_segments(path: &str) -> Vec<String> {
    path.split('/')
        .filter(|segment| !segment.is_empty())
        .map(str::to_string)
        .collect()
}

/// The generated module a path belongs to.
///
/// This is the first segment after the API version for `/v1/...` paths, so
/// `/v1/customers/{customer}/sources` lives in `customers`. v2 paths are grouped
/// by API area: `/v2/core/accounts` lives in `v2_core` and
/// `/v2/billing/meter_events` in `v2_billing`.
pub fn module_of_path(path: &str) -> String {
    let segments = path_segments(path);
    match segments.first().map(String::as_str) {
        Some("v2") => match segments.get(1) {
            Some(area) => plain_ident(&format!("v2_{area}")),
            None => "v2".to_string(),
        },
        Some("v1") => segments
            .get(1)
            .map_or_else(|| "common".to_string(), |segment| plain_ident(segment)),
        Some(segment) => plain_ident(segment),
        None => "common".to_string(),
    }
}

/// The resource chain of a path: every non-placeholder segment, singularized,
/// excluding the version prefix (`v1`/`v2`) and any trailing action segment.
pub fn chain_of_segments(segments: &[String], action: Option<&str>) -> String {
    segments
        .iter()
        .filter(|segment| !is_placeholder(segment))
        .filter(|segment| Some(segment.as_str()) != action)
        .map(|segment| singular(segment))
        .collect::<Vec<_>>()
        .join("_")
}

/// Escapes a documentation string into a `#[doc = "..."]` attribute value.
///
/// Only the first sentence is kept, so the generated source (and the work rustc
/// does to parse it) stays proportional to the help users actually read.
pub fn doc_attribute(text: &str) -> String {
    // Keep docs single-line and free of characters that would break the literal.
    let cleaned: String = text
        .chars()
        .map(|ch| match ch {
            '\n' | '\r' | '\t' => ' ',
            '\\' => ' ',
            '"' => '\'',
            other => other,
        })
        .collect();
    let cleaned = cleaned.trim();
    let first_sentence = match cleaned.find(". ") {
        Some(position) => &cleaned[..position + 1],
        None => cleaned,
    };
    let truncated = if first_sentence.chars().count() > 120 {
        let mut s: String = first_sentence.chars().take(117).collect();
        s.push_str("...");
        s
    } else {
        first_sentence.to_string()
    };
    truncated.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn singular_and_plural_round_trip() {
        for (word, expected) in [
            ("customers", "customer"),
            ("sources", "source"),
            ("application_fees", "application_fee"),
            ("tax_ids", "tax_id"),
            ("status", "status"),
            ("address", "address"),
            ("balance", "balance"),
            ("boxes", "box"),
            ("lines", "line"),
            ("invoices", "invoice"),
            ("capabilities", "capability"),
        ] {
            assert_eq!(singular(word), expected, "singular({word})");
        }
        for word in ["customer", "source", "invoice", "line", "tax_id", "capability"] {
            assert_eq!(singular(&plural(word)), word, "round trip {word}");
        }
    }

    #[test]
    fn module_of_path_groups_by_area() {
        assert_eq!(module_of_path("/v1/customers/{customer}/sources"), "customers");
        assert_eq!(module_of_path("/v1/checkout/sessions"), "checkout");
        assert_eq!(module_of_path("/v2/core/accounts"), "v2_core");
        assert_eq!(module_of_path("/v2/billing/meter_events"), "v2_billing");
    }

    #[test]
    fn chain_drops_placeholders_and_actions() {
        let segments = |path: &str| -> Vec<String> { path_segments(path)[1..].to_vec() };
        assert_eq!(
            chain_of_segments(&segments("/v1/customers/{customer}/sources/{id}"), None),
            "customer_source"
        );
        assert_eq!(
            chain_of_segments(&segments("/v1/checkout/sessions/{session}/expire"), Some("expire")),
            "checkout_session"
        );
    }
}

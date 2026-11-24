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
///
/// The result is valid single-line Markdown: truncation never leaves half an
/// inline link behind, bracket references that are not links are reduced to
/// their text, and bare URLs become autolinks. That keeps `cargo doc` warning
/// free, which is why the Stripe specification text can be embedded verbatim.
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
    sanitize_markdown(&truncated).trim().to_string()
}

/// Returns the index of the first `close` at or after `start`, if any.
fn find_closing(chars: &[char], start: usize, close: char) -> Option<usize> {
    chars
        .get(start..)?
        .iter()
        .position(|&ch| ch == close)
        .map(|offset| start + offset)
}

/// Reduces documentation text to Markdown that rustdoc documents without
/// warnings.
///
/// Complete inline links (`[label](target)`) are kept; bracket references that
/// are not complete links (shortcut references such as `[page]`, or a link cut
/// in half by truncation) are replaced by their label text; bare URLs are
/// wrapped in `<...>` so they become autolinks.
fn sanitize_markdown(text: &str) -> String {
    let chars: Vec<char> = text.chars().collect();
    let mut out = String::with_capacity(text.len());
    let mut i = 0;
    while i < chars.len() {
        if chars[i] != '[' {
            out.push(chars[i]);
            i += 1;
            continue;
        }
        let Some(label_end) = find_closing(&chars, i + 1, ']') else {
            // A `[` with no closing bracket was cut by truncation.
            i += 1;
            continue;
        };
        let label: String = chars[i + 1..label_end].iter().collect();
        if chars.get(label_end + 1) == Some(&'(') {
            if let Some(target_end) = find_closing(&chars, label_end + 2, ')') {
                out.extend(&chars[i..=target_end]);
                i = target_end + 1;
                continue;
            }
            // A link target cut by truncation: keep the label, drop the rest.
            out.push_str(&label);
            out.push_str("...");
            break;
        }
        out.push_str(&label);
        i = label_end + 1;
    }
    autolink_bare_urls(&out)
}

/// Wraps URLs that are not already part of a link in `<...>`.
fn autolink_bare_urls(text: &str) -> String {
    let chars: Vec<char> = text.chars().collect();
    let mut out = String::with_capacity(text.len());
    let mut i = 0;
    let mut in_link_target = false;
    while i < chars.len() {
        let ch = chars[i];
        if ch == '(' {
            in_link_target = true;
            out.push(ch);
            i += 1;
            continue;
        }
        if ch == ')' {
            in_link_target = false;
            out.push(ch);
            i += 1;
            continue;
        }
        let rest = &chars[i..];
        let is_url = !in_link_target
            && !out.ends_with('<')
            && (rest.starts_with(&['h', 't', 't', 'p', ':', '/', '/'])
                || rest.starts_with(&['h', 't', 't', 'p', 's', ':', '/', '/']));
        if is_url {
            let start = i;
            while i < chars.len() && !chars[i].is_whitespace() && chars[i] != '>' {
                i += 1;
            }
            // Keep sentence punctuation outside the autolink.
            let mut end = i;
            while end > start && matches!(chars[end - 1], '.' | ',' | ';' | ':') {
                end -= 1;
            }
            out.push('<');
            out.extend(&chars[start..end]);
            out.push('>');
            out.extend(&chars[end..i]);
            continue;
        }
        out.push(ch);
        i += 1;
    }
    out
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

    #[test]
    fn doc_attribute_keeps_markdown_valid() {
        // Complete links survive untouched.
        assert_eq!(
            doc_attribute("See [the docs](https://docs.stripe.com/api). More."),
            "See [the docs](https://docs.stripe.com/api)."
        );
        // Bare URLs become autolinks, with sentence punctuation kept outside.
        assert_eq!(
            doc_attribute("See https://docs.stripe.com/api. More."),
            "See <https://docs.stripe.com/api>."
        );
        // Shortcut references lose their brackets so rustdoc does not try to
        // resolve them as intra-doc links.
        assert_eq!(doc_attribute("Only [Deprecated] values. More."), "Only Deprecated values.");
        // A link cut in half by truncation keeps its label but not the broken
        // target, and the result is never left with half an inline link.
        let text = format!(
            "Array of strings containing [categories](https://docs.stripe.com/api#{})",
            "x".repeat(80)
        );
        let doc = doc_attribute(&text);
        assert!(doc.ends_with("..."), "{doc}");
        assert!(doc.contains("categories"));
        assert!(!doc.contains("]("), "{doc}");
        assert!(!doc.contains("https://"), "{doc}");
    }
}

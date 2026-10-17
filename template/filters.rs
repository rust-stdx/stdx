//! Built-in template filters: string transforms, escaping, URL encoding, etc.

use alloc::{
    rc::Rc,
    string::{String, ToString},
};

use crate::{escapers::html_escape_to_string, value::Value};

pub type FilterFn = fn(&Value, &[Value]) -> Result<Value, crate::error::Error>;

// Must stay sorted alphabetically for binary_search_by_key in vm.rs
pub const fn builtin_filters() -> &'static [(&'static str, FilterFn)] {
    const FILTERS: &[(&str, FilterFn)] = &[
        ("capitalize", filter_capitalize),
        ("default", filter_default),
        ("escape", filter_escape),
        ("first", filter_first),
        ("join", filter_join),
        ("last", filter_last),
        ("length", filter_length),
        ("lower", filter_lower),
        ("reverse", filter_reverse),
        ("safe", filter_safe),
        ("title", filter_title),
        ("trim", filter_trim),
        ("upper", filter_upper),
        ("urlencode", filter_urlencode),
    ];
    FILTERS
}

/// Return the accepted argument count range for a built-in filter, or `None` if
/// the filter is unknown.
///
/// The upper bound is [`usize::MAX`] for filters that accept any number of
/// arguments. Used to validate filters at parse time.
pub fn filter_arity(name: &str) -> Option<(usize, usize)> {
    Some(match name {
        "capitalize" | "escape" | "first" | "last" | "length" | "lower" | "reverse" | "safe" | "title" | "trim"
        | "upper" | "urlencode" => (0, 0),
        "default" | "join" => (0, 1),
        _ => return None,
    })
}

/// Render a non-string value to a string using its default representation.
fn value_to_string(val: &Value) -> String {
    let mut buf = String::new();
    // Writing to a `String` never fails.
    let _ = val.fmt_to(&mut buf);
    buf
}

/// Wrap a transformed string back into a value, preserving `Safe` status.
///
/// Filters that only transform the characters of a string (and never introduce
/// unescaped markup) keep the value safe: if the input was [`Value::Safe`] the
/// output stays [`Value::Safe`], otherwise it is a normal [`Value::Str`] and will
/// be auto-escaped on output.
fn string_result(input: &Value, s: String) -> Value {
    if matches!(input, Value::Safe(_)) {
        Value::Safe(s.into())
    } else {
        Value::Str(s.into())
    }
}

/// A value is safe to join verbatim if it is not a plain (unescaped) string.
fn is_safe_component(v: &Value) -> bool {
    !matches!(v, Value::Str(_))
}

fn filter_upper(val: &Value, _args: &[Value]) -> Result<Value, crate::error::Error> {
    let s = match val {
        Value::Str(s) | Value::Safe(s) => s.to_uppercase(),
        v => value_to_string(v).to_uppercase(),
    };
    Ok(string_result(val, s))
}

fn filter_lower(val: &Value, _args: &[Value]) -> Result<Value, crate::error::Error> {
    let s = match val {
        Value::Str(s) | Value::Safe(s) => s.to_lowercase(),
        v => value_to_string(v).to_lowercase(),
    };
    Ok(string_result(val, s))
}

fn filter_trim(val: &Value, _args: &[Value]) -> Result<Value, crate::error::Error> {
    let s = match val {
        Value::Str(s) | Value::Safe(s) => s.trim().to_string(),
        v => value_to_string(v).trim().to_string(),
    };
    Ok(string_result(val, s))
}

fn filter_escape(val: &Value, _args: &[Value]) -> Result<Value, crate::error::Error> {
    let s = match val {
        Value::Str(s) => s.as_ref(),
        Value::Safe(s) => s.as_ref(),
        v => {
            let buf = value_to_string(v);
            return Ok(Value::Safe(html_escape_to_string(&buf).into()));
        }
    };
    Ok(Value::Safe(html_escape_to_string(s).into()))
}

fn filter_safe(val: &Value, _args: &[Value]) -> Result<Value, crate::error::Error> {
    match val {
        Value::Str(s) | Value::Safe(s) => Ok(Value::Safe(Rc::clone(s))),
        other => Ok(Value::Safe(value_to_string(other).into())),
    }
}

fn filter_length(val: &Value, _args: &[Value]) -> Result<Value, crate::error::Error> {
    match val {
        Value::Str(s) | Value::Safe(s) => Ok(Value::I64(s.chars().count() as i64)),
        Value::Array(a) => Ok(Value::I64(a.len() as i64)),
        Value::Map(m) => Ok(Value::I64(m.len() as i64)),
        _ => Ok(Value::I64(0)),
    }
}

/// Replace [`Value::Null`] (a missing or explicitly-null variable) with the
/// first argument.
///
/// Unlike Jinja's default filter this does **not** trigger on other falsy
/// values such as `0`, `false`, `""`, `[]` or `{}`; only `null` is replaced.
fn filter_default(val: &Value, args: &[Value]) -> Result<Value, crate::error::Error> {
    if matches!(val, Value::Null) {
        Ok(args.first().cloned().unwrap_or(Value::Null))
    } else {
        Ok(val.clone())
    }
}

fn filter_capitalize(val: &Value, _args: &[Value]) -> Result<Value, crate::error::Error> {
    let s = match val {
        Value::Str(s) | Value::Safe(s) => s.to_string(),
        v => value_to_string(v),
    };
    let mut chars = s.chars();
    let result = match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().to_string() + &chars.as_str().to_lowercase(),
    };
    Ok(string_result(val, result))
}

fn filter_join(val: &Value, args: &[Value]) -> Result<Value, crate::error::Error> {
    let separator = args.first().and_then(|v| v.as_str()).unwrap_or("");
    match val {
        Value::Array(arr) => {
            let mut result = String::new();
            let mut all_safe = is_safe_component(args.first().unwrap_or(&Value::Null));
            for (i, v) in arr.iter().enumerate() {
                if i > 0 {
                    result.push_str(separator);
                }
                let _ = v.fmt_to(&mut result);
                all_safe &= is_safe_component(v);
            }
            if all_safe {
                Ok(Value::Safe(result.into()))
            } else {
                Ok(Value::Str(result.into()))
            }
        }
        Value::Str(s) | Value::Safe(s) => Ok(string_result(val, s.to_string())),
        v => Ok(string_result(val, value_to_string(v))),
    }
}

fn filter_title(val: &Value, _args: &[Value]) -> Result<Value, crate::error::Error> {
    let s = match val {
        Value::Str(s) | Value::Safe(s) => s.to_string(),
        v => value_to_string(v),
    };
    let mut result = String::with_capacity(s.len());
    for (i, word) in s.split_whitespace().enumerate() {
        if i > 0 {
            result.push(' ');
        }
        let mut chars = word.chars();
        match chars.next() {
            None => {}
            Some(c) => {
                result.push_str(&c.to_uppercase().to_string());
                result.push_str(&chars.as_str().to_lowercase());
            }
        }
    }
    Ok(string_result(val, result))
}

fn filter_reverse(val: &Value, _args: &[Value]) -> Result<Value, crate::error::Error> {
    match val {
        Value::Str(s) | Value::Safe(s) => {
            let reversed: String = s.chars().rev().collect();
            Ok(string_result(val, reversed))
        }
        Value::Array(arr) => {
            let mut reversed = arr.as_ref().clone();
            reversed.reverse();
            Ok(Value::Array(Rc::new(reversed)))
        }
        v => Ok(v.clone()),
    }
}

fn filter_first(val: &Value, _args: &[Value]) -> Result<Value, crate::error::Error> {
    match val {
        Value::Array(arr) => Ok(arr.first().cloned().unwrap_or(Value::Null)),
        Value::Str(s) | Value::Safe(s) => Ok(s
            .chars()
            .next()
            .map(|c| string_result(val, c.to_string()))
            .unwrap_or_else(|| string_result(val, String::new()))),
        _ => Ok(Value::Null),
    }
}

fn filter_last(val: &Value, _args: &[Value]) -> Result<Value, crate::error::Error> {
    match val {
        Value::Array(arr) => Ok(arr.last().cloned().unwrap_or(Value::Null)),
        Value::Str(s) | Value::Safe(s) => Ok(s
            .chars()
            .last()
            .map(|c| string_result(val, c.to_string()))
            .unwrap_or_else(|| string_result(val, String::new()))),
        _ => Ok(Value::Null),
    }
}

const HEX: &[u8; 16] = b"0123456789ABCDEF";

fn filter_urlencode(val: &Value, _args: &[Value]) -> Result<Value, crate::error::Error> {
    let s = match val {
        Value::Str(s) | Value::Safe(s) => s.as_ref().to_string(),
        v => value_to_string(v),
    };
    let mut encoded = String::with_capacity(s.len());
    for &b in s.as_bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(b as char);
            }
            b' ' => encoded.push('+'),
            _ => {
                encoded.push('%');
                encoded.push(HEX[(b >> 4) as usize] as char);
                encoded.push(HEX[(b & 0x0f) as usize] as char);
            }
        }
    }
    Ok(Value::Str(encoded.into()))
}

#[cfg(test)]
mod tests {
    use alloc::{collections::BTreeMap, vec, vec::Vec};

    use super::*;

    #[test]
    fn test_upper() {
        let val = Value::Str("hello".into());
        let result = filter_upper(&val, &[]).unwrap();
        assert_eq!(result.as_str(), Some("HELLO"));
    }

    #[test]
    fn test_escape() {
        let val = Value::Str("<script>alert('xss')</script>".into());
        let result = filter_escape(&val, &[]).unwrap();
        assert_eq!(result.as_str(), Some("&lt;script&gt;alert(&#39;xss&#39;)&lt;/script&gt;"));
    }

    #[test]
    fn test_default_truthy() {
        let val = Value::Str("hello".into());
        let result = filter_default(&val, &[Value::Str("fallback".into())]).unwrap();
        assert_eq!(result.as_str(), Some("hello"));
    }

    #[test]
    fn test_default_falsy() {
        let val = Value::Null;
        let result = filter_default(&val, &[Value::Str("fallback".into())]).unwrap();
        assert_eq!(result.as_str(), Some("fallback"));
    }

    #[test]
    fn test_capitalize() {
        let val = Value::Str("hello world".into());
        let result = filter_capitalize(&val, &[]).unwrap();
        assert_eq!(result.as_str(), Some("Hello world"));
    }

    #[test]
    fn test_title() {
        let val = Value::Str("hello world".into());
        let result = filter_title(&val, &[]).unwrap();
        assert_eq!(result.as_str(), Some("Hello World"));
    }

    #[test]
    fn test_join() {
        let val = Value::Array(Rc::new(vec![
            Value::Str("a".into()),
            Value::Str("b".into()),
            Value::Str("c".into()),
        ]));
        let result = filter_join(&val, &[Value::Str(", ".into())]).unwrap();
        assert_eq!(result.as_str(), Some("a, b, c"));
    }

    #[test]
    fn test_reverse_string() {
        let val = Value::Str("abc".into());
        let result = filter_reverse(&val, &[]).unwrap();
        assert_eq!(result.as_str(), Some("cba"));
    }

    #[test]
    fn test_reverse_array() {
        let val = Value::Array(Rc::new(vec![Value::I64(1), Value::I64(2), Value::I64(3)]));
        let result = filter_reverse(&val, &[]).unwrap();
        match &result {
            Value::Array(arr) => {
                assert_eq!(arr.len(), 3);
                assert_eq!(arr[0], Value::I64(3));
                assert_eq!(arr[1], Value::I64(2));
                assert_eq!(arr[2], Value::I64(1));
            }
            _ => panic!("expected array"),
        }
    }

    #[test]
    fn test_first() {
        let val = Value::Array(Rc::new(vec![Value::I64(1), Value::I64(2)]));
        let result = filter_first(&val, &[]).unwrap();
        assert_eq!(result, Value::I64(1));
    }

    #[test]
    fn test_last() {
        let val = Value::Array(Rc::new(vec![Value::I64(1), Value::I64(2)]));
        let result = filter_last(&val, &[]).unwrap();
        assert_eq!(result, Value::I64(2));
    }

    #[test]
    fn test_urlencode() {
        let val = Value::Str("hello world".into());
        let result = filter_urlencode(&val, &[]).unwrap();
        assert_eq!(result.as_str(), Some("hello+world"));
    }

    #[test]
    fn test_length_string() {
        let val = Value::Str("hello".into());
        let result = filter_length(&val, &[]).unwrap();
        assert_eq!(result, Value::I64(5));
    }

    #[test]
    fn test_length_array() {
        let val = Value::Array(Rc::new(vec![Value::I64(1), Value::I64(2)]));
        let result = filter_length(&val, &[]).unwrap();
        assert_eq!(result, Value::I64(2));
    }

    #[test]
    fn test_trim() {
        let val = Value::Str("  hello  ".into());
        let result = filter_trim(&val, &[]).unwrap();
        assert_eq!(result.as_str(), Some("hello"));
    }

    #[test]
    fn test_escape_non_string() {
        let val = Value::I64(42);
        let result = filter_escape(&val, &[]).unwrap();
        assert_eq!(result.as_str(), Some("42"));
    }

    #[test]
    fn test_first_empty_string() {
        let val = Value::Str("".into());
        let result = filter_first(&val, &[]).unwrap();
        assert_eq!(result.as_str(), Some(""));
    }

    #[test]
    fn test_last_empty_string() {
        let val = Value::Str("".into());
        let result = filter_last(&val, &[]).unwrap();
        assert_eq!(result.as_str(), Some(""));
    }

    #[test]
    fn test_first_non_array() {
        let val = Value::I64(42);
        let result = filter_first(&val, &[]).unwrap();
        assert_eq!(result, Value::Null);
    }

    #[test]
    fn test_safe_marks_value() {
        let val = Value::Str("<b>bold</b>".into());
        let result = filter_safe(&val, &[]).unwrap();
        assert_eq!(result, Value::Safe("<b>bold</b>".into()));
    }

    #[test]
    fn test_default_falsy_no_args() {
        let val = Value::Null;
        let result = filter_default(&val, &[]).unwrap();
        assert_eq!(result, Value::Null);
    }

    #[test]
    fn test_urlencode_special_chars() {
        let val = Value::Str("a&b=c".into());
        let result = filter_urlencode(&val, &[]).unwrap();
        assert_eq!(result.as_str(), Some("a%26b%3Dc"));
    }

    #[test]
    fn test_reverse_non_string_array() {
        let val = Value::I64(42);
        let result = filter_reverse(&val, &[]).unwrap();
        assert_eq!(result, Value::I64(42));
    }

    #[test]
    fn test_join_non_array() {
        let val = Value::Str("hello".into());
        let result = filter_join(&val, &[Value::Str(", ".into())]).unwrap();
        assert_eq!(result.as_str(), Some("hello"));
    }

    #[test]
    fn test_length_safe() {
        let val = Value::Safe("hello".into());
        let result = filter_length(&val, &[]).unwrap();
        assert_eq!(result, Value::I64(5));
    }

    #[test]
    fn test_first_safe() {
        let val = Value::Safe("abc".into());
        let result = filter_first(&val, &[]).unwrap();
        assert_eq!(result.as_str(), Some("a"));
    }

    #[test]
    fn test_last_safe() {
        let val = Value::Safe("abc".into());
        let result = filter_last(&val, &[]).unwrap();
        assert_eq!(result.as_str(), Some("c"));
    }

    #[test]
    fn test_reverse_safe() {
        let val = Value::Safe("abc".into());
        let result = filter_reverse(&val, &[]).unwrap();
        assert_eq!(result.as_str(), Some("cba"));
    }

    #[test]
    fn test_reverse_empty_safe() {
        let val = Value::Safe("".into());
        let result = filter_reverse(&val, &[]).unwrap();
        assert_eq!(result.as_str(), Some(""));
    }

    #[test]
    fn test_first_empty_safe() {
        let val = Value::Safe("".into());
        let result = filter_first(&val, &[]).unwrap();
        assert_eq!(result.as_str(), Some(""));
    }

    #[test]
    fn test_last_empty_safe() {
        let val = Value::Safe("".into());
        let result = filter_last(&val, &[]).unwrap();
        assert_eq!(result.as_str(), Some(""));
    }

    #[test]
    fn test_length_on_safe() {
        let val = Value::Safe("".into());
        let result = filter_length(&val, &[]).unwrap();
        assert_eq!(result, Value::I64(0));
    }

    #[test]
    fn test_default_on_zero() {
        // `default` only replaces Null, not other falsy values.
        let val = Value::I64(0);
        let result = filter_default(&val, &[Value::Str("fallback".into())]).unwrap();
        assert_eq!(result, Value::I64(0));
    }

    #[test]
    fn test_default_on_false() {
        let val = Value::Bool(false);
        let result = filter_default(&val, &[Value::Str("fallback".into())]).unwrap();
        assert_eq!(result, Value::Bool(false));
    }

    #[test]
    fn test_default_on_empty_string() {
        let val = Value::Str("".into());
        let result = filter_default(&val, &[Value::Str("fallback".into())]).unwrap();
        assert_eq!(result, Value::Str("".into()));
    }

    #[test]
    fn test_title_empty() {
        let val = Value::Str("".into());
        let result = filter_title(&val, &[]).unwrap();
        assert_eq!(result.as_str(), Some(""));
    }

    #[test]
    fn test_capitalize_empty() {
        let val = Value::Str("".into());
        let result = filter_capitalize(&val, &[]).unwrap();
        assert_eq!(result.as_str(), Some(""));
    }

    #[test]
    fn test_trim_empty() {
        let val = Value::Str("".into());
        let result = filter_trim(&val, &[]).unwrap();
        assert_eq!(result.as_str(), Some(""));
    }

    #[test]
    fn test_reverse_empty_array() {
        let val = Value::Array(Rc::new(vec![]));
        let result = filter_reverse(&val, &[]).unwrap();
        match &result {
            Value::Array(arr) => assert!(arr.is_empty()),
            _ => panic!("expected array"),
        }
    }

    #[test]
    fn test_first_empty_array() {
        let val = Value::Array(Rc::new(vec![] as Vec<Value>));
        let result = filter_first(&val, &[]).unwrap();
        assert_eq!(result, Value::Null);
    }

    #[test]
    fn test_last_empty_array() {
        let val = Value::Array(Rc::new(vec![] as Vec<Value>));
        let result = filter_last(&val, &[]).unwrap();
        assert_eq!(result, Value::Null);
    }

    #[test]
    fn test_default_on_f64_zero() {
        let val = Value::F64(0.0);
        let result = filter_default(&val, &[Value::Str("fb".into())]).unwrap();
        assert_eq!(result, Value::F64(0.0));
    }

    #[test]
    fn test_default_on_f64_nan() {
        let val = Value::F64(f64::NAN);
        let result = filter_default(&val, &[Value::Str("fb".into())]).unwrap();
        assert!(matches!(result, Value::F64(n) if n.is_nan()));
    }

    #[test]
    fn test_default_on_empty_array() {
        let val = Value::Array(Rc::new(vec![]));
        let result = filter_default(&val, &[Value::Str("fb".into())]).unwrap();
        assert_eq!(result, Value::Array(Rc::new(vec![])));
    }

    #[test]
    fn test_default_on_empty_map() {
        let val = Value::Map(Rc::new(BTreeMap::new()));
        let result = filter_default(&val, &[Value::Str("fb".into())]).unwrap();
        assert_eq!(result, Value::Map(Rc::new(BTreeMap::new())));
    }

    #[test]
    fn test_length_on_empty_map() {
        let val = Value::Map(Rc::new(BTreeMap::new()));
        let result = filter_length(&val, &[]).unwrap();
        assert_eq!(result, Value::I64(0));
    }

    #[test]
    fn test_length_on_multibyte_string() {
        let val = Value::Str("é".into());
        let result = filter_length(&val, &[]).unwrap();
        assert_eq!(result, Value::I64(1));
    }

    #[test]
    fn test_join_on_empty_array() {
        let val = Value::Array(Rc::new(vec![]));
        let result = filter_join(&val, &[Value::Str(",".into())]).unwrap();
        assert_eq!(result.as_str(), Some(""));
    }

    #[test]
    fn test_join_with_non_string_separator() {
        let val = Value::Array(Rc::new(vec![Value::Str("a".into()), Value::Str("b".into())]));
        let result = filter_join(&val, &[Value::I64(42)]).unwrap();
        assert_eq!(result.as_str(), Some("ab"));
    }

    #[test]
    fn test_safe_on_non_string() {
        let val = Value::I64(42);
        let result = filter_safe(&val, &[]).unwrap();
        assert_eq!(result, Value::Safe("42".into()));
    }

    #[test]
    fn test_escape_on_non_string() {
        let val = Value::I64(42);
        let result = filter_escape(&val, &[]).unwrap();
        assert_eq!(result, Value::Safe("42".into()));
    }

    #[test]
    fn test_upper_on_safe() {
        // `Safe` status is preserved through character transforms.
        let val = Value::Safe("abc".into());
        let result = filter_upper(&val, &[]).unwrap();
        assert_eq!(result, Value::Safe("ABC".into()));
    }

    #[test]
    fn test_lower_on_safe() {
        let val = Value::Safe("ABC".into());
        let result = filter_lower(&val, &[]).unwrap();
        assert_eq!(result, Value::Safe("abc".into()));
    }

    #[test]
    fn test_reverse_preserves_safe() {
        let val = Value::Safe("<b>".into());
        let result = filter_reverse(&val, &[]).unwrap();
        assert_eq!(result, Value::Safe(">b<".into()));
    }

    #[test]
    fn test_join_safe_array_stays_safe() {
        let val = Value::Array(Rc::new(vec![Value::Safe("a".into()), Value::Safe("b".into())]));
        let result = filter_join(&val, &[Value::Safe(",".into())]).unwrap();
        assert_eq!(result, Value::Safe("a,b".into()));
    }

    #[test]
    fn test_join_unsafe_element_is_escaped() {
        let val = Value::Array(Rc::new(vec![Value::Safe("a".into()), Value::Str("<b>".into())]));
        let result = filter_join(&val, &[Value::Safe(",".into())]).unwrap();
        assert_eq!(result, Value::Str("a,<b>".into()));
    }

    #[test]
    fn test_urlencode_utf8() {
        let val = Value::Str("é".into());
        let result = filter_urlencode(&val, &[]).unwrap();
        assert_eq!(result.as_str(), Some("%C3%A9"));
    }
}

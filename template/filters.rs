//! Built-in template filters: string transforms, escaping, URL encoding, etc.

use alloc::{
    format,
    rc::Rc,
    string::{String, ToString},
};

use crate::{escapers::html_escape_to_string, value::Value};

pub type FilterFn = fn(&Value, &[Value]) -> Result<Value, crate::error::Error>;

// Must stay sorted alphabetically for binary_search_by_key in vm.rs
pub const fn builtin_filters() -> &'static [(&'static str, FilterFn)] {
    const FILTERS: &[(&'static str, FilterFn)] = &[
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

fn filter_upper(val: &Value, _args: &[Value]) -> Result<Value, crate::error::Error> {
    match val {
        Value::Str(s) => Ok(Value::Str(s.to_uppercase().into())),
        v => {
            let mut buf = String::new();
            v.fmt_to(&mut buf).unwrap();
            Ok(Value::Str(buf.to_uppercase().into()))
        }
    }
}

fn filter_lower(val: &Value, _args: &[Value]) -> Result<Value, crate::error::Error> {
    match val {
        Value::Str(s) => Ok(Value::Str(s.to_lowercase().into())),
        v => {
            let mut buf = String::new();
            v.fmt_to(&mut buf).unwrap();
            Ok(Value::Str(buf.to_lowercase().into()))
        }
    }
}

fn filter_trim(val: &Value, _args: &[Value]) -> Result<Value, crate::error::Error> {
    match val {
        Value::Str(s) => Ok(Value::Str(s.trim().into())),
        v => {
            let mut buf = String::new();
            v.fmt_to(&mut buf).unwrap();
            Ok(Value::Str(buf.trim().into()))
        }
    }
}

fn filter_escape(val: &Value, _args: &[Value]) -> Result<Value, crate::error::Error> {
    let s = match val {
        Value::Str(s) => s.as_ref(),
        v => {
            let mut buf = String::new();
            v.fmt_to(&mut buf).unwrap();
            return Ok(Value::Safe(html_escape_to_string(&buf).into()));
        }
    };
    Ok(Value::Safe(html_escape_to_string(s).into()))
}

fn filter_safe(val: &Value, _args: &[Value]) -> Result<Value, crate::error::Error> {
    match val {
        Value::Str(s) => Ok(Value::Safe(Rc::clone(s))),
        Value::Safe(s) => Ok(Value::Safe(Rc::clone(s))),
        other => {
            let mut buf = String::new();
            other.fmt_to(&mut buf).unwrap();
            Ok(Value::Safe(buf.into()))
        }
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

fn filter_default(val: &Value, args: &[Value]) -> Result<Value, crate::error::Error> {
    if val.is_truthy() {
        Ok(val.clone())
    } else if let Some(default) = args.first() {
        Ok(default.clone())
    } else {
        Ok(Value::Null)
    }
}

fn filter_capitalize(val: &Value, _args: &[Value]) -> Result<Value, crate::error::Error> {
    let s = match val {
        Value::Str(s) => s.to_string(),
        v => {
            let mut buf = String::new();
            v.fmt_to(&mut buf).unwrap();
            buf
        }
    };
    let mut chars = s.chars();
    let result = match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().to_string() + &chars.as_str().to_lowercase(),
    };
    Ok(Value::Str(result.into()))
}

fn filter_join(val: &Value, args: &[Value]) -> Result<Value, crate::error::Error> {
    let separator = args.first().and_then(|v| v.as_str()).unwrap_or("");
    match val {
        Value::Array(arr) => {
            let mut result = String::new();
            for (i, v) in arr.iter().enumerate() {
                if i > 0 {
                    result.push_str(separator);
                }
                v.fmt_to(&mut result).unwrap();
            }
            Ok(Value::Str(result.into()))
        }
        v => {
            let mut buf = String::new();
            v.fmt_to(&mut buf).unwrap();
            Ok(Value::Str(buf.into()))
        }
    }
}

fn filter_title(val: &Value, _args: &[Value]) -> Result<Value, crate::error::Error> {
    let s = match val {
        Value::Str(s) => s.to_string(),
        v => {
            let mut buf = String::new();
            v.fmt_to(&mut buf).unwrap();
            buf
        }
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
    Ok(Value::Str(result.into()))
}

fn filter_reverse(val: &Value, _args: &[Value]) -> Result<Value, crate::error::Error> {
    match val {
        Value::Str(s) | Value::Safe(s) => Ok(Value::Str(s.chars().rev().collect::<String>().into())),
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
            .map(|c| Value::Str(c.to_string().into()))
            .unwrap_or(Value::Str("".into()))),
        _ => Ok(Value::Null),
    }
}

fn filter_last(val: &Value, _args: &[Value]) -> Result<Value, crate::error::Error> {
    match val {
        Value::Array(arr) => Ok(arr.last().cloned().unwrap_or(Value::Null)),
        Value::Str(s) | Value::Safe(s) => Ok(s
            .chars()
            .last()
            .map(|c| Value::Str(c.to_string().into()))
            .unwrap_or(Value::Str("".into()))),
        _ => Ok(Value::Null),
    }
}

fn filter_urlencode(val: &Value, _args: &[Value]) -> Result<Value, crate::error::Error> {
    let s = match val {
        Value::Str(s) => s.as_ref().to_string(),
        v => {
            let mut buf = String::new();
            v.fmt_to(&mut buf).unwrap();
            buf
        }
    };
    let mut encoded = String::with_capacity(s.len());
    for &b in s.as_bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(b as char);
            }
            b' ' => encoded.push('+'),
            _ => encoded.push_str(&format!("%{:02X}", b)),
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
        let val = Value::I64(0);
        let result = filter_default(&val, &[Value::Str("fallback".into())]).unwrap();
        assert_eq!(result.as_str(), Some("fallback"));
    }

    #[test]
    fn test_default_on_false() {
        let val = Value::Bool(false);
        let result = filter_default(&val, &[Value::Str("fallback".into())]).unwrap();
        assert_eq!(result.as_str(), Some("fallback"));
    }

    #[test]
    fn test_default_on_empty_string() {
        let val = Value::Str("".into());
        let result = filter_default(&val, &[Value::Str("fallback".into())]).unwrap();
        assert_eq!(result.as_str(), Some("fallback"));
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
        assert_eq!(result.as_str(), Some("fb"));
    }

    #[test]
    fn test_default_on_f64_nan() {
        let val = Value::F64(f64::NAN);
        let result = filter_default(&val, &[Value::Str("fb".into())]).unwrap();
        assert_eq!(result.as_str(), Some("fb"));
    }

    #[test]
    fn test_default_on_empty_array() {
        let val = Value::Array(Rc::new(vec![]));
        let result = filter_default(&val, &[Value::Str("fb".into())]).unwrap();
        assert_eq!(result.as_str(), Some("fb"));
    }

    #[test]
    fn test_default_on_empty_map() {
        let val = Value::Map(Rc::new(BTreeMap::new()));
        let result = filter_default(&val, &[Value::Str("fb".into())]).unwrap();
        assert_eq!(result.as_str(), Some("fb"));
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
        let val = Value::Safe("abc".into());
        let result = filter_upper(&val, &[]).unwrap();
        assert_eq!(result, Value::Str("ABC".into()));
    }

    #[test]
    fn test_lower_on_safe() {
        let val = Value::Safe("ABC".into());
        let result = filter_lower(&val, &[]).unwrap();
        assert_eq!(result, Value::Str("abc".into()));
    }
}

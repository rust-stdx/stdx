//! # Request parameter encoding
//!
//! Stripe's v1 API accepts `application/x-www-form-urlencoded` bodies that use
//! bracket notation for nested structures:
//!
//! ```text
//! email=jenny%40example.com
//! metadata[order_id]=6735
//! items[0][price]=price_123
//! expand[]=customer
//! ```
//!
//! [`FormBuilder`] writes exactly that syntax into a single, reusable buffer,
//! and [`FormEncode`] is implemented by the generated request parameter types.
//! Keys and values are percent-encoded individually, so a crafted metadata key
//! such as `evil]=x&metadata[` cannot inject extra parameters.
//!
//! Two details of the wire format are worth knowing:
//!
//! * An optional field set to `Some(String::new())` is sent as `name=`, which is
//!   how Stripe is told to clear a value. Omitting the field (`None`) leaves the
//!   current value alone.
//! * v1 arrays use `name[]=value`, v2 query arrays repeat the key
//!   (`name=a&name=b`); the generated encoders pick the right one.
//!
//! The v2 JSON APIs take JSON bodies instead; those types implement
//! `serde::Serialize` and are sent as `application/json`.

use core::fmt::{self, Write as _};

const HEX: &[u8; 16] = b"0123456789ABCDEF";

const fn unreserved_table() -> [bool; 256] {
    let mut table = [false; 256];
    let mut i = 0usize;
    while i < 256 {
        let byte = i as u8;
        table[i] = matches!(
            byte,
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~'
        );
        i += 1;
    }
    table
}

/// Bytes that are safe to emit verbatim in an `x-www-form-urlencoded` stream.
static UNRESERVED: [bool; 256] = unreserved_table();

/// Percent-encodes `value` into `out`.
///
/// Every byte outside the RFC 3986 unreserved set is encoded, including `+`,
/// `&`, `=`, `[`, `]` and space (`%20`). Encoding space as `%20` rather than
/// `+` keeps the output unambiguous for every server-side decoder.
pub fn percent_encode_into(out: &mut String, value: &str) {
    let bytes = value.as_bytes();
    let mut start = 0usize;
    for (index, &byte) in bytes.iter().enumerate() {
        if UNRESERVED[byte as usize] {
            continue;
        }
        if start < index {
            out.push_str(&value[start..index]);
        }
        out.push('%');
        out.push(HEX[(byte >> 4) as usize] as char);
        out.push(HEX[(byte & 0x0f) as usize] as char);
        start = index + 1;
    }
    if start < bytes.len() {
        out.push_str(&value[start..]);
    }
}

/// A single parameter value that can be written into a [`FormBuilder`].
///
/// Implemented for scalars and generated string enums; containers are handled
/// by the generated `encode_form` implementations so that arrays of objects use
/// indexed notation while arrays of scalars use `[]`.
pub trait FormEncode {
    /// Appends this value to `form` using the builder's current key prefix.
    fn encode_form(&self, form: &mut FormBuilder);
}

/// Builds an `application/x-www-form-urlencoded` payload or query string.
///
/// The builder keeps a reusable key prefix so that nested objects can be
/// encoded without allocating a key string per field.
#[derive(Debug, Default)]
pub struct FormBuilder {
    buf: String,
    prefix: String,
    stack: Vec<usize>,
}

impl FormBuilder {
    /// Creates an empty builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an empty builder with room for `capacity` bytes of output.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buf: String::with_capacity(capacity),
            prefix: String::with_capacity(32),
            stack: Vec::with_capacity(8),
        }
    }

    /// Returns the encoded output built so far.
    pub fn as_str(&self) -> &str {
        &self.buf
    }

    /// Consumes the builder and returns the encoded output.
    pub fn into_string(self) -> String {
        self.buf
    }

    /// Returns true when no parameter has been written.
    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }

    /// Returns the number of bytes written so far.
    pub fn len(&self) -> usize {
        self.buf.len()
    }

    fn start_pair(&mut self) {
        if !self.buf.is_empty() {
            self.buf.push('&');
        }
    }

    /// Pushes a key segment, making it the current prefix.
    ///
    /// Must be balanced by a call to [`FormBuilder::pop`].
    pub fn push_segment(&mut self, name: &str) {
        self.stack.push(self.prefix.len());
        if self.prefix.is_empty() {
            percent_encode_into(&mut self.prefix, name);
        } else {
            self.prefix.push('[');
            percent_encode_into(&mut self.prefix, name);
            self.prefix.push(']');
        }
    }

    /// Pops the most recent key segment.
    pub fn pop(&mut self) {
        let len = self.stack.pop().unwrap_or(0);
        self.prefix.truncate(len);
    }

    /// Encodes `value` under the field `name`.
    pub fn field<T: FormEncode + ?Sized>(&mut self, name: &str, value: &T) {
        self.push_segment(name);
        value.encode_form(self);
        self.pop();
    }

    /// Encodes `value` under `name` if it is present.
    pub fn field_opt<T: FormEncode + ?Sized>(&mut self, name: &str, value: Option<&T>) {
        if let Some(value) = value {
            self.field(name, value);
        }
    }

    /// Encodes an element of an array of objects or arrays: `name[i]=...`.
    pub fn field_indexed<T: FormEncode + ?Sized>(&mut self, name: &str, index: usize, value: &T) {
        self.push_segment(name);
        self.prefix.push('[');
        let _ = write!(self.prefix, "{index}");
        self.prefix.push(']');
        value.encode_form(self);
        self.pop();
    }

    /// Encodes entries of a map field: `name[key]=value`.
    pub fn field_map<T: FormEncode + ?Sized>(&mut self, name: &str, key: &str, value: &T) {
        self.push_segment(name);
        self.push_segment(key);
        value.encode_form(self);
        self.pop();
        self.pop();
    }

    /// Encodes a scalar array field using `[]` notation: `name[]=value`.
    pub fn field_array_str(&mut self, name: &str, values: &[String]) {
        self.push_segment(name);
        for value in values {
            self.set_array_item(value);
        }
        self.pop();
    }

    /// Encodes an array of form-encodable scalars: `name[]=value`.
    pub fn field_array<T: FormEncode>(&mut self, name: &str, values: &[T]) {
        self.push_segment(name);
        for value in values {
            self.set_array_item_encoded(value);
        }
        self.pop();
    }

    /// Encodes an array by repeating the key: `name=value&name=value`.
    ///
    /// The v2 JSON APIs use this style for array query parameters, where the v1
    /// APIs use `name[]=value`.
    pub fn field_repeated<T: FormEncode>(&mut self, name: &str, values: &[T]) {
        self.push_segment(name);
        for value in values.iter() {
            value.encode_form(self);
        }
        self.pop();
    }

    /// Writes a single `prefix[]=value` entry using the current prefix.
    pub fn set_array_item_encoded<T: FormEncode + ?Sized>(&mut self, value: &T) {
        let saved = self.prefix.len();
        self.prefix.push_str("[]");
        value.encode_form(self);
        self.prefix.truncate(saved);
    }

    /// Encodes an explicitly empty field (`name=`), Stripe's way of clearing a
    /// value.
    pub fn field_empty(&mut self, name: &str) {
        self.push_segment(name);
        self.set_empty();
        self.pop();
    }

    /// Writes `prefix=value` with `value` percent-encoded.
    pub fn set_str(&mut self, value: &str) {
        self.start_pair();
        self.buf.push_str(&self.prefix);
        self.buf.push('=');
        percent_encode_into(&mut self.buf, value);
    }

    /// Writes `prefix=` (an explicit empty value).
    pub fn set_empty(&mut self) {
        self.start_pair();
        self.buf.push_str(&self.prefix);
        self.buf.push('=');
    }

    /// Writes `prefix=value` for any [`fmt::Display`] value.
    pub fn set_display(&mut self, value: impl fmt::Display) {
        self.start_pair();
        self.buf.push_str(&self.prefix);
        self.buf.push('=');
        let _ = write!(self.buf, "{value}");
    }

    fn set_array_item(&mut self, value: &str) {
        self.start_pair();
        self.buf.push_str(&self.prefix);
        self.buf.push_str("[]=");
        percent_encode_into(&mut self.buf, value);
    }
}

/// The default output capacity reserved by [`FormBuilder::with_capacity`].
pub(crate) const REQUEST_FORM_CAPACITY: usize = 256;

/// Encodes a value into a fresh form/query string.
pub fn encode<T: FormEncode + ?Sized>(value: &T) -> String {
    let mut form = FormBuilder::with_capacity(REQUEST_FORM_CAPACITY);
    value.encode_form(&mut form);
    form.into_string()
}

impl FormEncode for str {
    fn encode_form(&self, form: &mut FormBuilder) {
        form.set_str(self);
    }
}

impl FormEncode for String {
    fn encode_form(&self, form: &mut FormBuilder) {
        form.set_str(self);
    }
}

impl FormEncode for bool {
    fn encode_form(&self, form: &mut FormBuilder) {
        form.set_display(self);
    }
}

macro_rules! impl_form_encode_int {
    ($($ty:ty),* $(,)?) => {
        $(
            impl FormEncode for $ty {
                fn encode_form(&self, form: &mut FormBuilder) {
                    form.set_display(self);
                }
            }
        )*
    };
}

impl_form_encode_int!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);

impl FormEncode for f64 {
    fn encode_form(&self, form: &mut FormBuilder) {
        form.set_display(self);
    }
}

impl FormEncode for f32 {
    fn encode_form(&self, form: &mut FormBuilder) {
        form.set_display(self);
    }
}

impl<T: FormEncode + ?Sized> FormEncode for &T {
    fn encode_form(&self, form: &mut FormBuilder) {
        (**self).encode_form(form);
    }
}

/// A parameter type with no fields, used by operations that take none.
#[derive(Debug, Clone, Copy, Default, serde::Serialize)]
pub struct Empty;

impl FormEncode for Empty {
    fn encode_form(&self, _form: &mut FormBuilder) {}
}

/// Substitutes `{name}` placeholders in a request path template.
///
/// Each value is percent-encoded, so path arguments cannot inject extra path
/// segments or query parameters.
pub fn format_path(template: &str, args: &[(&str, &str)]) -> String {
    let mut out = String::with_capacity(template.len() + 16);
    let mut rest = template;
    while let Some(open) = rest.find('{') {
        out.push_str(&rest[..open]);
        let after = &rest[open + 1..];
        match after.find('}') {
            Some(close) => {
                let name = &after[..close];
                if let Some((_, value)) = args.iter().find(|(key, _)| *key == name) {
                    percent_encode_into(&mut out, value);
                }
                rest = &after[close + 1..];
            }
            None => {
                out.push('{');
                rest = after;
            }
        }
    }
    out.push_str(rest);
    out
}

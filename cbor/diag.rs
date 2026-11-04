//! RFC 8949 diagnostic notation.

use core::fmt;

use crate::decode::Decoder;
#[cfg(feature = "alloc")]
use crate::value::Value;

/// Render one CBOR item as diagnostic notation.
///
/// Decoding errors are rendered inline as `!<error>` so a partially valid
/// input is still useful for debugging.
pub fn to_diagnostic<W: fmt::Write>(bytes: &[u8], out: &mut W) -> fmt::Result {
    let mut decoder = Decoder::new(bytes);
    let mut printer = Printer {
        decoder: &mut decoder,
        out,
    };
    printer.item(0)
}

/// Render one CBOR item as diagnostic notation into a new `String`.
#[cfg(feature = "alloc")]
pub fn to_diagnostic_string(bytes: &[u8]) -> alloc::string::String {
    let mut out = alloc::string::String::new();
    let _ = to_diagnostic(bytes, &mut out);
    out
}

struct Printer<'a, 'b, W> {
    decoder: &'a mut Decoder<'b>,
    out: &'a mut W,
}

impl<W: fmt::Write> Printer<'_, '_, W> {
    fn item(&mut self, depth: usize) -> fmt::Result {
        if depth > 128 {
            return self.out.write_str("...");
        }
        let ty = match self.decoder.datatype() {
            Ok(ty) => ty,
            Err(error) => {
                write!(self.out, "!{error}")?;
                return Ok(());
            }
        };
        use crate::data::Type::*;
        match ty {
            Null => {
                self.decoder.null().map_err(|_| fmt::Error)?;
                self.out.write_str("null")
            }
            Undefined => {
                self.decoder.undefined().map_err(|_| fmt::Error)?;
                self.out.write_str("undefined")
            }
            Bool => {
                let value = self.decoder.bool().map_err(|_| fmt::Error)?;
                self.out.write_str(if value { "true" } else { "false" })
            }
            U8 | U16 | U32 | U64 | I8 | I16 | I32 | I64 => {
                let value = self.decoder.int().map_err(|_| fmt::Error)?;
                write!(self.out, "{value}")
            }
            F16 | F32 | F64 => {
                let value = self.decoder.f64().map_err(|_| fmt::Error)?;
                write_float(self.out, value)
            }
            Simple => {
                let value = self.decoder.simple().map_err(|_| fmt::Error)?;
                write!(self.out, "simple({})", value.get())
            }
            Bytes => {
                let bytes = self.decoder.bytes().map_err(|_| fmt::Error)?;
                write_bytes(self.out, bytes)
            }
            BytesIndef => {
                self.out.write_str("(_ ")?;
                let iter = self.decoder.bytes_iter().map_err(|_| fmt::Error)?;
                let mut first = true;
                for chunk in iter {
                    let chunk = chunk.map_err(|_| fmt::Error)?;
                    if !first {
                        self.out.write_str(", ")?;
                    }
                    first = false;
                    write_bytes(self.out, chunk)?;
                }
                self.out.write_str(")")
            }
            Text => {
                let text = self.decoder.text().map_err(|_| fmt::Error)?;
                write_text(self.out, text)
            }
            TextIndef => {
                self.out.write_str("(_ ")?;
                let iter = self.decoder.text_iter().map_err(|_| fmt::Error)?;
                let mut first = true;
                for chunk in iter {
                    let chunk = chunk.map_err(|_| fmt::Error)?;
                    if !first {
                        self.out.write_str(", ")?;
                    }
                    first = false;
                    write_text(self.out, chunk)?;
                }
                self.out.write_str(")")
            }
            Array => {
                let len = self.decoder.array().map_err(|_| fmt::Error)?.unwrap_or(0);
                self.out.write_str("[")?;
                for i in 0..len {
                    if i > 0 {
                        self.out.write_str(", ")?;
                    }
                    self.item(depth + 1)?;
                }
                self.out.write_str("]")
            }
            ArrayIndef => {
                let _ = self.decoder.array().map_err(|_| fmt::Error)?;
                self.out.write_str("[_ ")?;
                let mut first = true;
                while !self.decoder.is_break() {
                    if !first {
                        self.out.write_str(", ")?;
                    }
                    first = false;
                    let before = self.decoder.position();
                    self.item(depth + 1)?;
                    if self.decoder.position() == before {
                        return Err(fmt::Error);
                    }
                }
                self.decoder.read_break().map_err(|_| fmt::Error)?;
                self.out.write_str("]")
            }
            Map => {
                let len = self.decoder.map().map_err(|_| fmt::Error)?.unwrap_or(0);
                self.out.write_str("{")?;
                for i in 0..len {
                    if i > 0 {
                        self.out.write_str(", ")?;
                    }
                    self.item(depth + 1)?;
                    self.out.write_str(": ")?;
                    self.item(depth + 1)?;
                }
                self.out.write_str("}")
            }
            MapIndef => {
                let _ = self.decoder.map().map_err(|_| fmt::Error)?;
                self.out.write_str("{_ ")?;
                let mut first = true;
                while !self.decoder.is_break() {
                    if !first {
                        self.out.write_str(", ")?;
                    }
                    first = false;
                    let before = self.decoder.position();
                    self.item(depth + 1)?;
                    if self.decoder.position() == before {
                        return Err(fmt::Error);
                    }
                    self.out.write_str(": ")?;
                    let before = self.decoder.position();
                    self.item(depth + 1)?;
                    if self.decoder.position() == before {
                        return Err(fmt::Error);
                    }
                }
                self.decoder.read_break().map_err(|_| fmt::Error)?;
                self.out.write_str("}")
            }
            Tag => {
                let tag = self.decoder.tag().map_err(|_| fmt::Error)?;
                write!(self.out, "{}(", tag.get())?;
                self.item(depth + 1)?;
                self.out.write_str(")")
            }
            Break => self.out.write_str("break"),
            Unknown(b) => write!(self.out, "unknown(0x{b:02x})"),
        }
    }
}

/// Render an in-memory [`Value`] as diagnostic notation.
#[cfg(feature = "alloc")]
pub fn write_value<W: fmt::Write>(value: &Value, out: &mut W) -> fmt::Result {
    match value {
        Value::Null => out.write_str("null"),
        Value::Undefined => out.write_str("undefined"),
        Value::Bool(v) => out.write_str(if *v { "true" } else { "false" }),
        Value::Integer(v) => write!(out, "{v}"),
        Value::Bignum(b) => {
            let tag = if b.negative { 3 } else { 2 };
            write!(out, "{tag}(h'")?;
            for byte in &b.bytes {
                write!(out, "{byte:02x}")?;
            }
            out.write_str("')")
        }
        Value::Float(v) => write_float(out, *v),
        Value::Bytes(bytes) => write_bytes(out, bytes),
        Value::Text(text) => write_text(out, text),
        Value::Array(values) => {
            out.write_str("[")?;
            for (i, value) in values.iter().enumerate() {
                if i > 0 {
                    out.write_str(", ")?;
                }
                write_value(value, out)?;
            }
            out.write_str("]")
        }
        Value::Map(map) => {
            out.write_str("{")?;
            for (i, (key, value)) in map.iter().enumerate() {
                if i > 0 {
                    out.write_str(", ")?;
                }
                write_value(key, out)?;
                out.write_str(": ")?;
                write_value(value, out)?;
            }
            out.write_str("}")
        }
        Value::Tag(tag, value) => {
            write!(out, "{tag}(")?;
            write_value(value, out)?;
            out.write_str(")")
        }
        Value::Simple(simple) => write!(out, "simple({})", simple.get()),
    }
}

fn write_float<W: fmt::Write>(out: &mut W, value: f64) -> fmt::Result {
    if value.is_nan() {
        out.write_str("NaN")
    } else if value == f64::INFINITY {
        out.write_str("Infinity")
    } else if value == f64::NEG_INFINITY {
        out.write_str("-Infinity")
    } else {
        write!(out, "{value}")
    }
}

fn write_bytes<W: fmt::Write>(out: &mut W, bytes: &[u8]) -> fmt::Result {
    out.write_str("h'")?;
    for byte in bytes {
        write!(out, "{byte:02x}")?;
    }
    out.write_str("'")
}

fn write_text<W: fmt::Write>(out: &mut W, text: &str) -> fmt::Result {
    out.write_str("\"")?;
    for c in text.chars() {
        match c {
            '"' => out.write_str("\\\"")?,
            '\\' => out.write_str("\\\\")?,
            '\n' => out.write_str("\\n")?,
            '\r' => out.write_str("\\r")?,
            '\t' => out.write_str("\\t")?,
            c if (c as u32) < 0x20 => write!(out, "\\u{:04x}", c as u32)?,
            c => out.write_char(c)?,
        }
    }
    out.write_str("\"")
}

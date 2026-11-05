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
    fn write_error(&mut self, error: crate::Error) -> fmt::Result {
        write!(self.out, "!{error}")
    }

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
            Null => match self.decoder.null() {
                Ok(()) => self.out.write_str("null"),
                Err(error) => self.write_error(error),
            },
            Undefined => match self.decoder.undefined() {
                Ok(()) => self.out.write_str("undefined"),
                Err(error) => self.write_error(error),
            },
            Bool => match self.decoder.bool() {
                Ok(value) => self.out.write_str(if value { "true" } else { "false" }),
                Err(error) => self.write_error(error),
            },
            U8 | U16 | U32 | U64 | I8 | I16 | I32 | I64 => match self.decoder.int() {
                Ok(value) => write!(self.out, "{value}"),
                Err(error) => self.write_error(error),
            },
            F16 | F32 | F64 => match self.decoder.f64() {
                Ok(value) => write_float(self.out, value),
                Err(error) => self.write_error(error),
            },
            Simple => match self.decoder.simple() {
                Ok(value) => write!(self.out, "simple({})", value.get()),
                Err(error) => self.write_error(error),
            },
            Bytes => match self.decoder.bytes() {
                Ok(bytes) => write_bytes(self.out, bytes),
                Err(error) => self.write_error(error),
            },
            BytesIndef => {
                self.out.write_str("(_ ")?;
                let iter = match self.decoder.bytes_iter() {
                    Ok(iter) => iter,
                    Err(error) => {
                        self.write_error(error)?;
                        return self.out.write_str(")");
                    }
                };
                let mut first = true;
                for chunk in iter {
                    if !first {
                        self.out.write_str(", ")?;
                    }
                    first = false;
                    match chunk {
                        Ok(chunk) => write_bytes(self.out, chunk)?,
                        Err(error) => {
                            self.write_error(error)?;
                            break;
                        }
                    }
                }
                self.out.write_str(")")
            }
            Text => match self.decoder.text() {
                Ok(text) => write_text(self.out, text),
                Err(error) => self.write_error(error),
            },
            TextIndef => {
                self.out.write_str("(_ ")?;
                let iter = match self.decoder.text_iter() {
                    Ok(iter) => iter,
                    Err(error) => {
                        self.write_error(error)?;
                        return self.out.write_str(")");
                    }
                };
                let mut first = true;
                for chunk in iter {
                    if !first {
                        self.out.write_str(", ")?;
                    }
                    first = false;
                    match chunk {
                        Ok(chunk) => write_text(self.out, chunk)?,
                        Err(error) => {
                            self.write_error(error)?;
                            break;
                        }
                    }
                }
                self.out.write_str(")")
            }
            Array => {
                let len = match self.decoder.array() {
                    Ok(Some(len)) => len,
                    Ok(None) => 0,
                    Err(error) => return self.write_error(error),
                };
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
                if let Err(error) = self.decoder.array() {
                    return self.write_error(error);
                }
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
                        self.out.write_str("...")?;
                        break;
                    }
                }
                match self.decoder.read_break() {
                    Ok(()) => self.out.write_str("]"),
                    Err(error) => {
                        self.write_error(error)?;
                        self.out.write_str("]")
                    }
                }
            }
            Map => {
                let len = match self.decoder.map() {
                    Ok(Some(len)) => len,
                    Ok(None) => 0,
                    Err(error) => return self.write_error(error),
                };
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
                if let Err(error) = self.decoder.map() {
                    return self.write_error(error);
                }
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
                        self.out.write_str("...")?;
                        break;
                    }
                    self.out.write_str(": ")?;
                    let before = self.decoder.position();
                    self.item(depth + 1)?;
                    if self.decoder.position() == before {
                        self.out.write_str("...")?;
                        break;
                    }
                }
                match self.decoder.read_break() {
                    Ok(()) => self.out.write_str("}"),
                    Err(error) => {
                        self.write_error(error)?;
                        self.out.write_str("}")
                    }
                }
            }
            Tag => match self.decoder.tag() {
                Ok(tag) => {
                    write!(self.out, "{}(", tag.get())?;
                    self.item(depth + 1)?;
                    self.out.write_str(")")
                }
                Err(error) => self.write_error(error),
            },
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
        // Debug preserves the float/integer distinction (`1.0`, `-0.0`) and
        // uses a compact exponent form for very large or small values.
        write!(out, "{value:?}")
    }
}

fn write_bytes<W: fmt::Write>(out: &mut W, bytes: &[u8]) -> fmt::Result {
    out.write_str("h'")?;
    const CHUNK: usize = 256;
    let mut buffer = [0u8; CHUNK * 2];
    for chunk in bytes.chunks(CHUNK) {
        let len = chunk.len() * 2;
        hex::encode_into(&mut buffer[..len], chunk, hex::Alphabet::Lower).map_err(|_| fmt::Error)?;
        out.write_str(core::str::from_utf8(&buffer[..len]).map_err(|_| fmt::Error)?)?;
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
            c if c.is_control() => write!(out, "\\u{:04x}", c as u32)?,
            c => out.write_char(c)?,
        }
    }
    out.write_str("\"")
}

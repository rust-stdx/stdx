//! Safe wrapper around the vendored libyaml parser and emitter.

#![allow(dead_code, clippy::all)]

mod cstr;
mod emitter;
pub(crate) use self::emitter::{
    Emitter, Event as EmitEvent, Mapping, Scalar as EmitScalar, ScalarStyle as EmitScalarStyle, Sequence,
};
pub mod error;
pub mod parser;
pub mod tag;
mod util;

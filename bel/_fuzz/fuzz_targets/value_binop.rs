#![no_main]

use std::hint::black_box;
use std::sync::Arc;

use bel::Value;
use libfuzzer_sys::fuzz_target;

#[derive(Debug, arbitrary::Arbitrary)]
enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Eq,
    Cmp,
}

#[derive(Debug, arbitrary::Arbitrary)]
enum ArbValue {
    Int(i64),
    Float(f64),
    String(String),
    Bytes(Vec<u8>),
    Bool(bool),
    Null,
}

impl From<ArbValue> for Value {
    fn from(value: ArbValue) -> Self {
        match value {
            ArbValue::Int(value) => Value::Int(value),
            ArbValue::Float(value) => Value::Float(value),
            ArbValue::String(value) => Value::String(Arc::new(value)),
            ArbValue::Bytes(value) => Value::Bytes(Arc::new(value)),
            ArbValue::Bool(value) => Value::Bool(value),
            ArbValue::Null => Value::Null,
        }
    }
}

#[derive(Debug, arbitrary::Arbitrary)]
struct Input {
    op: BinOp,
    lhs: ArbValue,
    rhs: ArbValue,
}

// Ensure that the binary operators on `Value` do not panic,
// c.f. https://github.com/bloom42/bel-rs/pull/145.
fuzz_target!(|input: Input| {
    let lhs: Value = input.lhs.into();
    let rhs: Value = input.rhs.into();
    match input.op {
        BinOp::Add => _ = black_box(lhs + rhs),
        BinOp::Sub => _ = black_box(lhs - rhs),
        BinOp::Mul => _ = black_box(lhs * rhs),
        BinOp::Div => _ = black_box(lhs / rhs),
        BinOp::Rem => _ = black_box(lhs % rhs),
        BinOp::Eq => _ = black_box(lhs == rhs),
        BinOp::Cmp => _ = black_box(lhs.partial_cmp(&rhs)),
    }
});

//! Compile-time result-shape verification.
//!
//! [`RowShape`] describes the columns a target type expects. The `query_as!`
//! macro compares it against the columns the server reports, at compile time,
//! so a mismatch is a compile error rather than a runtime decode failure.

use crate::{
    encode::StaticType,
    interval::Interval,
    json::Json,
    types::{
        BPCHAROID, BYTEAOID, CIDROID, DATEOID, INETOID, JSONBOID, JSONOID, NAMEOID, Oid, TEXTOID, TIMEOID,
        TIMESTAMPOID, TIMESTAMPTZOID, TIMETZOID, VARCHAROID, array_oid_checked,
    },
};

/// One expected result column.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColumnSpec {
    /// Expected column name; empty means "any name" (positional match).
    pub name: &'static str,
    /// The canonical OID of the Rust type.
    pub type_oid: Oid,
    /// Whether the Rust type is nullable (`Option<...>`).
    pub nullable: bool,
    /// Whether `nullable` is known. For server-side expression columns the
    /// nullability cannot be derived, so it is not enforced.
    pub known: bool,
}

/// A type whose result columns can be checked at compile time.
pub trait RowShape {
    /// The columns this type expects.
    const SHAPE: &'static [ColumnSpec];
}

const fn str_eq(a: &str, b: &str) -> bool {
    let a = a.as_bytes();
    let b = b.as_bytes();
    if a.len() != b.len() {
        return false;
    }
    let mut i = 0;
    while i < a.len() {
        if a[i] != b[i] {
            return false;
        }
        i += 1;
    }
    true
}

/// Returns `true` if a value with canonical OID `rust_oid` can be decoded from
/// a column of type `column_oid`.
///
/// PostgreSQL is loose about a few equivalences (for example `varchar` and
/// `text` both decode to `String`), which are accounted for here.
pub const fn decodes_from(rust_oid: Oid, column_oid: Oid) -> bool {
    if rust_oid == column_oid {
        return true;
    }
    match rust_oid {
        TEXTOID => matches!(column_oid, VARCHAROID | BPCHAROID | NAMEOID),
        TIMESTAMPTZOID => matches!(column_oid, DATEOID | TIMESTAMPOID | TIMEOID | TIMETZOID),
        JSONBOID => column_oid == JSONOID,
        INETOID => column_oid == CIDROID,
        _ => false,
    }
}

/// Verifies that a described result set matches the target type's shape.
///
/// Evaluated at compile time by the `query_as!` macro; a mismatch panics during
/// constant evaluation, producing a compile error.
pub const fn verify_shape(expected: &[ColumnSpec], actual: &[ColumnSpec]) {
    if expected.len() != actual.len() {
        panic!("postgresql: the query returns a different number of columns than the target type expects");
    }
    let mut i = 0;
    while i < expected.len() {
        let e = expected[i];
        let a = actual[i];
        if !decodes_from(e.type_oid, a.type_oid) {
            panic!("postgresql: a result column has a type incompatible with its target field");
        }
        if e.nullable != a.nullable && a.known {
            panic!(
                "postgresql: a result column's nullability does not match its target field (wrap it in Option or make the column NOT NULL)"
            );
        }
        if !e.name.is_empty() && !str_eq(e.name, a.name) {
            panic!("postgresql: a result column name does not match its target field name");
        }
        i += 1;
    }
}

macro_rules! scalar_shape {
    ($($ty:ty),* $(,)?) => {
        $(
            impl RowShape for $ty {
                const SHAPE: &'static [ColumnSpec] = &[ColumnSpec {
                    name: "",
                    type_oid: <$ty as StaticType>::OID,
                    nullable: <$ty as StaticType>::NULLABLE,
                    known: true,
                }];
            }
        )*
    };
}

scalar_shape!(
    i16,
    i32,
    i64,
    f32,
    f64,
    bool,
    String,
    uuid::Uuid,
    time::DateTime,
    serde_json::Value,
    ipnetwork::IpNetwork,
    Interval,
);

impl<T: StaticType> RowShape for Option<T> {
    const SHAPE: &'static [ColumnSpec] = &[ColumnSpec {
        name: "",
        type_oid: T::OID,
        nullable: true,
        known: true,
    }];
}

impl<T: StaticType> RowShape for Vec<T> {
    const SHAPE: &'static [ColumnSpec] = &[ColumnSpec {
        name: "",
        type_oid: array_oid_checked(T::OID),
        nullable: false,
        known: true,
    }];
}

macro_rules! tuple_shape {
    ($($name:ident),+) => {
        impl<$($name: StaticType),+> RowShape for ($($name,)+) {
            const SHAPE: &'static [ColumnSpec] = &[
                $(ColumnSpec {
                    name: "",
                    type_oid: <$name as StaticType>::OID,
                    nullable: <$name as StaticType>::NULLABLE,
                    known: true,
                },)+
            ];
        }
    };
}

tuple_shape!(A);
tuple_shape!(A, B);
tuple_shape!(A, B, C);
tuple_shape!(A, B, C, D);
tuple_shape!(A, B, C, D, E);
tuple_shape!(A, B, C, D, E, F);
tuple_shape!(A, B, C, D, E, F, G);
tuple_shape!(A, B, C, D, E, F, G, H);

impl<T> RowShape for Json<T> {
    const SHAPE: &'static [ColumnSpec] = &[ColumnSpec {
        name: "",
        type_oid: JSONBOID,
        nullable: false,
        known: true,
    }];
}

// `Vec<u8>` is BYTEA, not an array of `u8` (which has no PostgreSQL type).
impl RowShape for Vec<u8> {
    const SHAPE: &'static [ColumnSpec] = &[ColumnSpec {
        name: "",
        type_oid: BYTEAOID,
        nullable: false,
        known: true,
    }];
}

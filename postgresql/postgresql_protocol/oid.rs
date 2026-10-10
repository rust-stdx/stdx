//! PostgreSQL type object identifiers and the binary/text format marker.

use std::fmt;

/// A PostgreSQL type object identifier (OID).
pub type Oid = u32;

/// `bool`
pub const BOOLOID: Oid = 16;
/// `bytea`
pub const BYTEAOID: Oid = 17;
/// `name`
pub const NAMEOID: Oid = 19;
/// `int8`
pub const INT8OID: Oid = 20;
/// `int2`
pub const INT2OID: Oid = 21;
/// `int4`
pub const INT4OID: Oid = 23;
/// `text`
pub const TEXTOID: Oid = 25;
/// `oid`
pub const OIDOID: Oid = 26;
/// `json`
pub const JSONOID: Oid = 114;
/// `float4`
pub const FLOAT4OID: Oid = 700;
/// `float8`
pub const FLOAT8OID: Oid = 701;
/// `inet`
pub const INETOID: Oid = 869;
/// `cidr`
pub const CIDROID: Oid = 650;
/// `varchar`
pub const VARCHAROID: Oid = 1043;
/// `bpchar` (blank-padded `char(n)`)
pub const BPCHAROID: Oid = 1042;
/// `date`
pub const DATEOID: Oid = 1082;
/// `time`
pub const TIMEOID: Oid = 1083;
/// `timestamp` (without time zone)
pub const TIMESTAMPOID: Oid = 1114;
/// `timestamptz`
pub const TIMESTAMPTZOID: Oid = 1184;
/// `interval`
pub const INTERVALOID: Oid = 1186;
/// `timetz`
pub const TIMETZOID: Oid = 1266;
/// `numeric`
pub const NUMERICOID: Oid = 1700;
/// `uuid`
pub const UUIDOID: Oid = 2950;
/// `jsonb`
pub const JSONBOID: Oid = 3802;

/// `name[]`
pub const NAME_ARRAY_OID: Oid = 1003;
/// `bpchar[]`
pub const BPCHAR_ARRAY_OID: Oid = 1014;
/// `int2[]`
pub const INT2_ARRAY_OID: Oid = 1005;
/// `int4[]`
pub const INT4_ARRAY_OID: Oid = 1007;
/// `text[]`
pub const TEXT_ARRAY_OID: Oid = 1009;
/// `int8[]`
pub const INT8_ARRAY_OID: Oid = 1016;
/// `float4[]`
pub const FLOAT4_ARRAY_OID: Oid = 1021;
/// `float8[]`
pub const FLOAT8_ARRAY_OID: Oid = 1022;
/// `bool[]`
pub const BOOL_ARRAY_OID: Oid = 1000;
/// `bytea[]`
pub const BYTEA_ARRAY_OID: Oid = 1001;
/// `uuid[]`
pub const UUID_ARRAY_OID: Oid = 2951;
/// `timestamptz[]`
pub const TIMESTAMPTZ_ARRAY_OID: Oid = 1185;
/// `date[]`
pub const DATE_ARRAY_OID: Oid = 1182;
/// `timestamp[]`
pub const TIMESTAMP_ARRAY_OID: Oid = 1115;
/// `varchar[]`
pub const VARCHAR_ARRAY_OID: Oid = 1015;
/// `json[]`
pub const JSON_ARRAY_OID: Oid = 199;
/// `inet[]`
pub const INET_ARRAY_OID: Oid = 1041;
/// `cidr[]`
pub const CIDR_ARRAY_OID: Oid = 651;
/// `time[]`
pub const TIME_ARRAY_OID: Oid = 1183;
/// `timetz[]`
pub const TIMETZ_ARRAY_OID: Oid = 1270;
/// `interval[]`
pub const INTERVAL_ARRAY_OID: Oid = 1187;
/// `numeric[]`
pub const NUMERIC_ARRAY_OID: Oid = 1231;
/// `jsonb[]`
pub const JSONB_ARRAY_OID: Oid = 3807;

/// The wire format of a value: text or binary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    /// Human-readable text encoding.
    Text,
    /// PostgreSQL binary encoding.
    Binary,
}

impl Format {
    /// The protocol code for this format (`0` = text, `1` = binary).
    pub const fn code(self) -> i16 {
        match self {
            Format::Text => 0,
            Format::Binary => 1,
        }
    }
}

/// Returns the array type OID for the given element type.
///
/// Returns `None` for a type that has no array counterpart here (for example
/// `interval` before it was added, or any extension type): silently claiming
/// `int4[]` would make the server interpret the values as the wrong type.
pub const fn array_oid(elem: Oid) -> Option<Oid> {
    Some(match elem {
        INT2OID => INT2_ARRAY_OID,
        INT4OID => INT4_ARRAY_OID,
        INT8OID => INT8_ARRAY_OID,
        FLOAT4OID => FLOAT4_ARRAY_OID,
        FLOAT8OID => FLOAT8_ARRAY_OID,
        BOOLOID => BOOL_ARRAY_OID,
        TEXTOID => TEXT_ARRAY_OID,
        NAMEOID => NAME_ARRAY_OID,
        VARCHAROID => VARCHAR_ARRAY_OID,
        BPCHAROID => BPCHAR_ARRAY_OID,
        BYTEAOID => BYTEA_ARRAY_OID,
        UUIDOID => UUID_ARRAY_OID,
        JSONOID => JSON_ARRAY_OID,
        JSONBOID => JSONB_ARRAY_OID,
        INETOID => INET_ARRAY_OID,
        CIDROID => CIDR_ARRAY_OID,
        DATEOID => DATE_ARRAY_OID,
        TIMEOID => TIME_ARRAY_OID,
        TIMETZOID => TIMETZ_ARRAY_OID,
        TIMESTAMPTZOID => TIMESTAMPTZ_ARRAY_OID,
        TIMESTAMPOID => TIMESTAMP_ARRAY_OID,
        INTERVALOID => INTERVAL_ARRAY_OID,
        NUMERICOID => NUMERIC_ARRAY_OID,
        _ => return None,
    })
}

/// Like [`array_oid`], for use in `const` contexts where a missing mapping has
/// to fail at compile time instead of silently declaring the wrong type.
///
/// # Panics
///
/// Panics during constant evaluation when `elem` has no array counterpart.
pub const fn array_oid_checked(elem: Oid) -> Oid {
    match array_oid(elem) {
        Some(oid) => oid,
        None => panic!("this type has no PostgreSQL array type"),
    }
}

/// A PostgreSQL type: its OID and canonical name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PgType {
    /// The type's object identifier.
    pub oid: Oid,
    /// The canonical PostgreSQL name (e.g. `int4`, `_int4`).
    pub name: &'static str,
}

impl PgType {
    /// Creates a new type descriptor.
    pub const fn new(oid: Oid, name: &'static str) -> Self {
        PgType {
            oid,
            name,
        }
    }

    /// Returns the array type whose element type is `self`.
    ///
    /// # Panics
    ///
    /// Panics when this type has no array counterpart; use
    /// [`array_oid`] to handle that case.
    pub const fn array_of(self) -> PgType {
        let Some(oid) = array_oid(self.oid) else {
            panic!("this type has no array type")
        };
        PgType {
            oid,
            name: "",
        }
    }
}

impl fmt::Display for PgType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

/// `int2`
pub static INT2: PgType = PgType::new(INT2OID, "int2");
/// `int4`
pub static INT4: PgType = PgType::new(INT4OID, "int4");
/// `int8`
pub static INT8: PgType = PgType::new(INT8OID, "int8");
/// `float4`
pub static FLOAT4: PgType = PgType::new(FLOAT4OID, "float4");
/// `float8`
pub static FLOAT8: PgType = PgType::new(FLOAT8OID, "float8");
/// `bool`
pub static BOOL: PgType = PgType::new(BOOLOID, "bool");
/// `text`
pub static TEXT: PgType = PgType::new(TEXTOID, "text");
/// `varchar`
pub static VARCHAR: PgType = PgType::new(VARCHAROID, "varchar");
/// `bytea`
pub static BYTEA: PgType = PgType::new(BYTEAOID, "bytea");
/// `uuid`
pub static UUID: PgType = PgType::new(UUIDOID, "uuid");
/// `json`
pub static JSON: PgType = PgType::new(JSONOID, "json");
/// `jsonb`
pub static JSONB: PgType = PgType::new(JSONBOID, "jsonb");
/// `inet`
pub static INET: PgType = PgType::new(INETOID, "inet");
/// `cidr`
pub static CIDR: PgType = PgType::new(CIDROID, "cidr");
/// `date`
pub static DATE: PgType = PgType::new(DATEOID, "date");
/// `time`
pub static TIME: PgType = PgType::new(TIMEOID, "time");
/// `timetz`
pub static TIMETZ: PgType = PgType::new(TIMETZOID, "timetz");
/// `timestamp`
pub static TIMESTAMP: PgType = PgType::new(TIMESTAMPOID, "timestamp");
/// `timestamptz`
pub static TIMESTAMPTZ: PgType = PgType::new(TIMESTAMPTZOID, "timestamptz");
/// `interval`
pub static INTERVAL: PgType = PgType::new(INTERVALOID, "interval");
/// `numeric`
pub static NUMERIC: PgType = PgType::new(NUMERICOID, "numeric");

/// Maps an array type OID back to its element type, when known.
pub const fn array_element(oid: Oid) -> Option<PgType> {
    let elem = match oid {
        INT2_ARRAY_OID => INT2OID,
        INT4_ARRAY_OID => INT4OID,
        INT8_ARRAY_OID => INT8OID,
        FLOAT4_ARRAY_OID => FLOAT4OID,
        FLOAT8_ARRAY_OID => FLOAT8OID,
        BOOL_ARRAY_OID => BOOLOID,
        TEXT_ARRAY_OID => TEXTOID,
        NAME_ARRAY_OID => NAMEOID,
        VARCHAR_ARRAY_OID => VARCHAROID,
        BPCHAR_ARRAY_OID => BPCHAROID,
        BYTEA_ARRAY_OID => BYTEAOID,
        UUID_ARRAY_OID => UUIDOID,
        JSON_ARRAY_OID => JSONOID,
        JSONB_ARRAY_OID => JSONBOID,
        INET_ARRAY_OID => INETOID,
        CIDR_ARRAY_OID => CIDROID,
        DATE_ARRAY_OID => DATEOID,
        TIME_ARRAY_OID => TIMEOID,
        TIMETZ_ARRAY_OID => TIMETZOID,
        TIMESTAMPTZ_ARRAY_OID => TIMESTAMPTZOID,
        TIMESTAMP_ARRAY_OID => TIMESTAMPOID,
        INTERVAL_ARRAY_OID => INTERVALOID,
        NUMERIC_ARRAY_OID => NUMERICOID,
        _ => return None,
    };
    Some(PgType::new(elem, ""))
}

/// Returns `true` if `oid` is a known array type.
pub const fn is_array(oid: Oid) -> bool {
    array_element(oid).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn array_of_known() {
        assert_eq!(INT2.array_of().oid, INT2_ARRAY_OID);
        assert_eq!(INT4.array_of().oid, INT4_ARRAY_OID);
        assert_eq!(TEXT.array_of().oid, TEXT_ARRAY_OID);
        assert_eq!(VARCHAR.array_of().oid, VARCHAR_ARRAY_OID);
        assert_eq!(UUID.array_of().oid, UUID_ARRAY_OID);
        assert_eq!(TIMESTAMPTZ.array_of().oid, TIMESTAMPTZ_ARRAY_OID);
        assert_eq!(array_oid(INTERVALOID), Some(INTERVAL_ARRAY_OID));
        assert_eq!(array_oid(JSONBOID), Some(JSONB_ARRAY_OID));
        assert_eq!(array_oid(NAMEOID), Some(NAME_ARRAY_OID));
    }

    #[test]
    fn unknown_element_has_no_array_oid() {
        assert_eq!(array_oid(4242), None);
    }

    #[test]
    fn element_roundtrip() {
        assert_eq!(array_element(INT2_ARRAY_OID).unwrap().oid, INT2OID);
        assert!(array_element(TEXTOID).is_none());
        assert!(is_array(UUID_ARRAY_OID));
        assert!(!is_array(UUIDOID));
    }
}

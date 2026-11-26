#![cfg(feature = "derive")]

use yaml::{Decode, Encode, Value};

mod helper {
    pub fn encode<T>(_value: &T) -> yaml::Value {
        yaml::Value::Null
    }

    pub fn decode<T: Default>(_value: &yaml::Value) -> yaml::Result<T> {
        Ok(T::default())
    }
}

#[derive(Debug, Default, PartialEq)]
struct NotEncode(u32);

#[derive(Debug, PartialEq, Encode, Decode)]
#[yaml(bound = "T: Default")]
struct WithHelper<T> {
    #[yaml(with = "helper")]
    value: T,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct TupleDefault(u32, #[yaml(default)] u32);

#[derive(Debug, PartialEq, Encode, Decode)]
enum TupleVariantDefault {
    Value(u32, #[yaml(default)] u32),
}

#[derive(Debug, PartialEq)]
struct NoDefault(u32);

impl yaml::Encode for NoDefault {
    fn encode(&self) -> Value {
        self.0.encode()
    }
}

impl yaml::Decode for NoDefault {
    fn decode(value: &Value) -> yaml::Result<Self> {
        u32::decode(value).map(NoDefault)
    }
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum Named {
    V { x: u32, y: u32 },
    W { x: u32, y: u32 },
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum WithNonDefault {
    V { n: NoDefault },
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Generic<T> {
    value: T,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum GenericEnum<T> {
    Value(T),
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Skipped {
    #[yaml(skip)]
    hidden: u32,
    shown: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct TupleSkipped(#[yaml(skip)] u32, String);

#[derive(Debug, PartialEq, Encode, Decode)]
enum TupleVariantSkipped {
    Value(#[yaml(skip)] u32, String),
}

#[allow(non_snake_case)]
#[derive(Debug, PartialEq, Encode, Decode)]
#[yaml(rename_all = "camelCase")]
struct Acronym {
    HTTPPort: u32,
}

#[test]
fn enum_struct_variant_missing_fields_are_errors() {
    assert!(yaml::from_str::<Named>("V:\n  x: 7\n").is_err());
    let parsed: Named = yaml::from_str("V:\n  x: 7\n  y: 8\n").unwrap();
    assert_eq!(
        parsed,
        Named::V {
            x: 7,
            y: 8
        }
    );
}

#[test]
fn enum_struct_variant_fields_do_not_require_default() {
    let value = WithNonDefault::V {
        n: NoDefault(7),
    };
    let text = yaml::to_string(&value).unwrap();
    assert_eq!(yaml::from_str::<WithNonDefault>(&text).unwrap(), value);
}

#[test]
fn generic_types_get_trait_bounds_automatically() {
    let value = Generic {
        value: 7u32,
    };
    let text = yaml::to_string(&value).unwrap();
    assert_eq!(yaml::from_str::<Generic<u32>>(&text).unwrap(), value);

    let value = GenericEnum::Value("hello".to_string());
    let text = yaml::to_string(&value).unwrap();
    assert_eq!(yaml::from_str::<GenericEnum<String>>(&text).unwrap(), value);
}

#[test]
fn skip_ignores_input_and_omits_output() {
    let text = yaml::to_string(&Skipped {
        hidden: 5,
        shown: 7,
    })
    .unwrap();
    assert!(!text.contains("hidden"), "{text}");
    assert_eq!(
        yaml::from_str::<Skipped>("hidden: 5\nshown: 7\n").unwrap(),
        Skipped {
            hidden: 0,
            shown: 7,
        }
    );
}

#[test]
fn skip_works_on_tuple_structs_and_variants() {
    let text = yaml::to_string(&TupleSkipped(11, "hello".to_string())).unwrap();
    assert_eq!(
        yaml::from_str::<TupleSkipped>(&text).unwrap(),
        TupleSkipped(0, "hello".to_string())
    );

    let text = yaml::to_string(&TupleVariantSkipped::Value(11, "hello".to_string())).unwrap();
    assert_eq!(
        yaml::from_str::<TupleVariantSkipped>(&text).unwrap(),
        TupleVariantSkipped::Value(0, "hello".to_string())
    );
}

#[test]
fn externally_tagged_mappings_require_exactly_one_key() {
    let input = "V:\n  x: 1\n  y: 2\nW:\n  x: 3\n  y: 4\n";
    assert!(yaml::from_str::<Named>(input).is_err());
}

#[test]
fn with_fields_do_not_require_encode_or_decode_bounds() {
    let value = WithHelper {
        value: NotEncode(3),
    };
    let text = yaml::to_string(&value).unwrap();
    assert_eq!(
        yaml::from_str::<WithHelper<NotEncode>>(&text).unwrap(),
        WithHelper {
            value: NotEncode::default(),
        }
    );
}

#[test]
fn tuple_default_applies_to_trailing_missing_elements() {
    let value = TupleDefault(1, 2);
    let text = yaml::to_string(&value).unwrap();
    assert_eq!(yaml::from_str::<TupleDefault>(&text).unwrap(), value);
    assert_eq!(yaml::from_str::<TupleDefault>("- 1\n").unwrap(), TupleDefault(1, 0));
    assert!(yaml::from_str::<TupleDefault>("[]").is_err());

    let value = TupleVariantDefault::Value(1, 2);
    let text = yaml::to_string(&value).unwrap();
    assert_eq!(yaml::from_str::<TupleVariantDefault>(&text).unwrap(), value);
    assert_eq!(
        yaml::from_str::<TupleVariantDefault>("Value:\n- 1\n").unwrap(),
        TupleVariantDefault::Value(1, 0)
    );
}

#[test]
fn camel_case_splits_acronyms() {
    let text = yaml::to_string(&Acronym {
        HTTPPort: 1,
    })
    .unwrap();
    assert!(text.contains("httpPort"), "{text}");
}

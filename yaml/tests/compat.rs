#![cfg(feature = "derive")]

use std::{collections::HashMap, io::Cursor};

use yaml::{
    Decode, DuplicateKeys, Encode, Options, Value, from_slice, from_str, from_str_multi, from_str_multi_with_options,
    from_str_with_options,
};

fn value(input: &str) -> Value {
    from_str(input).unwrap()
}

#[test]
fn scalar_resolution() {
    assert_eq!(value("null"), Value::Null);
    assert_eq!(value("~"), Value::Null);
    assert_eq!(value(""), Value::Null);
    assert_eq!(value("true"), Value::Bool(true));
    assert_eq!(value("FALSE"), Value::Bool(false));
    assert_eq!(value("0"), Value::from(0));
    assert_eq!(value("-17"), Value::from(-17));
    assert_eq!(value("0x10"), Value::from(16));
    assert_eq!(value("0o17"), Value::from(15));
    assert_eq!(value("0b101"), Value::from(5));
    assert_eq!(value("1.5"), Value::from(1.5));
    assert_eq!(value("1e3"), Value::from(1000.0));
    assert!(value(".inf").as_f64().unwrap().is_infinite());
    assert!(value("-.inf").as_f64().unwrap().is_infinite());
    assert!(value(".nan").as_f64().unwrap().is_nan());
    assert_eq!(value("yes"), Value::from("yes"));
    assert_eq!(value("on"), Value::from("on"));
}

#[test]
fn quoted_and_block_scalars() {
    assert_eq!(value("'true'"), Value::from("true"));
    assert_eq!(value("\"42\""), Value::from("42"));
    assert_eq!(value("'it''s'"), Value::from("it's"));
    assert_eq!(value("|\n  line one\n  line two\n"), Value::from("line one\nline two\n"));
    assert_eq!(value(">\n  line one\n  line two\n"), Value::from("line one line two\n"));
    assert_eq!(value("\"a\\tb\""), Value::from("a\tb"));
    assert_eq!(value("\"a\\u263Ab\""), Value::from("a☺b"));
}

#[test]
fn sequences_and_mappings() {
    let root = value(
        r#"
# comment
list:
  - one
  - two
  - nested:
      - 1
      - 2
map:
  a: 1
  b: { c: 2, d: [3, 4] }
empty_list: []
empty_map: {}
"#,
    );
    assert_eq!(root["list"][0], Value::from("one"));
    assert_eq!(root["list"][2]["nested"][1], Value::from(2));
    assert_eq!(root["map"]["a"], Value::from(1));
    assert_eq!(root["map"]["b"]["c"], Value::from(2));
    assert_eq!(root["map"]["b"]["d"][0], Value::from(3));
    assert!(root["empty_list"].as_sequence().unwrap().is_empty());
    assert!(root["empty_map"].as_mapping().unwrap().is_empty());
}

#[test]
fn complex_keys_and_non_string_keys() {
    let root = value(
        r#"
? [a, b]
: pair
1: integer
true: boolean
"#,
    );
    assert_eq!(root[1usize], Value::from("integer"));
}

#[test]
fn anchors_aliases_and_merge_keys() {
    let root = value(
        r#"
defaults: &defaults
  retries: 3
  timeout: 30
service:
  <<: *defaults
  timeout: 60
copy: *defaults
"#,
    );
    assert_eq!(root["service"]["retries"], Value::from(3));
    assert_eq!(root["service"]["timeout"], Value::from(60));
    assert_eq!(root["copy"]["timeout"], Value::from(30));
}

#[test]
fn tags() {
    let root = value("!point {x: 1, y: 2}");
    let Value::Tagged(tagged) = &root else {
        panic!("expected tagged value");
    };
    assert_eq!(tagged.tag, "!point");
    assert_eq!(tagged.value["x"], Value::from(1));
}

#[test]
fn multiple_documents() {
    let docs: Vec<Value> = from_str_multi("---\na: 1\n---\nb: 2\n...").unwrap();
    assert_eq!(docs.len(), 2);
    assert_eq!(docs[0]["a"], Value::from(1));
    assert_eq!(docs[1]["b"], Value::from(2));
}

#[test]
fn duplicate_keys_policy() {
    let input = "a: 1\na: 2\n";
    assert!(from_str::<Value>(input).is_err());

    let first = Options {
        duplicate_keys: DuplicateKeys::First,
        ..Options::default()
    };
    assert_eq!(from_str_with_options::<Value>(input, first).unwrap()["a"], Value::from(1));

    let last = Options {
        duplicate_keys: DuplicateKeys::Last,
        ..Options::default()
    };
    assert_eq!(from_str_with_options::<Value>(input, last).unwrap()["a"], Value::from(2));
}

#[test]
fn resource_limits() {
    let depth = Options {
        max_depth: 2,
        ..Options::default()
    };
    assert!(from_str_with_options::<Value>("a: {b: {c: {d: 1}}}", depth).is_err());

    let aliases = Options {
        max_aliases: Some(1),
        ..Options::default()
    };
    assert!(from_str_with_options::<Value>("a: &a 1\nb: [*a, *a]\n", aliases).is_err());

    let values = Options {
        max_values: Some(3),
        ..Options::default()
    };
    assert!(from_str_with_options::<Value>("[1, 2, 3, 4]", values).is_err());

    let documents = Options {
        max_documents: Some(1),
        ..Options::default()
    };
    assert!(from_str_multi_with_options::<Value>("---\n1\n---\n2\n", documents).is_err());
}

#[test]
fn malformed_input_reports_position() {
    let error = from_str::<Value>("a: [1, 2").unwrap_err();
    assert!(error.line().is_some());
    assert!(!error.message().is_empty());
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Server {
    name: String,
    #[yaml(default)]
    workers: u16,
    #[yaml(rename = "max-connections")]
    max_connections: u32,
    #[yaml(skip)]
    internal: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
#[yaml(rename_all = "camelCase")]
enum Mode {
    Simple,
    Count(u32),
    Pair(u32, u32),
    Config { retries: u32 },
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Root {
    server: Server,
    modes: Vec<Mode>,
    labels: HashMap<String, String>,
}

#[test]
fn derive_roundtrip_and_defaults() {
    let root = Root {
        server: Server {
            name: "api".to_string(),
            workers: 0,
            max_connections: 100,
            internal: false,
        },
        modes: vec![
            Mode::Simple,
            Mode::Count(3),
            Mode::Pair(1, 2),
            Mode::Config {
                retries: 5,
            },
        ],
        labels: HashMap::from([("env".to_string(), "prod".to_string())]),
    };
    let text = yaml::to_string(&root).unwrap();
    let decoded: Root = from_str(&text).unwrap();
    assert_eq!(decoded, root);

    let decoded: Server = from_str("name: api\nmax-connections: 7\n").unwrap();
    assert_eq!(
        decoded,
        Server {
            name: "api".to_string(),
            workers: 0,
            max_connections: 7,
            internal: false,
        }
    );
}

#[test]
fn emission_preserves_ambiguous_and_multiline_strings() {
    let strings = vec![
        "true".to_string(),
        "yes".to_string(),
        "on".to_string(),
        "42".to_string(),
        "1.5".to_string(),
        "null".to_string(),
        "".to_string(),
        " leading".to_string(),
        "trailing ".to_string(),
        "multi\nline\ntext".to_string(),
        "tab\there".to_string(),
        "quote's".to_string(),
        "colon: value".to_string(),
    ];
    let text = yaml::to_string(&strings).unwrap();
    let decoded: Vec<String> = from_str(&text).unwrap();
    assert_eq!(decoded, strings, "{text}");
}

#[test]
fn emission_handles_control_characters() {
    let strings = vec!["nul\0byte".to_string(), "bell\u{7}".to_string()];
    let text = yaml::to_string(&strings).unwrap();
    let decoded: Vec<String> = from_str(&text).unwrap();
    assert_eq!(decoded, strings);
}

#[test]
fn reader_and_slice_helpers() {
    let mut reader = Cursor::new(b"a: 1\n".to_vec());
    let value: Value = yaml::from_reader(&mut reader).unwrap();
    assert_eq!(value["a"], Value::from(1));

    let value: Value = from_slice(b"b: 2\n").unwrap();
    assert_eq!(value["b"], Value::from(2));
}

#[test]
fn mapping_and_value_indexing() {
    let root = value("a: 1\nb: 2\n");
    assert_eq!(root["a"], Value::from(1));
    let mapping = root.as_mapping().unwrap();
    assert_eq!(mapping["b"], Value::from(2));
    assert!(root.is_mapping());
    assert!(root["a"].is_number());
}

#[test]
fn broad_string_roundtrip() {
    let cases = [
        "simple",
        "with space",
        " leading",
        "trailing ",
        "both ",
        "\ttab",
        "\nleading newline",
        "trailing newline\n",
        "internal\nnewline",
        "carriage\rreturn",
        "windows\r\nnewline",
        "nul\0byte",
        "bell\u{7}",
        "unicode: héllo ☺",
        "'single quotes'",
        "\"double quotes\"",
        "a: b",
        "# comment",
        "- list",
        "? key",
        "{ flow }",
        "[ flow ]",
        "& anchor",
        "* alias",
        "! tag",
        "| block",
        "> folded",
        "---",
        "...",
        "%directive",
        "null",
        "NULL",
        "~",
        "true",
        "false",
        "yes",
        "no",
        "on",
        "off",
        "0x10",
        "0o17",
        "0b101",
        "1e3",
        ".inf",
        ".nan",
        "-1",
        "+1",
        "0",
        "",
    ];
    for case in cases {
        let text = yaml::to_string(&case).unwrap();
        let decoded: String = from_str(&text).unwrap();
        assert_eq!(decoded, case, "text: {text:?}");
    }
}

#[test]
fn nested_value_roundtrip() {
    let root: Value = from_str(
        r#"
map:
  ? [a, b]
  : pair
  1: integer
  true: boolean
list:
  - null
  - true
  - 3
  - 1.5
  - "text"
  - { nested: [1, 2, 3] }
"#,
    )
    .unwrap();
    let text = yaml::to_string(&root).unwrap();
    let decoded: Value = from_str(&text).unwrap();
    assert_eq!(decoded, root);
}

#[test]
fn empty_documents_and_comments() {
    assert_eq!(from_str::<Value>("").unwrap(), Value::Null);
    assert_eq!(from_str::<Value>("# only a comment\n").unwrap(), Value::Null);
    let root = value("a:\nb: null\n");
    assert_eq!(root["a"], Value::Null);
    assert_eq!(root["b"], Value::Null);
}

#[test]
fn explicit_tags_override_plain_resolution() {
    assert_eq!(value("!!str 42"), Value::from("42"));
    assert_eq!(value("!!int '42'"), Value::from(42));
    assert_eq!(value("!!bool 'true'"), Value::Bool(true));
    assert_eq!(value("!!null 'anything'"), Value::Null);
    assert_eq!(value("!!float '1.5'"), Value::from(1.5));
}

#[test]
fn byte_order_mark_is_accepted() {
    assert_eq!(from_str::<Value>("\u{feff}a: 1\n").unwrap()["a"], Value::from(1));
}

#[test]
fn alias_expansion_budget_is_enforced() {
    let mut input = String::from("base: &base [1, 2, 3, 4]\nrefs:\n");
    for _ in 0..10_000 {
        input.push_str("  - *base\n");
    }
    let options = Options {
        max_values: Some(100),
        ..Options::default()
    };
    assert!(from_str_with_options::<Value>(&input, options).is_err());
}

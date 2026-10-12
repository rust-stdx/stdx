//! Parser robustness: malformed and adversarial input must never panic, and a
//! deterministic mini-fuzzer explores random template/expression sources.

use template::{Engine, EscapeMode, context};

fn try_parse(source: &str) -> bool {
    let mut engine = Engine::new(EscapeMode::Html);
    match engine.add_template("t", source) {
        Ok(()) => {
            // Reaching the renderer must not panic either. Errors are fine.
            let _ = engine.render("t", context! {});
            true
        }
        Err(_) => false,
    }
}

#[test]
fn truncated_inputs_never_panic() {
    let inputs = [
        "",
        "{",
        "{{",
        "{{ ",
        "{{ x",
        "{{ x }",
        "{{ x }}",
        "{{{",
        "{{{{",
        "%",
        "{%",
        "{% ",
        "{% if",
        "{% if x",
        "{% if x %}",
        "{% if x %}{% endif",
        "{% endfor %}",
        "{% else %}",
        "{#",
        "{# unterminated",
        "{# ok #}",
        "{% raw %}",
        "{% raw %}{% endraw %}",
        "{% raw %}{% endraw",
        "{{ \"unterminated",
        "{{ 'unterminated",
        "{{ \"a}}b\" }}",
        "{% if x == \"%}\" %}y{% endif %}",
        "é",
        "ééé{% if true %}ok{% endif %}",
        "\0",
        "a\0b",
        "{{ }}",
        "{{ | }}",
        "{{ a | }}",
        "{{ a | nope }}",
        "{{ a[ }}",
        "{{ a[] }}",
        "{{ a[1 }}",
        "{{ a b }}",
        "{{ 1 + }}",
        "{% for %}",
        "{% for x %}",
        "{% for x in %}",
        "{% for x in y %}",
        "{% for x in y %}{% endfor %}",
        "{% block %}{% endblock %}",
        "{% extends %}",
        "{% include %}",
        "{% set %}",
        "{% set x %}",
        "{% set x = %}",
        "{% unknown %}",
    ];
    for input in inputs {
        // Must return (Ok or Err) without panicking.
        let _ = try_parse(input);
    }
}

#[test]
fn valid_delimiters_inside_strings() {
    let mut engine = Engine::new(EscapeMode::Text);
    engine.add_template("t", "{{ \"a}}b\" }}").unwrap();
    assert_eq!(engine.render("t", context! {}).unwrap(), "a}}b");

    engine
        .add_template("u", "{% if x == \"%}\" %}Y{% else %}N{% endif %}")
        .unwrap();
    assert_eq!(engine.render("u", context! { x: "%}" }).unwrap(), "Y");
}

#[test]
fn nesting_limits_error_without_panic() {
    let mut engine = Engine::new(EscapeMode::Text);

    let deep_tags = "{% if true %}".repeat(1000) + &"{% endif %}".repeat(1000);
    assert!(engine.add_template("tags", &deep_tags).is_err());

    let deep_parens = "{{ ".to_string() + &"(".repeat(5000) + "1" + &")".repeat(5000) + " }}";
    assert!(engine.add_template("parens", &deep_parens).is_err());

    let long_filters = format!("{{{{ x{} }}}}", "|upper".repeat(50_000));
    assert!(engine.add_template("filters", &long_filters).is_err());
}

/// Deterministic byte-level mini-fuzzer.
///
/// Generates pseudo-random sources from a template-heavy alphabet and asserts
/// that parsing and (when parsing succeeds) rendering never panic. No external
/// dependencies, so it runs on every `cargo test`.
#[test]
fn fuzz_parser_never_panics() {
    // Simple xorshift64 PRNG for reproducibility.
    let mut state: u64 = 0x9E3779B97F4A7C15;
    let mut next = move || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        state
    };

    const ALPHABET: &[u8] = b"{}%#'\"\\|(),.[]=<>!+-*/:; \tabcxyz019_ntruefalslr";

    for _ in 0..30_000 {
        let len = (next() % 48) as usize;
        let mut source = String::with_capacity(len);
        for _ in 0..len {
            let idx = (next() as usize) % ALPHABET.len();
            source.push(ALPHABET[idx] as char);
        }
        // Must not panic.
        let _ = try_parse(&source);
    }
}

/// The same alphabet, biased towards well-formed fragments, to exercise deeper
/// code paths (`if`/`for`/raw/comment/filters).
#[test]
fn fuzz_fragments_never_panic() {
    const FRAGMENTS: &[&str] = &[
        "{{ x }}",
        "{{ x.y.z }}",
        "{{ x[0] }}",
        "{{ x | upper }}",
        "{{ x | default(1) }}",
        "{% if x %}",
        "{% elif y %}",
        "{% else %}",
        "{% endif %}",
        "{% for i in x %}",
        "{% endfor %}",
        "{% set a = 1 %}",
        "{% raw %}",
        "{% endraw %}",
        "{% block b %}",
        "{% endblock %}",
        "{% include \"t\" %}",
        "{# c #}",
        "text",
        "{{ 'a}}b' }}",
        "{% if x == \"%}\" %}",
        "{{ not x }}",
        "{{ is_set(a.b.c) }}",
        "{{ range(3) }}",
    ];

    let mut state: u64 = 0x1234_5678_9ABC_DEF0;
    let mut next = move || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        state
    };

    for _ in 0..20_000 {
        let count = (next() % 12) as usize;
        let mut source = String::new();
        for _ in 0..count {
            source.push_str(FRAGMENTS[(next() as usize) % FRAGMENTS.len()]);
        }
        let _ = try_parse(&source);
    }
}

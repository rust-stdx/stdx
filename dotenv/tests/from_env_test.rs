use std::sync::Mutex;

use dotenv::{FromEnv, FromEnvError, FromEnvValue};

// Tests mutate the process-wide environment, so they must not run
// concurrently with each other.
static LOCK: Mutex<()> = Mutex::new(());

fn set(k: &str, v: &str) {
    unsafe { std::env::set_var(k, v) };
}

fn remove(k: &str) {
    unsafe { std::env::remove_var(k) };
}

#[derive(Debug, FromEnv)]
struct SimpleConfig {
    #[env(rename = "TEST_FE_SIMPLE_HOST")]
    host: String,
    #[env(rename = "TEST_FE_SIMPLE_PORT")]
    port: u16,
}

#[test]
fn from_env_simple() {
    let _g = LOCK.lock().unwrap();
    set("TEST_FE_SIMPLE_HOST", "localhost");
    set("TEST_FE_SIMPLE_PORT", "8080");

    let cfg = SimpleConfig::from_env().unwrap();
    assert_eq!(cfg.host, "localhost");
    assert_eq!(cfg.port, 8080);

    remove("TEST_FE_SIMPLE_HOST");
    remove("TEST_FE_SIMPLE_PORT");
}

#[test]
fn from_env_missing_var() {
    let _g = LOCK.lock().unwrap();
    remove("TEST_FE_SIMPLE_HOST");
    remove("TEST_FE_SIMPLE_PORT");

    let err = SimpleConfig::from_env().unwrap_err();
    match &err {
        FromEnvError::Missing(var) => assert_eq!(var, "TEST_FE_SIMPLE_HOST"),
        _ => panic!("expected Missing, got {err:?}"),
    }
}

#[test]
fn from_env_invalid_value() {
    let _g = LOCK.lock().unwrap();
    set("TEST_FE_SIMPLE_HOST", "localhost");
    set("TEST_FE_SIMPLE_PORT", "not_a_number");

    let err = SimpleConfig::from_env().unwrap_err();
    match &err {
        FromEnvError::Invalid {
            var, ..
        } => assert_eq!(var, "TEST_FE_SIMPLE_PORT"),
        _ => panic!("expected Invalid, got {err:?}"),
    }

    remove("TEST_FE_SIMPLE_HOST");
    remove("TEST_FE_SIMPLE_PORT");
}

// ── default / default = expr ─────────────────────────────────────────────

#[derive(Debug, FromEnv)]
struct WithDefault {
    #[env(rename = "TEST_WD_HOST")]
    host: String,
    #[env(rename = "TEST_WD_PORT", default)]
    port: Option<u16>,
}

#[test]
fn from_env_with_default() {
    let _g = LOCK.lock().unwrap();
    set("TEST_WD_HOST", "example.com");
    remove("TEST_WD_PORT");

    let cfg = WithDefault::from_env().unwrap();
    assert_eq!(cfg.host, "example.com");
    assert_eq!(cfg.port, None);

    // The variable is read first; the default only applies when it is unset.
    set("TEST_WD_PORT", "1234");
    let cfg = WithDefault::from_env().unwrap();
    assert_eq!(cfg.port, Some(1234));

    // An empty value yields `None` for `Option`.
    set("TEST_WD_PORT", "");
    let cfg = WithDefault::from_env().unwrap();
    assert_eq!(cfg.port, None);

    remove("TEST_WD_PORT");
    remove("TEST_WD_HOST");
}

#[derive(Debug, FromEnv)]
struct WithDefaultExpr {
    #[env(rename = "TEST_WDE_HOST")]
    host: String,
    #[env(rename = "TEST_WDE_PORT", default = 3000)]
    port: u16,
}

#[test]
fn from_env_with_default_expr() {
    let _g = LOCK.lock().unwrap();
    set("TEST_WDE_HOST", "example.com");
    remove("TEST_WDE_PORT");

    let cfg = WithDefaultExpr::from_env().unwrap();
    assert_eq!(cfg.host, "example.com");
    assert_eq!(cfg.port, 3000);

    set("TEST_WDE_PORT", "9090");
    let cfg = WithDefaultExpr::from_env().unwrap();
    assert_eq!(cfg.port, 9090);

    remove("TEST_WDE_PORT");
    remove("TEST_WDE_HOST");
}

// ── default without rename: reads the field-derived variable first ───────

#[derive(Debug, FromEnv)]
struct DefaultReadFirst {
    #[env(default)]
    dntest_default_port: u16,
}

#[derive(Debug, FromEnv)]
struct DefaultReadFirstOpt {
    #[env(default)]
    dntest_default_opt: Option<u16>,
}

#[test]
fn from_env_default_reads_variable_first() {
    let _g = LOCK.lock().unwrap();

    remove("DNTEST_DEFAULT_PORT");
    assert_eq!(DefaultReadFirst::from_env().unwrap().dntest_default_port, 0);

    set("DNTEST_DEFAULT_PORT", "4321");
    assert_eq!(DefaultReadFirst::from_env().unwrap().dntest_default_port, 4321);

    remove("DNTEST_DEFAULT_PORT");
}

#[test]
fn from_env_default_option_reads_variable_first() {
    let _g = LOCK.lock().unwrap();

    remove("DNTEST_DEFAULT_OPT");
    assert_eq!(DefaultReadFirstOpt::from_env().unwrap().dntest_default_opt, None);

    set("DNTEST_DEFAULT_OPT", "7");
    assert_eq!(DefaultReadFirstOpt::from_env().unwrap().dntest_default_opt, Some(7));

    set("DNTEST_DEFAULT_OPT", "");
    assert_eq!(DefaultReadFirstOpt::from_env().unwrap().dntest_default_opt, None);

    remove("DNTEST_DEFAULT_OPT");
}

// ── rename + default / rename + default = expr ───────────────────────────

#[derive(Debug, FromEnv)]
struct RenameDefaultStandard {
    #[env(rename = "TEST_RDS_PORT", default)]
    port: u16,
    #[env(rename = "TEST_RDS_HOST")]
    host: String,
}

#[test]
fn from_env_rename_default_standard() {
    let _g = LOCK.lock().unwrap();
    set("TEST_RDS_HOST", "example.com");
    remove("TEST_RDS_PORT");

    let cfg = RenameDefaultStandard::from_env().unwrap();
    assert_eq!(cfg.host, "example.com");
    assert_eq!(cfg.port, 0);

    set("TEST_RDS_PORT", "8080");
    let cfg = RenameDefaultStandard::from_env().unwrap();
    assert_eq!(cfg.port, 8080);

    remove("TEST_RDS_PORT");
    remove("TEST_RDS_HOST");
}

// ── all defaults (no env vars needed) ────────────────────────────────────

#[derive(Debug, FromEnv)]
struct AllDefaults {
    #[env(rename = "TEST_AD_FLAG", default)]
    flag: Option<bool>,
    #[env(rename = "TEST_AD_ANSWER", default = 42)]
    answer: u32,
    #[env(rename = "TEST_AD_NAME", default)]
    name: String,
}

#[test]
fn from_env_all_defaults() {
    let _g = LOCK.lock().unwrap();
    remove("TEST_AD_FLAG");
    remove("TEST_AD_ANSWER");
    remove("TEST_AD_NAME");

    let cfg = AllDefaults::from_env().unwrap();
    assert_eq!(cfg.flag, None);
    assert_eq!(cfg.answer, 42);
    assert_eq!(cfg.name, "");
}

// ── bare Option<T> fields are optional ───────────────────────────────────

#[derive(Debug, FromEnv)]
struct OptionalField {
    #[env(rename = "TEST_OPT_PORT")]
    port: Option<u16>,
    #[env(rename = "TEST_OPT_NAME", default)]
    name: Option<String>,
}

#[test]
fn from_env_optional_field() {
    let _g = LOCK.lock().unwrap();
    remove("TEST_OPT_PORT");
    remove("TEST_OPT_NAME");

    let cfg = OptionalField::from_env().unwrap();
    assert_eq!(cfg.port, None);
    assert_eq!(cfg.name, None);

    set("TEST_OPT_PORT", "44");
    set("TEST_OPT_NAME", "hi");
    let cfg = OptionalField::from_env().unwrap();
    assert_eq!(cfg.port, Some(44));
    assert_eq!(cfg.name.as_deref(), Some("hi"));

    set("TEST_OPT_PORT", "");
    let cfg = OptionalField::from_env().unwrap();
    assert_eq!(cfg.port, None);

    remove("TEST_OPT_PORT");
    remove("TEST_OPT_NAME");
}

// ── empty struct ─────────────────────────────────────────────────────────

#[derive(Debug, FromEnv)]
struct EmptyStruct {}

#[test]
fn from_env_empty_struct() {
    let cfg = EmptyStruct::from_env().unwrap();
    let _ = cfg;
}

// ── with (custom parser), including a module path ────────────────────────

mod parsers {
    use dotenv::FromEnvError;

    pub fn parse_u16_from_env(var: &str, val: &str) -> Result<u16, FromEnvError> {
        val.parse::<u16>()
            .map_err(|e| FromEnvError::invalid(var, val, e.to_string()))
    }
}

#[derive(Debug, FromEnv)]
struct WithCustomParser {
    #[env(rename = "TEST_WCP_NUM", with = "parsers::parse_u16_from_env")]
    num: u16,
    #[env(rename = "TEST_WCP_OTHER")]
    other: String,
}

#[test]
fn from_env_with_custom_parser() {
    let _g = LOCK.lock().unwrap();
    set("TEST_WCP_NUM", "42");
    set("TEST_WCP_OTHER", "hello");

    let cfg = WithCustomParser::from_env().unwrap();
    assert_eq!(cfg.num, 42);
    assert_eq!(cfg.other, "hello");

    remove("TEST_WCP_NUM");
    remove("TEST_WCP_OTHER");
}

// ── nested struct ────────────────────────────────────────────────────────

#[derive(Debug, FromEnv)]
struct Database {
    #[env(rename = "TEST_DB_URL")]
    url: String,
    #[env(default = 5)]
    pool_size: u32,
}

#[derive(Debug, FromEnv)]
struct NestedConfig {
    database: Database,
    #[env(rename = "TEST_APP_DEBUG")]
    debug: bool,
}

#[test]
fn from_env_nested() {
    let _g = LOCK.lock().unwrap();
    set("TEST_DB_URL", "postgres://localhost/mydb");
    set("TEST_APP_DEBUG", "true");

    let cfg = NestedConfig::from_env().unwrap();
    assert_eq!(cfg.database.url, "postgres://localhost/mydb");
    assert_eq!(cfg.database.pool_size, 5);
    assert!(cfg.debug);

    remove("TEST_DB_URL");
    remove("TEST_APP_DEBUG");
}

// ── deeply nested with prefix propagation ────────────────────────────────

#[derive(Debug, FromEnv)]
struct DeepChild {
    key: String,
}

#[derive(Debug, FromEnv)]
struct Middle {
    child: DeepChild,
}

#[derive(Debug, FromEnv)]
struct Root {
    middle: Middle,
}

#[test]
fn from_env_deeply_nested() {
    let _g = LOCK.lock().unwrap();
    set("MIDDLE_CHILD_KEY", "deep-value");

    let cfg = Root::from_env().unwrap();
    assert_eq!(cfg.middle.child.key, "deep-value");

    remove("MIDDLE_CHILD_KEY");
}

// ── with (custom parser) without rename → uses field-based prefix ────────

fn parse_debug_flag(var: &str, val: &str) -> Result<bool, FromEnvError> {
    match val {
        "1" | "yes" | "true" => Ok(true),
        "0" | "no" | "false" => Ok(false),
        _ => Err(FromEnvError::invalid(var, val, format!("unexpected value: {val}"))),
    }
}

#[derive(Debug, FromEnv)]
struct NestedWithPrefix {
    #[env(with = "parse_debug_flag")]
    debug_flag: bool,
}

#[derive(Debug, FromEnv)]
struct ParentOfWith {
    #[env(rename = "TEST_PARENT_DEBUG")]
    debug: bool,
    nested: NestedWithPrefix,
}

#[test]
fn from_env_with_without_rename_uses_prefix() {
    let _g = LOCK.lock().unwrap();
    set("TEST_PARENT_DEBUG", "true");
    set("NESTED_DEBUG_FLAG", "yes");

    let cfg = ParentOfWith::from_env().unwrap();
    assert!(cfg.debug);
    assert!(cfg.nested.debug_flag);

    remove("TEST_PARENT_DEBUG");
    remove("NESTED_DEBUG_FLAG");
}

// ── custom parser with missing env var ──────────────────────────────────

#[derive(Debug, FromEnv)]
struct WithCustomParserMissing {
    #[env(rename = "TEST_WCP_MISSING", with = "parsers::parse_u16_from_env")]
    _num: u16,
}

#[test]
fn from_env_with_custom_parser_missing_var() {
    let _g = LOCK.lock().unwrap();
    remove("TEST_WCP_MISSING");

    let err = WithCustomParserMissing::from_env().unwrap_err();
    match &err {
        FromEnvError::Missing(var) => assert_eq!(var, "TEST_WCP_MISSING"),
        _ => panic!("expected Missing, got {err:?}"),
    }
}

// ── FromEnvValue trait ───────────────────────────────────────────────────

#[test]
fn from_env_value_trait() {
    let s = <String as FromEnvValue>::from_env_value("hello").unwrap();
    assert_eq!(s, "hello");

    let n = <u16 as FromEnvValue>::from_env_value("42").unwrap();
    assert_eq!(n, 42);

    let b = <bool as FromEnvValue>::from_env_value("true").unwrap();
    assert!(b);

    let err = <u16 as FromEnvValue>::from_env_value("not_a_number").unwrap_err();
    assert!(!err.is_empty());
}

// ── FromEnvError Display ─────────────────────────────────────────────────

#[test]
fn from_env_error_display() {
    let e = FromEnvError::missing("MY_VAR");
    assert_eq!(e.to_string(), "environment variable `MY_VAR` is not set");

    let e = FromEnvError::invalid("PORT", "abc", "invalid digit");
    assert_eq!(
        e.to_string(),
        "environment variable `PORT` has invalid value `abc`: invalid digit"
    );
}

// ── dotenv::from_env free function ───────────────────────────────────────

#[test]
fn from_env_free_function() {
    let _g = LOCK.lock().unwrap();
    set("TEST_FE_SIMPLE_HOST", "free-fn");
    set("TEST_FE_SIMPLE_PORT", "9090");

    let cfg: SimpleConfig = dotenv::from_env().unwrap();
    assert_eq!(cfg.host, "free-fn");
    assert_eq!(cfg.port, 9090);

    remove("TEST_FE_SIMPLE_HOST");
    remove("TEST_FE_SIMPLE_PORT");
}

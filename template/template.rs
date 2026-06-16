#![no_std]

//! A fast, safe template engine for HTML and text rendering, inspired by Jinja2
//! with `no_std`support.
//!
//! `template` provides a runtime template engine that compiles templates into
//! an AST and renders them via a tree-walking interpreter. It supports both
//! HTML mode (with automatic escaping) and text mode (no escaping).
//!
//! # Quick start
//!
//! ```rust
//! use template::{Engine, EscapeMode, args};
//!
//! let mut engine = Engine::new(EscapeMode::Html);
//! engine.add_template("hello", "<p>Hello, {{ name }}!</p>");
//!
//! let result = engine.render("hello", args! { name: "World" });
//! assert_eq!(result.unwrap(), "<p>Hello, World!</p>");
//! ```
//!
//! The [`args!`] macro builds a context map.
//! You can also pass any `#[derive(Serialize)]` struct, or `serde_json::Value`.
//! Quoted keys (e.g. `"my-key"`) are also accepted for programmatic context
//! construction from external data, though template variables must currently be
//! valid Rust identifiers.
//!
//! # Features
//!
//! | Feature    | Description                                                                 |
//! |------------|-----------------------------------------------------------------------------|
//! | `default`  | Enables the `std` feature.                                                  |
//! | `std`      | Enables `std`-dependent features such as `std::error::Error` on error types and full `serde`/`memchr` `std` integration. |
//!
//! # Working with slices and vectors
//!
//! Iterate over a list with `{% for %}`:
//!
//! ```rust
//! use template::{Engine, EscapeMode, args};
//!
//! let mut engine = Engine::new(EscapeMode::Text);
//! engine.add_template("list", "{% for item in items %}- {{ item }}
//! {% endfor %}").unwrap();
//!
//! let result = engine.render("list", args! {
//!     items: vec!["apple", "banana", "cherry"],
//! }).unwrap();
//! assert_eq!(result, "- apple\n- banana\n- cherry\n");
//! ```
//!
//! Access elements by index, including nested fields:
//!
//! ```rust
//! use template::{Engine, EscapeMode, args};
//!
//! let mut engine = Engine::new(EscapeMode::Text);
//! engine.add_template("t", "{{ users[0].name }}, {{ users[1].name }}").unwrap();
//!
//! let result = engine.render("t", args! {
//!     users: vec![
//!         args! { name: "Alice", age: 30 },
//!         args! { name: "Bob", age: 25 },
//!     ],
//! }).unwrap();
//! assert_eq!(result, "Alice, Bob");
//! ```
//!
//! Use filters on arrays — `join`, `first`, `last`, `length`, `reverse`:
//!
//! ```rust
//! use template::{Engine, EscapeMode, args};
//!
//! let mut engine = Engine::new(EscapeMode::Text);
//! engine.add_template("t", "\
//! join: {{ items | join(\", \") }}
//! first: {{ items | first }}
//! last:  {{ items | last }}
//! count: {{ items | length }}
//! rev:   {{ items | reverse | join(\", \") }}
//! ").unwrap();
//!
//! let result = engine.render("t", args! {
//!     items: vec!["a", "b", "c"],
//! }).unwrap();
//! assert_eq!(result, "\
//! join: a, b, c
//! first: a
//! last:  c
//! count: 3
//! rev:   c, b, a
//! ");
//! ```
//!
//! Check membership with the `in` operator:
//!
//! ```rust
//! use template::{Engine, EscapeMode, args};
//!
//! let mut engine = Engine::new(EscapeMode::Text);
//! engine.add_template("t",
//!     "{% if \"admin\" in roles %}Welcome, admin!{% endif %}"
//! ).unwrap();
//!
//! let result = engine.render("t", args! {
//!     roles: vec!["user", "admin", "moderator"],
//! }).unwrap();
//! assert_eq!(result, "Welcome, admin!");
//! ```
//!
//! Render directly from a Rust `Vec`:
//!
//! ```rust
//! use template::{Engine, EscapeMode, args};
//!
//! let mut engine = Engine::new(EscapeMode::Text);
//! engine.add_template("t", "{% for n in numbers %}{{ n }} {% endfor %}").unwrap();
//!
//! let result = engine.render("t", args! {
//!     numbers: vec![10, 20, 30],
//! }).unwrap();
//! assert_eq!(result, "10 20 30 ");
//! ```
//!
//! Filter across a nested array field:
//!
//! ```rust
//! use template::{Engine, EscapeMode, args};
//!
//! let mut engine = Engine::new(EscapeMode::Text);
//! engine.add_template("t",
//!     "{% for tag in post.tags %}{{ tag | upper }} {% endfor %}"
//! ).unwrap();
//!
//! let result = engine.render("t", args! {
//!     post: args! {
//!         title: "Hello",
//!         tags: vec!["rust", "template", "dev"],
//!     },
//! }).unwrap();
//! assert_eq!(result, "RUST TEMPLATE DEV ");
//! ```
//!
//! # Modes
//!
//! - `EscapeMode::Html` — auto-escapes `{{ ... }}` output (escapes `&`, `<`, `>`, `"`, `'`)
//! - `EscapeMode::Text` — no escaping, raw output
//!
//! # Template syntax
//!
//! | Syntax | Description |
//! |--------|-------------|
//! | `{{ expr }}` | Output expression value (auto-escaped in HTML mode) |
//! | `{% if cond %}...{% elif %}...{% else %}...{% endif %}` | Conditional |
//! | `{% for item in items %}...{% endfor %}` | Loop |
//! | `{% include "name" %}` | Include another template |
//! | `{% extends "base" %}` | Template inheritance |
//! | `{% block name %}...{% endblock %}` | Overridable block |
//! | `{{ super() }}` | Render parent's block content (only inside `{% block %}`) |
//! | `{% set var = expr %}` | Assign a variable |
//! | `{% raw %}...{% endraw %}` | Raw text (no parsing; can contain `{%` sequences) |
//! | `{# comment #}` | Comment (ignored) |
//! | `expr \| filter_name` | Apply a filter |
//!
//! # Expressions
//!
//! - Variable access: `name`, `user.email`, `items[0]`
//! - String literals: `"hello"`, `'world'`
//! - Number literals: `42`, `3.14` (scientific notation and hex are not supported)
//! - Boolean: `true`, `false`
//! - Comparisons: `==`, `!=`, `<`, `>`, `<=`, `>=` (floats follow IEEE 754; `NaN` is falsy and `NaN` compared to anything is `false`)
//! - Logical: `and`, `or`, `not`
//! - Arithmetic: `+`, `-`, `*`, `/`, `%` (dividing or modulo by zero returns an error)
//! - Containment: `item in list`
//! - Grouping: `(expr)`
//! - Filters: `expr | filter_name`, `expr | filter(arg1, arg2)`
//! - Function calls: `super()`, `range(n)`, `range(start, end)`
//!
//! ## Safety boundaries
//!
//! - **Integer division/modulo by zero** is rejected with a render error (not a panic).
//!   Float division/modulo by zero follows IEEE 754 (returns `inf` / `-inf` / `NaN`).
//! - **Circular includes** (`a -> b -> a`) are detected by a depth limit (64).
//! - **Circular extends** (`a extends b extends a`) are detected by a depth limit (128).
//! - **Unknown functions** (`{{ myfunc() }}`) return a render error.
//!
//! # Built-in filters
//!
//! | Filter | Description |
//! |--------|-------------|
//! | `upper` | Convert to uppercase |
//! | `lower` | Convert to lowercase |
//! | `trim` | Trim leading/trailing whitespace |
//! | `escape` | HTML-escape the value (`Safe` result, no double-escape) |
//! | `safe` | Mark a string as safe (bypasses auto-escaping) |
//! | `length` | Length of string (character count), array, or map |
//! | `default(val)` | Return `val` if the input is falsy (i.e. `false`, `0`, `0.0`, `NaN`, `""`, `[]`, `null`) |
//! | `capitalize` | Uppercase first character, lowercase the rest |
//! | `title` | Title case (capitalize each word) |
//! | `join(sep)` | Join array elements with separator |
//! | `reverse` | Reverse a string (by Unicode scalar value) or array |
//! | `first` | First element of an array or first character of a string |
//! | `last` | Last element of an array or last character of a string |
//! | `urlencode` | URL-encode (form-style, `+` for spaces) |
//!
//! # Error behavior
//!
//! - `add_template` returns an error if a template with the same name already exists.
//! - Unknown filter names (`{{ x | unknown }}`) produce a parse-time error.
//! - Exceeding the include depth (64) or extend depth (128) returns a render error.
//!
//! # Architecture
//!
//! ```text
//!               ┌──────────────┐
//!  add_template │              │
//!  ────────────▶│   PARSER     │
//!   (source)    │  (recursive  │
//!               │   descent    │
//!               │   parser)    │
//!               └──────┬───────┘
//!                      │ AST
//!               ┌──────▼───────┐
//!               │   ENGINE     │
//!               │  (template   │
//!               │   cache)     │
//!               └──────┬───────┘
//!                      │ render(name, ctx)
//!               ┌──────▼───────┐
//!               │  RENDERER    │
//!               │  (tree-walk  │
//!               │   VM with    │
//!               │   extends /  │
//!               │   blocks /   │
//!               │   includes   │
//!               │   resolution)│
//!               └──────┬───────┘
//!                      │ output
//!               ┌──────▼───────┐
//!               │  fmt::Write  │
//!               │  (String,    │
//!               │   Vec<u8>,   │
//!               │   io::Write) │
//!               └──────────────┘
//! ```

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod ast;
pub mod engine;
pub mod error;
mod escapers;
pub mod expr;
mod filters;
mod parser;
pub mod value;
mod vm;

pub use engine::{Engine, EscapeMode};
pub use error::Error;
pub use value::Value;

#[doc(hidden)]
pub mod __macro_support {
    pub use alloc::{collections::BTreeMap, rc::Rc};
    pub use core::convert::Into;
}

/// Build a context map for [`Engine::render`] without requiring `serde_json`.
///
/// Unquoted identifiers (e.g. `name`) are stringified automatically.
/// Quoted string keys (e.g. `"my-key"`) are also accepted for programmatic
/// construction from external data.
///
/// ```rust
/// use template::{Engine, EscapeMode, args};
///
/// let mut engine = Engine::new(EscapeMode::Text);
/// engine.add_template("t", "Hello, {{ name }}!").unwrap();
///
/// let result = engine.render("t", args! { name: "World" }).unwrap();
/// assert_eq!(result, "Hello, World!");
/// ```
///
/// Supports nesting, vectors, and all types that implement [`Into<Value>`]:
///
/// ```rust
/// use template::{Engine, EscapeMode, args};
///
/// let mut engine = Engine::new(EscapeMode::Text);
/// engine.add_template("t", "\
/// {% for item in items %}- {{ item }}
/// {% endfor %}").unwrap();
///
/// let result = engine.render("t", args! {
///     items: vec!["apple", "banana"],
/// }).unwrap();
/// assert_eq!(result, "- apple\n- banana\n");
/// ```
#[macro_export]
macro_rules! args {
    (@key $key:ident) => { stringify!($key) };
    (@key $key:expr) => { $key };

    ($($key:tt : $value:expr),* $(,)?) => {{
        let mut __map = $crate::__macro_support::BTreeMap::new();
        $(
            __map.insert(
                $crate::args!(@key $key).to_string(),
                $crate::__macro_support::Into::<$crate::Value>::into($value),
            );
        )*
        $crate::Value::Map($crate::__macro_support::Rc::new(__map))
    }};
}

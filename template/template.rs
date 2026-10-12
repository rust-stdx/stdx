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
//! use template::{Engine, EscapeMode, context};
//!
//! let mut engine = Engine::new(EscapeMode::Html);
//! engine.add_template("hello", "<p>Hello, {{ name }}!</p>");
//!
//! let result = engine.render("hello", context! { name: "World" });
//! assert_eq!(result.unwrap(), "<p>Hello, World!</p>");
//! ```
//!
//! The [`context!`] macro builds a context map.
//! You can also pass any `#[derive(Serialize)]` struct (requires the `std`
//! feature), using `&` to pass a reference: `engine.render("t", &my_struct)`.
//!
//! # Trust model
//!
//! Templates are **trusted code**, exactly like Jinja2 or MiniJinja. A template
//! can read every variable in the rendering context and can consume CPU and
//! memory within the configured limits (nesting depth, `range()` cap). Never
//! compile untrusted *template source* with [`Engine::add_template`]. Untrusted
//! *data* belongs in the context, where it is escaped automatically in
//! [`EscapeMode::Html`].
//!
//! # Features
//!
//! | Feature    | Description                                                                 |
//! |------------|-----------------------------------------------------------------------------|
//! | `default`  | Enables the `std` feature.                                                  |
//! | `std`      | Enables `std`-dependent features such as `serde` integration and the ability to pass `#[derive(Serialize)]` structs directly to [`Engine::render`]. |
//!
//! # Working with slices and vectors
//!
//! Iterate over a list with `{% for %}`:
//!
//! ```rust
//! use template::{Engine, EscapeMode, context};
//!
//! let mut engine = Engine::new(EscapeMode::Text);
//! engine.add_template("list", "{% for item in items %}- {{ item }}
//! {% endfor %}").unwrap();
//!
//! let result = engine.render("list", context! {
//!     items: vec!["apple", "banana", "cherry"],
//! }).unwrap();
//! assert_eq!(result, "- apple\n- banana\n- cherry\n");
//! ```
//!
//! Access elements by index, including nested fields:
//!
//! ```rust
//! use template::{Engine, EscapeMode, context};
//!
//! let mut engine = Engine::new(EscapeMode::Text);
//! engine.add_template("t", "{{ users[0].name }}, {{ users[1].name }}").unwrap();
//!
//! let result = engine.render("t", context! {
//!     users: vec![
//!         context! { name: "Alice", age: 30 },
//!         context! { name: "Bob", age: 25 },
//!     ],
//! }).unwrap();
//! assert_eq!(result, "Alice, Bob");
//! ```
//!
//! Use filters on arrays — `join`, `first`, `last`, `length`, `reverse`:
//!
//! ```rust
//! use template::{Engine, EscapeMode, context};
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
//! let result = engine.render("t", context! {
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
//! use template::{Engine, EscapeMode, context};
//!
//! let mut engine = Engine::new(EscapeMode::Text);
//! engine.add_template("t",
//!     "{% if \"admin\" in roles %}Welcome, admin!{% endif %}"
//! ).unwrap();
//!
//! let result = engine.render("t", context! {
//!     roles: vec!["user", "admin", "moderator"],
//! }).unwrap();
//! assert_eq!(result, "Welcome, admin!");
//! ```
//!
//! Render directly from a Rust `Vec`:
//!
//! ```rust
//! use template::{Engine, EscapeMode, context};
//!
//! let mut engine = Engine::new(EscapeMode::Text);
//! engine.add_template("t", "{% for n in numbers %}{{ n }} {% endfor %}").unwrap();
//!
//! let result = engine.render("t", context! {
//!     numbers: vec![10, 20, 30],
//! }).unwrap();
//! assert_eq!(result, "10 20 30 ");
//! ```
//!
//! Filter across a nested array field:
//!
//! ```rust
//! use template::{Engine, EscapeMode, context};
//!
//! let mut engine = Engine::new(EscapeMode::Text);
//! engine.add_template("t",
//!     "{% for tag in post.tags %}{{ tag | upper }} {% endfor %}"
//! ).unwrap();
//!
//! let result = engine.render("t", context! {
//!     post: context! {
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
//! | `{% block name %}...{% endblock %}` | Overridable block (`{% endblock name %}` labels are accepted and validated) |
//! | `{{ super() }}` | Render parent's block content (only inside `{% block %}`) |
//! | `{% set var = expr %}` | Assign a variable |
//! | `{% raw %}...{% endraw %}` | Raw text (no parsing; can contain `{%` sequences) |
//! | `{# comment #}` | Comment (ignored) |
//! | `expr \| filter_name` | Apply a filter |
//!
//! # Expressions
//!
//! - Variable access: `name`, `user.email`, `items[0]`
//!   Identifiers are ASCII only (`[A-Za-z_][A-Za-z0-9_]*`). Non-ASCII identifiers
//!   are rejected **intentionally**, to rule out Unicode confusable/homoglyph and
//!   normalization attacks.
//! - String literals: `"hello"`, `'world'` (may contain `}}`/`%}`; backslash escapes
//!   `\n`, `\t`, `\r`, `\\`, `\'`, `\"` are supported)
//! - Number literals: `42`, `3.14` (scientific notation and hex are not supported)
//! - Boolean: `true`, `false`
//! - Comparisons: `==`, `!=`, `<`, `>`, `<=`, `>=` (floats follow IEEE 754; `NaN`
//!   is never equal and every ordering comparison involving `NaN` is `false`)
//! - Logical: `and`, `or`, `not` (**strict booleans**, see below)
//! - Arithmetic: `+`, `-`, `*`, `/`, `%` (integer division/modulo by zero and
//!   integer overflow return an error; float operations follow IEEE 754)
//! - Containment: `item in list`
//! - Grouping: `(expr)`
//! - Filters: `expr | filter_name`, `expr | filter(arg1, arg2)`
//! - Function calls: `super()`, `range(n)`, `range(start, end)`, `is_set(expr)`
//!
//! # Booleans are strict
//!
//! Logical operators and conditions do **not** use implicit truthiness. The
//! decision operand of `and` / `or` / `not`, and every `{% if %}` / `{% elif %}`
//! condition, must evaluate to a boolean:
//!
//! ```rust
//! # use template::{Engine, EscapeMode, context};
//! let mut engine = Engine::new(EscapeMode::Text);
//! engine.add_template("t",
//!     "{% if is_set(user.bio) and user.verified %}ok{% else %}no{% endif %}"
//! ).unwrap();
//! let result = engine.render("t", context! { user: context! { verified: true } }).unwrap();
//! assert_eq!(result, "no");
//! ```
//!
//! `{{ 0 or "fallback" }}` is a type error (an integer is not a boolean); write
//! `{{ x | default("fallback") }}` or compare explicitly (`{% if count > 0 %}`).
//! `and` / `or` still return their operand values: with a boolean decision, the
//! result is the other operand untouched (`false or "x"` renders `x`).
//!
//! ## `is_set`
//!
//! `is_set(expr)` returns `true` when `expr` evaluates to anything other than
//! `null`/missing, and `false` for missing variables or fields. It never fails:
//! nested access such as `is_set(a.b.c)` is `false` when `b` or `c` is absent.
//!
//! ## Safety boundaries
//!
//! - **Integer division/modulo by zero and integer overflow** are rejected with a
//!   render error (never a panic), in both debug and release builds.
//!   Float division/modulo by zero follows IEEE 754 (returns `inf` / `-inf` / `NaN`).
//! - **Circular includes** (`a -> b -> a`) are limited by a depth of 64.
//! - **Circular extends** (`a extends b extends a`) are limited by a depth of 128.
//! - **Template nesting** (`{% if %}`/`{% for %}`/`{% block %}`) is limited to 128
//!   at parse time, and render/eval nesting to 500, so adversarial templates
//!   return an error instead of overflowing the stack.
//! - **`range()`** is capped (default 32,000 items, see [`Engine::set_range_limit`])
//!   to bound memory use.
//! - **Unknown functions** (`{{ myfunc() }}`) return a render error and **unknown
//!   filters** a parse error.
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
//! | `default(val)` | Return `val` if the input is `null`/missing (does **not** trigger on other falsy values) |
//! | `capitalize` | Uppercase first character, lowercase the rest |
//! | `title` | Title case (capitalize each word) |
//! | `join(sep)` | Join array elements with separator |
//! | `reverse` | Reverse a string (by Unicode scalar value) or array |
//! | `first` | First element of an array or first character of a string |
//! | `last` | Last element of an array or last character of a string |
//! | `urlencode` | URL-encode (form-style, `+` for spaces) |
//!
//! Filters that only transform characters (`upper`, `lower`, `trim`,
//! `capitalize`, `title`, `reverse`, `first`, `last`, `join`) preserve the
//! `Safe` marker of already-safe values, so `{{ x | safe | reverse }}` is not
//! escaped again.
//!
//! # Error behavior
//!
//! - `add_template` returns an error if a template with the same name already
//!   exists, on syntax errors, when nesting limits are exceeded, and on
//!   statically-detectable type errors (e.g. `{{ 0 or "x" }}`).
//! - Unknown filter names (`{{ x | unknown }}`) produce a parse-time error, as do
//!   wrong filter argument counts.
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
//!               │   any sink)  │
//!               └──────────────┘
//! ```

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod ast;
mod context;
mod engine;
mod error;
mod escapers;
mod expr;
mod filters;
mod parser;
mod value;
mod vm;

pub use context::{Context, IntoContext};
pub use engine::{Engine, EscapeMode};
pub use error::{Error, SerdeError};

#[doc(hidden)]
pub mod __macro_support {
    pub use alloc::{collections::BTreeMap, rc::Rc, string::String};
    pub use core::convert::Into;

    pub fn into_value<T: Into<crate::value::Value>>(v: T) -> crate::value::Value {
        v.into()
    }

    pub fn build_context(map: BTreeMap<String, crate::value::Value>) -> crate::context::Context {
        crate::context::Context(crate::value::Value::Map(Rc::new(map)))
    }
}

/// Build a context map for [`Engine::render`] without requiring `Serialize`.
///
/// Unquoted identifiers (e.g. `name`) are stringified automatically.
/// Quoted string keys (e.g. `"my-key"`) are also accepted for programmatic
/// construction from external data.
///
/// ```rust
/// use template::{Engine, EscapeMode, context};
///
/// let mut engine = Engine::new(EscapeMode::Text);
/// engine.add_template("t", "Hello, {{ name }}!").unwrap();
///
/// let result = engine.render("t", context! { name: "World" }).unwrap();
/// assert_eq!(result, "Hello, World!");
/// ```
///
/// Supports nesting, vectors, and all types that implement [`Into<Value>`]:
///
/// ```rust
/// use template::{Engine, EscapeMode, context};
///
/// let mut engine = Engine::new(EscapeMode::Text);
/// engine.add_template("t", "\
/// {% for item in items %}- {{ item }}
/// {% endfor %}").unwrap();
///
/// let result = engine.render("t", context! {
///     items: vec!["apple", "banana"],
/// }).unwrap();
/// assert_eq!(result, "- apple\n- banana\n");
/// ```
#[macro_export]
macro_rules! context {
    (@key $key:ident) => { stringify!($key) };
    (@key $key:expr) => { $key };

    ($($key:tt : $value:expr),* $(,)?) => {{
        let mut __map = $crate::__macro_support::BTreeMap::new();
        $(
            __map.insert(
                $crate::context!(@key $key).to_string(),
                $crate::__macro_support::into_value($value),
            );
        )*
        $crate::__macro_support::build_context(__map)
    }};
}

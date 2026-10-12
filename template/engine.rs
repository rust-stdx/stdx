use alloc::{
    collections::BTreeMap,
    format,
    string::{String, ToString},
    sync::Arc,
    vec,
    vec::Vec,
};
use core::fmt;

use crate::{
    ast::{Node, NodeList},
    context::{Context, IntoContext},
    error::Error,
    parser::TemplateParser,
    value::Value,
    vm::Renderer,
};

const MAX_EXTEND_DEPTH: usize = 128;

/// Default maximum number of items a `range()` call may produce.
///
/// This bounds the memory a template can allocate through `range()`. It can be
/// changed per engine with [`Engine::set_range_limit`].
pub const DEFAULT_RANGE_LIMIT: usize = 32_000;

/// Controls auto-escaping behavior.
///
/// - `Html` — auto-escapes `{{ ... }}` output (`&`, `<`, `>`, `"`, `'`)
/// - `Text` — no escaping, raw output
#[derive(Clone, Debug)]
pub enum EscapeMode {
    Html,
    Text,
}

pub(crate) struct ParsedTemplate {
    pub(crate) nodes: NodeList,
}

/// A template engine that compiles templates into an AST and renders them.
///
/// Templates are added by name and can reference each other via `{% include %}`
/// and `{% extends %}` / `{% block %}`.
///
/// # Trust model
///
/// Templates are **trusted code**, exactly like Jinja2 or MiniJinja: a template
/// can read any variable in the rendering context and can consume CPU/memory
/// within the configured limits. Never feed untrusted template *source* to
/// [`add_template`](Engine::add_template). User data belongs in the *context*,
/// which is escaped automatically in [`EscapeMode::Html`].
pub struct Engine {
    pub(crate) mode: EscapeMode,
    pub(crate) templates: BTreeMap<String, ParsedTemplate>,
    pub(crate) range_limit: usize,
}

impl Engine {
    /// Create a new engine with the given escaping mode.
    pub fn new(mode: EscapeMode) -> Self {
        Self {
            mode,
            templates: BTreeMap::new(),
            range_limit: DEFAULT_RANGE_LIMIT,
        }
    }

    /// Set the maximum number of items a `range()` call may produce.
    ///
    /// Defaults to [`DEFAULT_RANGE_LIMIT`]. Calls that would exceed the limit
    /// return a render error instead of allocating an unbounded amount of memory.
    pub fn set_range_limit(&mut self, limit: usize) {
        self.range_limit = limit;
    }

    /// Return the current `range()` item limit.
    pub fn range_limit(&self) -> usize {
        self.range_limit
    }

    /// Compile and register a template by name.
    ///
    /// Returns an error if the template source contains invalid syntax
    /// (e.g. unclosed `{{ }}`, mismatched `{% if %}` / `{% endif %}`),
    /// exceeded nesting limits, statically-detectable type errors, or if a
    /// template with the same name already exists.
    pub fn add_template(&mut self, name: &str, source: &str) -> Result<(), Error> {
        if self.templates.contains_key(name) {
            return Err(Error::parse(format!("template `{name}` already exists")));
        }
        let mut parser = TemplateParser::new(source);
        let nodes = parser.parse()?;

        self.templates.insert(
            name.to_string(),
            ParsedTemplate {
                nodes,
            },
        );
        Ok(())
    }

    /// Render a named template with the given context variables.
    ///
    /// Pass a [`Context`] built with the [`context!`] macro, or a reference to
    /// any type that implements [`Serialize`](serde::Serialize) (requires the
    /// `std` feature). Non-map values such as plain strings or numbers produce
    /// a render error.
    ///
    /// ```rust
    /// use template::{Engine, EscapeMode, context};
    ///
    /// let mut engine = Engine::new(EscapeMode::Text);
    /// engine.add_template("t", "Hello, {{ name }}!").unwrap();
    /// let result = engine.render("t", context! { name: "World" }).unwrap();
    /// assert_eq!(result, "Hello, World!");
    /// ```
    ///
    /// Returns an error if the template name is not registered or if an
    /// expression fails during rendering.
    pub fn render(&self, name: &str, context: impl IntoContext) -> Result<String, Error> {
        let mut output = String::new();
        self.render_to(name, context, &mut output)?;
        Ok(output)
    }

    /// Render a named template directly into any [`fmt::Write`] sink.
    ///
    /// This is the allocation-friendly counterpart of [`Engine::render`]: output
    /// can be streamed into an existing `String`, a `Vec<u8>` wrapper, or any
    /// other `fmt::Write` implementation without building an intermediate buffer.
    pub fn render_to(&self, name: &str, context: impl IntoContext, out: &mut impl fmt::Write) -> Result<(), Error> {
        let template = self
            .templates
            .get(name)
            .ok_or_else(|| Error::undefined_template(name))?;

        let context: Context = context.into_context().map_err(|e| Error::parse(e.0))?;

        self.render_with_extends(&template.nodes, &context.0, out, None, 0)
    }

    fn render_with_extends(
        &self,
        nodes: &NodeList,
        variables: &Value,
        output: &mut dyn fmt::Write,
        parent_blocks: Option<&BTreeMap<String, Vec<Arc<NodeList>>>>,
        extend_depth: usize,
    ) -> Result<(), Error> {
        if extend_depth >= MAX_EXTEND_DEPTH {
            return Err(Error::render(format!("extend depth limit ({MAX_EXTEND_DEPTH}) exceeded")));
        }
        // Check if this template extends another
        let extends_name = nodes.iter().find_map(|n| {
            if let Node::Extends(name) = n {
                Some(name.clone())
            } else {
                None
            }
        });

        if let Some(parent_name) = extends_name {
            // Collect blocks from this template (cheap `Arc` clones, no deep copy).
            let mut blocks: BTreeMap<String, Arc<NodeList>> = BTreeMap::new();
            for node in nodes {
                if let Node::Block(block) = node {
                    blocks.insert(block.name.clone(), Arc::clone(&block.body));
                }
            }

            // Get parent template
            let parent = self
                .templates
                .get(parent_name.as_str())
                .ok_or_else(|| Error::undefined_template(&parent_name))?;

            // Merge current blocks into the parent block chain.
            // Current blocks that shadow existing chain entries push onto the chain;
            // new blocks start a fresh chain.
            let mut merged_chain: BTreeMap<String, Vec<Arc<NodeList>>> = parent_blocks.cloned().unwrap_or_default();
            for (name, body) in &blocks {
                match merged_chain.get_mut(name) {
                    Some(chain) => chain.push(Arc::clone(body)),
                    None => {
                        merged_chain.insert(name.clone(), vec![Arc::clone(body)]);
                    }
                }
            }

            // Render parent with merged blocks
            self.render_with_extends(&parent.nodes, variables, output, Some(&merged_chain), extend_depth + 1)?;
        } else {
            // No extends - render normally with block overrides
            let mut renderer = Renderer::new(self, output, variables.clone());

            if let Some(blocks) = parent_blocks {
                for (name, chain) in blocks {
                    let mut full_chain = chain.clone();
                    if let Some(current_block) = nodes.iter().find_map(|n| {
                        if let Node::Block(b) = n {
                            if b.name == *name {
                                Some(Arc::clone(&b.body))
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }) {
                        full_chain.push(current_block);
                    }
                    renderer.block_overrides.insert(name.clone(), full_chain);
                }
            }

            renderer.render_nodes(nodes)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use serde::Serialize;

    use super::*;
    use crate::context;

    #[test]
    fn test_simple_template() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("hello", "Hello, {{ name }}!").unwrap();
        let result = engine.render("hello", context! { name: "World" }).unwrap();
        assert_eq!(result, "Hello, World!");
    }

    #[test]
    fn test_html_escape() {
        let mut engine = Engine::new(EscapeMode::Html);
        engine.add_template("t", "<p>{{ content }}</p>").unwrap();
        let result = engine
            .render("t", context! { content: "<script>alert('xss')</script>" })
            .unwrap();
        assert_eq!(result, "<p>&lt;script&gt;alert(&#39;xss&#39;)&lt;/script&gt;</p>");
    }

    #[test]
    fn test_text_no_escape() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{{ content }}").unwrap();
        let result = engine
            .render("t", context! { content: "<script>alert('xss')</script>" })
            .unwrap();
        assert_eq!(result, "<script>alert('xss')</script>");
    }

    #[test]
    fn test_if_true() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{% if show %}visible{% endif %}").unwrap();
        let result = engine.render("t", context! { show: true }).unwrap();
        assert_eq!(result, "visible");
    }

    #[test]
    fn test_if_false() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{% if show %}visible{% endif %}").unwrap();
        let result = engine.render("t", context! { show: false }).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_if_else() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine
            .add_template("t", "{% if show %}yes{% else %}no{% endif %}")
            .unwrap();
        let result = engine.render("t", context! { show: false }).unwrap();
        assert_eq!(result, "no");
    }

    #[test]
    fn test_if_elif_else() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine
            .add_template("t", "{% if x == 1 %}one{% elif x == 2 %}two{% else %}other{% endif %}")
            .unwrap();
        assert_eq!(engine.render("t", context! { x: 1 }).unwrap(), "one");
        assert_eq!(engine.render("t", context! { x: 2 }).unwrap(), "two");
        assert_eq!(engine.render("t", context! { x: 3 }).unwrap(), "other");
    }

    #[test]
    fn test_for_loop() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine
            .add_template("t", "{% for item in items %}{{ item }},{% endfor %}")
            .unwrap();
        let result = engine.render("t", context! { items: vec!["a", "b", "c"] }).unwrap();
        assert_eq!(result, "a,b,c,");
    }

    #[test]
    fn test_dotted_access() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{{ user.name }}").unwrap();
        let result = engine
            .render("t", context! { user: context! { name: "Alice" } })
            .unwrap();
        assert_eq!(result, "Alice");
    }

    #[test]
    fn test_filter_upper() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{{ name | upper }}").unwrap();
        let result = engine.render("t", context! { name: "hello" }).unwrap();
        assert_eq!(result, "HELLO");
    }

    #[test]
    fn test_filter_chain() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{{ name | upper | reverse }}").unwrap();
        let result = engine.render("t", context! { name: "abc" }).unwrap();
        assert_eq!(result, "CBA");
    }

    #[test]
    fn test_set_variable() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{% set x = 42 %}{{ x }}").unwrap();
        let result = engine.render("t", context! {}).unwrap();
        assert_eq!(result, "42");
    }

    #[test]
    fn test_raw_block() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine
            .add_template("t", "before{% raw %}{{ not processed }}{% endraw %}after")
            .unwrap();
        let result = engine.render("t", context! {}).unwrap();
        assert_eq!(result, "before{{ not processed }}after");
    }

    #[test]
    fn test_include() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("header", "Header").unwrap();
        engine.add_template("page", "{% include \"header\" %}Body").unwrap();
        let result = engine.render("page", context! {}).unwrap();
        assert_eq!(result, "HeaderBody");
    }

    #[test]
    fn test_comparisons() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{% if x == 1 %}eq{% endif %}").unwrap();
        assert_eq!(engine.render("t", context! { x: 1 }).unwrap(), "eq");
        assert_eq!(engine.render("t", context! { x: 2 }).unwrap(), "");
    }

    #[test]
    fn test_not_operator() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{% if not x %}empty{% endif %}").unwrap();
        let result = engine.render("t", context! { x: false }).unwrap();
        assert_eq!(result, "empty");
    }

    #[test]
    fn test_in_operator() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine
            .add_template("t", "{% if \"a\" in items %}found{% endif %}")
            .unwrap();
        let result = engine.render("t", context! { items: vec!["a", "b", "c"] }).unwrap();
        assert_eq!(result, "found");
    }

    #[test]
    fn test_nested_if_for() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine
            .add_template(
                "t",
                "{% for item in items %}{% if item.active %}{{ item.name }}{% endif %}{% endfor %}",
            )
            .unwrap();
        let result = engine
            .render(
                "t",
                context! {
                    items: vec![
                        context! { name: "a", active: true },
                        context! { name: "b", active: false },
                        context! { name: "c", active: true },
                    ]
                },
            )
            .unwrap();
        assert_eq!(result, "ac");
    }

    #[test]
    fn test_extends_and_blocks() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine
            .add_template("base", "before{% block content %}default{% endblock %}after")
            .unwrap();
        engine
            .add_template("child", "{% extends \"base\" %}{% block content %}child content{% endblock %}")
            .unwrap();
        let result = engine.render("child", context! {}).unwrap();
        assert_eq!(result, "beforechild contentafter");
    }

    #[test]
    fn test_super_in_block() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine
            .add_template("base", "{% block content %}parent{% endblock %}")
            .unwrap();
        engine
            .add_template(
                "child",
                "{% extends \"base\" %}{% block content %}{{ super() }} + child{% endblock %}",
            )
            .unwrap();
        let result = engine.render("child", context! {}).unwrap();
        assert_eq!(result, "parent + child");
    }

    #[test]
    fn test_comment_is_ignored() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "before{# comment #}after").unwrap();
        let result = engine.render("t", context! {}).unwrap();
        assert_eq!(result, "beforeafter");
    }

    #[test]
    fn test_empty_variable() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{{ missing }}").unwrap();
        let result = engine.render("t", context! {}).unwrap();
        assert_eq!(result, "");
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_struct_variables() {
        #[derive(Serialize)]
        struct User {
            name: String,
            age: i32,
        }

        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{{ name }} is {{ age }}").unwrap();
        let user = User {
            name: "Bob".into(),
            age: 30,
        };
        let result = engine.render("t", &user).unwrap();
        assert_eq!(result, "Bob is 30");
    }

    #[test]
    fn test_default_filter() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{{ name | default(\"unknown\") }}").unwrap();

        // With variable set
        let result = engine.render("t", context! { name: "Alice" }).unwrap();
        assert_eq!(result, "Alice");

        // Without variable
        let result = engine.render("t", context! {}).unwrap();
        assert_eq!(result, "unknown");
    }

    #[test]
    fn test_length_filter() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{{ items | length }}").unwrap();
        let result = engine.render("t", context! { items: vec![1, 2, 3] }).unwrap();
        assert_eq!(result, "3");
    }

    #[test]
    fn test_arithmetic() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{{ 1 + 2 * 3 }}").unwrap();
        let result = engine.render("t", context! {}).unwrap();
        assert_eq!(result, "7");
    }

    #[test]
    fn test_float_arithmetic() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{{ 3.5 + 2.5 }}").unwrap();
        let result = engine.render("t", context! {}).unwrap();
        assert_eq!(result, "6");
    }

    #[test]
    fn test_float_division() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{{ 10.0 / 3 }}").unwrap();
        let result = engine.render("t", context! {}).unwrap();
        assert!(
            result == "3.3333333333333335" || result == "3.333333333333333",
            "unexpected float division result: {result}"
        );
    }

    #[test]
    fn test_super_in_html_mode() {
        let mut engine = Engine::new(EscapeMode::Html);
        engine
            .add_template("base", "{% block content %}<b>parent</b>{% endblock %}")
            .unwrap();
        engine
            .add_template(
                "child",
                "{% extends \"base\" %}{% block content %}{{ super() }}<i>child</i>{% endblock %}",
            )
            .unwrap();
        let result = engine.render("child", context! {}).unwrap();
        assert_eq!(result, "<b>parent</b><i>child</i>");
    }

    #[test]
    fn test_multi_level_extends() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine
            .add_template("base", "{% block content %}base{% endblock %}")
            .unwrap();
        engine
            .add_template(
                "child",
                "{% extends \"base\" %}{% block content %}child {{ super() }}{% endblock %}",
            )
            .unwrap();
        engine
            .add_template(
                "grandchild",
                "{% extends \"child\" %}{% block content %}grandchild {{ super() }}{% endblock %}",
            )
            .unwrap();
        let result = engine.render("grandchild", context! {}).unwrap();
        assert_eq!(result, "grandchild child base");
    }

    #[test]
    fn test_safe_filter_html_mode() {
        let mut engine = Engine::new(EscapeMode::Html);
        engine.add_template("t", "{{ content | safe }}").unwrap();
        let result = engine.render("t", context! { content: "<b>bold</b>" }).unwrap();
        assert_eq!(result, "<b>bold</b>");
    }

    #[test]
    fn test_escape_filter_html_mode() {
        let mut engine = Engine::new(EscapeMode::Html);
        engine
            .add_template("t", "{{ content }} and {{ content | escape }}")
            .unwrap();
        let result = engine.render("t", context! { content: "<br>" }).unwrap();
        // Already auto-escaped in HTML mode, |escape should not double-escape
        assert_eq!(result, "&lt;br&gt; and &lt;br&gt;");
    }

    #[test]
    fn test_for_loop_over_string() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine
            .add_template("t", "{% for c in s %}{{ c }}|{% endfor %}")
            .unwrap();
        let result = engine.render("t", context! { s: "ab" }).unwrap();
        assert_eq!(result, "a|b|");
    }

    #[test]
    fn test_index_access() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{{ items[0] }},{{ items[1] }}").unwrap();
        let result = engine.render("t", context! { items: vec!["a", "b"] }).unwrap();
        assert_eq!(result, "a,b");
    }

    #[test]
    fn test_in_operator_string() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine
            .add_template("t", "{% if \"world\" in \"hello world\" %}found{% endif %}")
            .unwrap();
        let result = engine.render("t", context! {}).unwrap();
        assert_eq!(result, "found");
    }

    #[test]
    fn test_missing_endif_errors() {
        let mut engine = Engine::new(EscapeMode::Text);
        let result = engine.add_template("t", "{% if true %}hello");
        assert!(result.is_err());
    }

    #[test]
    fn test_length_filter_map() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{{ obj | length }}").unwrap();
        let result = engine.render("t", context! { obj: context! { a: 1, b: 2 } }).unwrap();
        assert_eq!(result, "2");
    }

    #[test]
    fn test_undefined_template_error() {
        let engine = Engine::new(EscapeMode::Text);
        let result = engine.render("nonexistent", context! {});
        assert!(result.is_err());
    }

    #[test]
    fn test_division_by_zero() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{{ 1 / 0 }}").unwrap();
        let result = engine.render("t", context! {});
        assert!(result.is_err());
    }

    #[test]
    fn test_modulo_by_zero() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{{ 10 % 0 }}").unwrap();
        let result = engine.render("t", context! {});
        assert!(result.is_err());
    }

    #[test]
    fn test_circular_include() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("a", "{% include \"b\" %}").unwrap();
        engine.add_template("b", "{% include \"a\" %}").unwrap();
        let result = engine.render("a", context! {});
        assert!(result.is_err());
    }

    #[test]
    fn test_circular_extends() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine
            .add_template("a", "{% extends \"b\" %}{% block x %}a{% endblock %}")
            .unwrap();
        engine
            .add_template("b", "{% extends \"a\" %}{% block x %}b{% endblock %}")
            .unwrap();
        let result = engine.render("a", context! {});
        assert!(result.is_err());
    }

    #[test]
    fn test_unknown_function() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{{ foobar() }}").unwrap();
        let result = engine.render("t", context! {});
        assert!(result.is_err());
    }

    #[test]
    fn test_range_with_float() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine
            .add_template("t", "{% for i in range(5.5) %}{{ i }}{% endfor %}")
            .unwrap();
        let result = engine.render("t", context! {});
        assert!(result.is_err());
    }

    #[test]
    fn test_trailing_tokens_in_expr() {
        let mut engine = Engine::new(EscapeMode::Text);
        let result = engine.add_template("t", "{{ true false }}");
        assert!(result.is_err());
    }

    #[test]
    fn test_trailing_tokens_after_filter() {
        let mut engine = Engine::new(EscapeMode::Text);
        let result = engine.add_template("t", "{{ name | upper + 1 }}");
        assert!(result.is_err());
    }

    #[test]
    fn test_add_template_overwrite_error() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "hello").unwrap();
        let result = engine.add_template("t", "world");
        assert!(result.is_err());
    }

    #[test]
    fn test_raw_block_preserves_inner_tags() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine
            .add_template("t", "before{% raw %}{% inner %}{% endraw %}after")
            .unwrap();
        let result = engine.render("t", context! {}).unwrap();
        assert_eq!(result, "before{% inner %}after");
    }

    #[test]
    fn test_raw_block_preserves_trailing_whitespace() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine
            .add_template("t", "before{% raw %}hello   {% endraw %}after")
            .unwrap();
        let result = engine.render("t", context! {}).unwrap();
        assert_eq!(result, "beforehello   after");
    }

    #[test]
    fn test_range_with_integer() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine
            .add_template("t", "{% for i in range(3) %}{{ i }}{% endfor %}")
            .unwrap();
        let result = engine.render("t", context! {}).unwrap();
        assert_eq!(result, "012");
    }

    #[test]
    fn test_length_filter_in_html_mode_on_safe() {
        let mut engine = Engine::new(EscapeMode::Html);
        engine.add_template("t", "{{ \"hello\" | escape | length }}").unwrap();
        let result = engine.render("t", context! {}).unwrap();
        // "hello" escaped is "hello" (no HTML chars), and length is 5 chars
        assert_eq!(result, "5");
    }

    #[test]
    fn test_first_on_safe_string() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{{ \"abc\" | safe | first }}").unwrap();
        let result = engine.render("t", context! {}).unwrap();
        assert_eq!(result, "a");
    }

    #[test]
    fn test_last_on_safe_string() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{{ \"abc\" | safe | last }}").unwrap();
        let result = engine.render("t", context! {}).unwrap();
        assert_eq!(result, "c");
    }

    #[test]
    fn test_reverse_on_safe_string() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{{ \"abc\" | safe | reverse }}").unwrap();
        let result = engine.render("t", context! {}).unwrap();
        assert_eq!(result, "cba");
    }

    #[test]
    fn test_length_on_safe_string() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{{ \"hello\" | safe | length }}").unwrap();
        let result = engine.render("t", context! {}).unwrap();
        assert_eq!(result, "5");
    }

    #[test]
    fn test_short_circuit_and() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{% if false and 1/0 %}ok{% endif %}").unwrap();
        let result = engine.render("t", context! {}).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_short_circuit_or() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{% if true or 1/0 %}ok{% endif %}").unwrap();
        let result = engine.render("t", context! {}).unwrap();
        assert_eq!(result, "ok");
    }

    #[test]
    fn test_range_no_args() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{{ range() }}").unwrap();
        let result = engine.render("t", context! {});
        assert!(result.is_err());
    }

    #[test]
    fn test_range_extra_args() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine
            .add_template("t", "{% for i in range(1,2,3) %}{{ i }}{% endfor %}")
            .unwrap();
        let result = engine.render("t", context! {});
        assert!(result.is_err());
    }

    #[test]
    fn test_for_empty_string() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine
            .add_template("t", "{% for c in \"\" %}{{ c }}{% endfor %}")
            .unwrap();
        let result = engine.render("t", context! {}).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_for_empty_array() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine
            .add_template("t", "{% for i in items %}{{ i }}{% endfor %}")
            .unwrap();
        let result = engine.render("t", context! { items: Vec::<i32>::new() }).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_for_over_number() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine
            .add_template("t", "{% for i in 42 %}{{ i }}{% endfor %}")
            .unwrap();
        let result = engine.render("t", context! {}).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_for_over_map() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{% for i in m %}{{ i }}{% endfor %}").unwrap();
        let result = engine.render("t", context! { m: context! { a: 1 } }).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_missing_include() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{% include \"missing\" %}").unwrap();
        let result = engine.render("t", context! {});
        assert!(result.is_err());
    }

    #[test]
    fn test_missing_extends() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{% extends \"missing\" %}").unwrap();
        let result = engine.render("t", context! {});
        assert!(result.is_err());
    }

    #[test]
    fn test_super_without_parent() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{{ super() }}").unwrap();
        let result = engine.render("t", context! {}).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_set_overwrite() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine
            .add_template("t", "{% set x = 1 %}{% set x = 2 %}{{ x }}")
            .unwrap();
        let result = engine.render("t", context! {}).unwrap();
        assert_eq!(result, "2");
    }

    #[test]
    fn test_nested_missing() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{{ a.b.c }}").unwrap();
        let result = engine.render("t", context! {}).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_negative_index_runtime() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{{ items[i] }}").unwrap();
        let result = engine.render("t", context! { items: vec![1, 2], i: -1 });
        assert!(result.is_err());
    }

    #[test]
    fn test_float_index_runtime() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{{ items[i] }}").unwrap();
        let result = engine.render("t", context! { items: vec![1, 2], i: 1.5 });
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_map_index() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{{ m.key }}").unwrap();
        let result = engine.render("t", context! {}).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_float_div_by_zero_inf() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{{ 1.0 / 0.0 }}").unwrap();
        let result = engine.render("t", context! {}).unwrap();
        assert_eq!(result, "inf");
    }

    #[test]
    fn test_neg_float_div_by_zero_neg_inf() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{{ -1.0 / 0.0 }}").unwrap();
        let result = engine.render("t", context! {}).unwrap();
        assert_eq!(result, "-inf");
    }

    #[test]
    fn test_float_zero_div_by_zero_nan() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{{ 0.0 / 0.0 }}").unwrap();
        let result = engine.render("t", context! {}).unwrap();
        assert_eq!(result, "NaN");
    }

    #[test]
    fn test_elif_after_else_error() {
        let mut engine = Engine::new(EscapeMode::Text);
        let result = engine.add_template("t", "{% if a %}b{% else %}c{% elif d %}e{% endif %}");
        assert!(result.is_err());
    }

    #[test]
    fn test_multiple_else_error() {
        let mut engine = Engine::new(EscapeMode::Text);
        let result = engine.add_template("t", "{% if a %}b{% else %}c{% else %}d{% endif %}");
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_if_condition_error() {
        let mut engine = Engine::new(EscapeMode::Text);
        let result = engine.add_template("t", "{% if %}a{% endif %}");
        assert!(result.is_err());
    }

    // ---- strict boolean semantics ----

    #[test]
    fn test_static_non_bool_or_is_parse_error() {
        let mut engine = Engine::new(EscapeMode::Text);
        assert!(engine.add_template("t", "{{ 0 or \"x\" }}").is_err());
        assert!(engine.add_template("t", "{{ \"x\" or true }}").is_err());
        assert!(engine.add_template("t", "{{ 1 and true }}").is_err());
        assert!(engine.add_template("t", "{{ not 0 }}").is_err());
        assert!(engine.add_template("t", "{% if 1 %}a{% endif %}").is_err());
        assert!(engine.add_template("t", "{% if \"a\" %}a{% endif %}").is_err());
    }

    #[test]
    fn test_or_and_return_values() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine
            .add_template(
                "t",
                "{{ false or \"x\" }}|{{ true or \"x\" }}|{{ true and \"y\" }}|{{ false and \"y\" }}",
            )
            .unwrap();
        // decision operand must be boolean; the fallback value is returned verbatim
        assert_eq!(engine.render("t", context! {}).unwrap(), "x|true|y|false");
    }

    #[test]
    fn test_or_chain_flattens() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{{ false or false or \"last\" }}").unwrap();
        assert_eq!(engine.render("t", context! {}).unwrap(), "last");
    }

    #[test]
    fn test_dynamic_non_bool_or_is_render_error() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{{ x or \"fallback\" }}").unwrap();
        assert!(engine.render("t", context! { x: 5 }).is_err());
        assert_eq!(engine.render("t", context! { x: false }).unwrap(), "fallback");
        assert_eq!(engine.render("t", context! { x: true }).unwrap(), "true");
    }

    #[test]
    fn test_strict_if_condition_rejects_non_bool() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{% if x %}y{% endif %}").unwrap();
        assert!(engine.render("t", context! { x: 0 }).is_err());
        assert!(engine.render("t", context! { x: 1 }).is_err());
        assert!(engine.render("t", context! { x: "s" }).is_err());
        assert!(engine.render("t", context! { x: vec![1] }).is_err());
        // missing => null, also not a boolean
        assert!(engine.render("t", context! {}).is_err());
        assert_eq!(engine.render("t", context! { x: true }).unwrap(), "y");
        assert_eq!(engine.render("t", context! { x: false }).unwrap(), "");
    }

    #[test]
    fn test_is_set() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{{ is_set(a.b.c) }}|{{ is_set(x) }}").unwrap();
        // Missing nested field must be false, never an error.
        assert_eq!(engine.render("t", context! {}).unwrap(), "false|false");
        assert_eq!(
            engine
                .render("t", context! { a: context! { b: context! { c: 1 } }, x: 0 })
                .unwrap(),
            "true|true"
        );
        // b exists but is not a map, so a.b.c is missing.
        assert_eq!(engine.render("t", context! { a: context! { b: 1 } }).unwrap(), "false|false");
    }

    #[test]
    fn test_is_set_wrong_arity() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{{ is_set() }}").unwrap();
        assert!(engine.render("t", context! {}).is_err());
    }

    // ---- arithmetic overflow ----

    #[test]
    fn test_integer_overflow_is_error_not_panic() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{{ x + y }}").unwrap();
        assert!(engine.render("t", context! { x: i64::MAX, y: 1i64 }).is_err());
        engine.add_template("u", "{{ x * y }}").unwrap();
        assert!(engine.render("u", context! { x: i64::MAX, y: 2i64 }).is_err());
        engine.add_template("v", "{{ x - y }}").unwrap();
        assert!(engine.render("v", context! { x: i64::MIN, y: 1i64 }).is_err());
        engine.add_template("w", "{{ -x }}").unwrap();
        assert!(engine.render("w", context! { x: i64::MIN }).is_err());
    }

    #[test]
    fn test_min_div_minus_one_is_error() {
        // These panic in plain Rust even in release mode; the engine must not.
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("d", "{{ x / -1 }}").unwrap();
        engine.add_template("m", "{{ x % -1 }}").unwrap();
        assert!(engine.render("d", context! { x: i64::MIN }).is_err());
        assert!(engine.render("m", context! { x: i64::MIN }).is_err());
    }

    // ---- range limit ----

    #[test]
    fn test_range_limit_default() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{{ range(n) | length }}").unwrap();
        assert_eq!(engine.render("t", context! { n: 32_000i64 }).unwrap(), "32000");
        assert!(engine.render("t", context! { n: 32_001i64 }).is_err());
        // A range that would overflow i64 must not panic.
        assert!(engine.render("t", context! { n: i64::MAX }).is_err());
    }

    #[test]
    fn test_range_limit_configurable() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.set_range_limit(3);
        assert_eq!(engine.range_limit(), 3);
        engine.add_template("t", "{{ range(n) | join(\",\") }}").unwrap();
        assert_eq!(engine.render("t", context! { n: 3i64 }).unwrap(), "0,1,2");
        assert!(engine.render("t", context! { n: 4i64 }).is_err());
    }

    #[test]
    fn test_range_empty_and_reversed() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{{ range(a, b) | join(\",\") }}").unwrap();
        assert_eq!(engine.render("t", context! { a: 5i64, b: 3i64 }).unwrap(), "");
        assert_eq!(engine.render("t", context! { a: -2i64, b: 2i64 }).unwrap(), "-2,-1,0,1");
    }

    // ---- recursion limits ----

    #[test]
    fn test_template_nesting_limit_returns_error() {
        let mut engine = Engine::new(EscapeMode::Text);
        let ok = "{% if true %}".repeat(100) + &"{% endif %}".repeat(100);
        engine.add_template("ok", &ok).unwrap();
        let too_deep = "{% if true %}".repeat(200) + &"{% endif %}".repeat(200);
        assert!(engine.add_template("deep", &too_deep).is_err());
    }

    #[test]
    fn test_expression_nesting_limit_returns_error() {
        let mut engine = Engine::new(EscapeMode::Text);
        let ok = "{{ ".to_string() + &"(".repeat(50) + "1" + &")".repeat(50) + " }}";
        engine.add_template("ok", &ok).unwrap();
        let too_deep = "{{ ".to_string() + &"(".repeat(200) + "1" + &")".repeat(200) + " }}";
        assert!(engine.add_template("deep", &too_deep).is_err());
    }

    #[test]
    fn test_huge_expression_rejected() {
        let mut engine = Engine::new(EscapeMode::Text);
        // A long filter chain must be rejected instead of overflowing the stack.
        let t = format!("{{{{ x{} }}}}", "|upper".repeat(5000));
        assert!(engine.add_template("t", &t).is_err());
    }

    // ---- include / extends ----

    #[test]
    fn test_extends_must_be_top_level() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("base", "{% block b %}d{% endblock %}").unwrap();
        assert!(
            engine
                .add_template("c", "{% if true %}{% extends \"base\" %}{% endif %}")
                .is_err()
        );
    }

    #[test]
    fn test_include_with_extends_is_error() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("base", "BASE{% block b %}d{% endblock %}").unwrap();
        engine
            .add_template("sub", "{% extends \"base\" %}{% block b %}S{% endblock %}")
            .unwrap();
        engine.add_template("page", "{% include \"sub\" %}").unwrap();
        assert!(engine.render("page", context! {}).is_err());
    }

    // ---- parser correctness ----

    #[test]
    fn test_delimiters_inside_string_literals() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{{ \"a}}b\" }}").unwrap();
        assert_eq!(engine.render("t", context! {}).unwrap(), "a}}b");
        engine
            .add_template("u", "{% if x == \"%}\" %}Y{% else %}N{% endif %}")
            .unwrap();
        assert_eq!(engine.render("u", context! { x: "%}" }).unwrap(), "Y");
        assert_eq!(engine.render("u", context! { x: "nope" }).unwrap(), "N");
    }

    #[test]
    fn test_unterminated_string_is_parse_error() {
        let mut engine = Engine::new(EscapeMode::Text);
        assert!(engine.add_template("t", "{{ \"abc }}").is_err());
        assert!(engine.add_template("u", "{% if x == \"abc %}y{% endif %}").is_err());
    }

    #[test]
    fn test_unknown_filter_is_parse_error() {
        let mut engine = Engine::new(EscapeMode::Text);
        assert!(engine.add_template("t", "{{ x | nope }}").is_err());
    }

    #[test]
    fn test_wrong_filter_arity_is_parse_error() {
        let mut engine = Engine::new(EscapeMode::Text);
        assert!(engine.add_template("t", "{{ x | upper(\"junk\") }}").is_err());
        assert!(engine.add_template("u", "{{ x | join(\",\", \"extra\") }}").is_err());
        // Valid arities still work.
        assert!(engine.add_template("v", "{{ x | default(1) }}").is_ok());
        assert!(engine.add_template("w", "{{ x | upper }}").is_ok());
    }

    #[test]
    fn test_default_is_null_only() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine
            .add_template(
                "t",
                "{{ z | default(9) }}|{{ f | default(9) }}|{{ s | default(9) }}|{{ m | default(9) }}",
            )
            .unwrap();
        let out = engine.render("t", context! { z: 0, f: false, s: "" }).unwrap();
        assert_eq!(out, "0|false||9");
    }

    // ---- Safe propagation ----

    #[test]
    fn test_safe_survives_pure_filters() {
        let mut engine = Engine::new(EscapeMode::Html);
        engine.add_template("t", "{{ x | safe | reverse }}").unwrap();
        // Safe status is preserved, so the result is not escaped again.
        assert_eq!(engine.render("t", context! { x: "<b>" }).unwrap(), ">b<");
    }

    #[test]
    fn test_safe_compares_equal_to_str() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine
            .add_template("t", "{{ (x | safe) == x }}|{{ (x | safe) < \"z\" }}")
            .unwrap();
        assert_eq!(engine.render("t", context! { x: "a" }).unwrap(), "true|true");
    }

    #[test]
    fn test_join_leaves_unsafe_element_escaped() {
        let mut engine = Engine::new(EscapeMode::Html);
        engine.add_template("t", "{{ items | join(\",\") }}").unwrap();
        let out = engine.render("t", context! { items: vec!["<a>", "<b>"] }).unwrap();
        assert_eq!(out, "&lt;a&gt;,&lt;b&gt;");
    }

    // ---- injection ----

    #[test]
    fn test_xss_payloads_are_escaped() {
        let mut engine = Engine::new(EscapeMode::Html);
        engine.add_template("t", "{{ x }}").unwrap();
        for payload in [
            "<script>alert(1)</script>",
            "\" onmouseover=alert(1)",
            "' onclick='alert(1)",
            "`backtick`",
            "&amp;",
            "javascript:alert(1)",
        ] {
            let out = engine.render("t", context! { x: payload }).unwrap();
            assert!(!out.contains('<'), "raw `<` leaked for {payload:?}");
            assert!(!out.contains('>'), "raw `>` leaked for {payload:?}");
            assert!(!out.contains('"'), "raw `\"` leaked for {payload:?}");
            assert!(!out.contains('\''), "raw `'` leaked for {payload:?}");
        }
    }

    #[test]
    fn test_escape_filter_does_not_double_escape() {
        let mut engine = Engine::new(EscapeMode::Html);
        engine.add_template("t", "{{ x | escape }}").unwrap();
        assert_eq!(engine.render("t", context! { x: "<br>" }).unwrap(), "&lt;br&gt;");
    }

    // ---- render_to ----

    #[test]
    fn test_render_to_writer() {
        use core::fmt;

        struct Sink(String);
        impl fmt::Write for Sink {
            fn write_str(&mut self, s: &str) -> fmt::Result {
                self.0.push_str(s);
                Ok(())
            }
        }

        let mut engine = Engine::new(EscapeMode::Html);
        engine.add_template("t", "Hi {{ name }}").unwrap();
        let mut sink = Sink(String::new());
        engine.render_to("t", context! { name: "<b>" }, &mut sink).unwrap();
        assert_eq!(sink.0, "Hi &lt;b&gt;");
    }

    // ---- error positions ----

    #[test]
    fn test_error_position_is_template_absolute() {
        let mut engine = Engine::new(EscapeMode::Text);
        // `-1` sits at column 30 of the template, not column 8 of the expression.
        let err = engine
            .add_template("t", "aaaaaaaaaaaaaaaaaaaa{{ items[-1] }}")
            .unwrap_err();
        assert!(
            err.to_string().contains("1:30"),
            "expected template-absolute position 1:30, got: {err}"
        );
    }

    #[test]
    fn test_error_position_on_later_line() {
        let mut engine = Engine::new(EscapeMode::Text);
        let src = "a\n{{ items[-1] }}";
        let err = engine.add_template("t", src).unwrap_err();
        assert!(err.to_string().contains("2:10"), "expected position 2:10, got: {err}");
    }

    #[test]
    fn test_unknown_filter_position() {
        let mut engine = Engine::new(EscapeMode::Text);
        let err = engine.add_template("t", "prefix {{ x | nope }}").unwrap_err();
        // `nope` starts at column 15 (`prefix {{ x | ` is 14 chars).
        assert!(err.to_string().contains("1:15"), "expected position 1:15, got: {err}");
    }

    #[test]
    fn test_multiline_tag_error_position() {
        let mut engine = Engine::new(EscapeMode::Text);
        // The tag content spans two lines; the negative index is on line 2.
        let err = engine
            .add_template("t", "{% if items[\n-1] %}x{% endif %}")
            .unwrap_err();
        assert!(err.to_string().contains("2:1"), "expected position 2:1, got: {err}");
    }

    // ---- endblock labels ----

    #[test]
    fn test_endblock_with_matching_label() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine
            .add_template("t", "{% block content %}hi{% endblock content %}")
            .unwrap();
        assert_eq!(engine.render("t", context! {}).unwrap(), "hi");
    }

    #[test]
    fn test_endblock_without_label() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{% block content %}hi{% endblock %}").unwrap();
        assert_eq!(engine.render("t", context! {}).unwrap(), "hi");
    }

    #[test]
    fn test_endblock_with_mismatched_label_is_error() {
        let mut engine = Engine::new(EscapeMode::Text);
        let err = engine
            .add_template("t", "{% block content %}hi{% endblock footer %}")
            .unwrap_err();
        assert!(
            err.to_string().contains("endblock content"),
            "expected mismatch message, got: {err}"
        );
    }

    #[test]
    fn test_endfor_label_still_rejected() {
        let mut engine = Engine::new(EscapeMode::Text);
        assert!(engine.add_template("t", "{% for i in x %}{% endfor i %}").is_err());
    }

    #[test]
    fn test_non_ascii_identifier_rejected() {
        let mut engine = Engine::new(EscapeMode::Text);
        assert!(engine.add_template("t", "{{ café }}").is_err());
    }
}

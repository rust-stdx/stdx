use alloc::{
    collections::BTreeMap,
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};

use crate::{
    ast::{Node, NodeList},
    error::Error,
    parser::TemplateParser,
    value::{Context, IntoContext, Value},
    vm::Renderer,
};

const MAX_EXTEND_DEPTH: usize = 128;

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
pub struct Engine {
    pub(crate) mode: EscapeMode,
    pub(crate) templates: BTreeMap<String, ParsedTemplate>,
}

impl Engine {
    /// Create a new engine with the given escaping mode.
    pub fn new(mode: EscapeMode) -> Self {
        Self {
            mode,
            templates: BTreeMap::new(),
        }
    }

    /// Compile and register a template by name.
    ///
    /// Returns an error if the template source contains invalid syntax
    /// (e.g. unclosed `{{ }}`, mismatched `{% if %}` / `{% endif %}`)
    /// or if a template with the same name already exists.
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
    /// Pass a [`Context`] built with the [`context!`] macro, or any type that
    /// implements [`Serialize`](serde::Serialize) (requires the `std` feature).
    /// Non-map values such as plain strings or numbers produce a render error.
    ///
    /// ```rust
    /// use template::{Engine, EscapeMode, context};
    ///
    /// let mut engine = Engine::new(EscapeMode::Text);
    /// engine.add_template("t", "Hello, {{ name }}!").unwrap();
    /// let result = engine.render("t", &context! { name: "World" }).unwrap();
    /// assert_eq!(result, "Hello, World!");
    /// ```
    ///
    /// Returns an error if the template name is not registered or if an
    /// expression fails during rendering.
    pub fn render<C: IntoContext + ?Sized>(&self, name: &str, variables: &C) -> Result<String, Error> {
        let template = self
            .templates
            .get(name)
            .ok_or_else(|| Error::undefined_template(name))?;

        let context: Context = variables.into_context().map_err(|e| Error::parse(e.0))?;

        let mut output = String::new();

        self.render_with_extends(&template.nodes, &context.0, &mut output, None, 0)?;

        Ok(output)
    }

    fn render_with_extends(
        &self,
        nodes: &NodeList,
        variables: &Value,
        output: &mut String,
        parent_blocks: Option<&BTreeMap<String, Vec<NodeList>>>,
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
            // Collect blocks from this template
            let mut blocks: BTreeMap<String, NodeList> = BTreeMap::new();
            for node in nodes {
                if let Node::Block(block) = node {
                    blocks.insert(block.name.clone(), block.body.clone());
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
            let mut merged_chain: BTreeMap<String, Vec<NodeList>> = if let Some(pb) = parent_blocks {
                pb.clone()
            } else {
                BTreeMap::new()
            };
            for (name, body) in &blocks {
                match merged_chain.get_mut(name) {
                    Some(chain) => chain.push(body.clone()),
                    None => {
                        merged_chain.insert(name.clone(), vec![body.clone()]);
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
                            if b.name == *name { Some(b.body.clone()) } else { None }
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
        let result = engine.render("hello", &context! { name: "World" }).unwrap();
        assert_eq!(result, "Hello, World!");
    }

    #[test]
    fn test_html_escape() {
        let mut engine = Engine::new(EscapeMode::Html);
        engine.add_template("t", "<p>{{ content }}</p>").unwrap();
        let result = engine
            .render("t", &context! { content: "<script>alert('xss')</script>" })
            .unwrap();
        assert_eq!(result, "<p>&lt;script&gt;alert(&#39;xss&#39;)&lt;/script&gt;</p>");
    }

    #[test]
    fn test_text_no_escape() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{{ content }}").unwrap();
        let result = engine
            .render("t", &context! { content: "<script>alert('xss')</script>" })
            .unwrap();
        assert_eq!(result, "<script>alert('xss')</script>");
    }

    #[test]
    fn test_if_true() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{% if show %}visible{% endif %}").unwrap();
        let result = engine.render("t", &context! { show: true }).unwrap();
        assert_eq!(result, "visible");
    }

    #[test]
    fn test_if_false() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{% if show %}visible{% endif %}").unwrap();
        let result = engine.render("t", &context! { show: false }).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_if_else() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine
            .add_template("t", "{% if show %}yes{% else %}no{% endif %}")
            .unwrap();
        let result = engine.render("t", &context! { show: false }).unwrap();
        assert_eq!(result, "no");
    }

    #[test]
    fn test_if_elif_else() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine
            .add_template("t", "{% if x == 1 %}one{% elif x == 2 %}two{% else %}other{% endif %}")
            .unwrap();
        assert_eq!(engine.render("t", &context! { x: 1 }).unwrap(), "one");
        assert_eq!(engine.render("t", &context! { x: 2 }).unwrap(), "two");
        assert_eq!(engine.render("t", &context! { x: 3 }).unwrap(), "other");
    }

    #[test]
    fn test_for_loop() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine
            .add_template("t", "{% for item in items %}{{ item }},{% endfor %}")
            .unwrap();
        let result = engine.render("t", &context! { items: vec!["a", "b", "c"] }).unwrap();
        assert_eq!(result, "a,b,c,");
    }

    #[test]
    fn test_dotted_access() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{{ user.name }}").unwrap();
        let result = engine
            .render("t", &context! { user: context! { name: "Alice" } })
            .unwrap();
        assert_eq!(result, "Alice");
    }

    #[test]
    fn test_filter_upper() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{{ name | upper }}").unwrap();
        let result = engine.render("t", &context! { name: "hello" }).unwrap();
        assert_eq!(result, "HELLO");
    }

    #[test]
    fn test_filter_chain() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{{ name | upper | reverse }}").unwrap();
        let result = engine.render("t", &context! { name: "abc" }).unwrap();
        assert_eq!(result, "CBA");
    }

    #[test]
    fn test_set_variable() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{% set x = 42 %}{{ x }}").unwrap();
        let result = engine.render("t", &context! {}).unwrap();
        assert_eq!(result, "42");
    }

    #[test]
    fn test_raw_block() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine
            .add_template("t", "before{% raw %}{{ not processed }}{% endraw %}after")
            .unwrap();
        let result = engine.render("t", &context! {}).unwrap();
        assert_eq!(result, "before{{ not processed }}after");
    }

    #[test]
    fn test_include() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("header", "Header").unwrap();
        engine.add_template("page", "{% include \"header\" %}Body").unwrap();
        let result = engine.render("page", &context! {}).unwrap();
        assert_eq!(result, "HeaderBody");
    }

    #[test]
    fn test_comparisons() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{% if x == 1 %}eq{% endif %}").unwrap();
        assert_eq!(engine.render("t", &context! { x: 1 }).unwrap(), "eq");
        assert_eq!(engine.render("t", &context! { x: 2 }).unwrap(), "");
    }

    #[test]
    fn test_not_operator() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{% if not x %}empty{% endif %}").unwrap();
        let result = engine.render("t", &context! { x: false }).unwrap();
        assert_eq!(result, "empty");
    }

    #[test]
    fn test_in_operator() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine
            .add_template("t", "{% if \"a\" in items %}found{% endif %}")
            .unwrap();
        let result = engine.render("t", &context! { items: vec!["a", "b", "c"] }).unwrap();
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
                &context! {
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
        let result = engine.render("child", &context! {}).unwrap();
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
        let result = engine.render("child", &context! {}).unwrap();
        assert_eq!(result, "parent + child");
    }

    #[test]
    fn test_comment_is_ignored() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "before{# comment #}after").unwrap();
        let result = engine.render("t", &context! {}).unwrap();
        assert_eq!(result, "beforeafter");
    }

    #[test]
    fn test_empty_variable() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{{ missing }}").unwrap();
        let result = engine.render("t", &context! {}).unwrap();
        assert_eq!(result, "");
    }

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
        let result = engine.render("t", &context! { name: "Alice" }).unwrap();
        assert_eq!(result, "Alice");

        // Without variable
        let result = engine.render("t", &context! {}).unwrap();
        assert_eq!(result, "unknown");
    }

    #[test]
    fn test_length_filter() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{{ items | length }}").unwrap();
        let result = engine.render("t", &context! { items: vec![1, 2, 3] }).unwrap();
        assert_eq!(result, "3");
    }

    #[test]
    fn test_arithmetic() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{{ 1 + 2 * 3 }}").unwrap();
        let result = engine.render("t", &context! {}).unwrap();
        assert_eq!(result, "7");
    }

    #[test]
    fn test_float_arithmetic() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{{ 3.5 + 2.5 }}").unwrap();
        let result = engine.render("t", &context! {}).unwrap();
        assert_eq!(result, "6");
    }

    #[test]
    fn test_float_division() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{{ 10.0 / 3 }}").unwrap();
        let result = engine.render("t", &context! {}).unwrap();
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
        let result = engine.render("child", &context! {}).unwrap();
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
        let result = engine.render("grandchild", &context! {}).unwrap();
        assert_eq!(result, "grandchild child base");
    }

    #[test]
    fn test_safe_filter_html_mode() {
        let mut engine = Engine::new(EscapeMode::Html);
        engine.add_template("t", "{{ content | safe }}").unwrap();
        let result = engine.render("t", &context! { content: "<b>bold</b>" }).unwrap();
        assert_eq!(result, "<b>bold</b>");
    }

    #[test]
    fn test_escape_filter_html_mode() {
        let mut engine = Engine::new(EscapeMode::Html);
        engine
            .add_template("t", "{{ content }} and {{ content | escape }}")
            .unwrap();
        let result = engine.render("t", &context! { content: "<br>" }).unwrap();
        // Already auto-escaped in HTML mode, |escape should not double-escape
        assert_eq!(result, "&lt;br&gt; and &lt;br&gt;");
    }

    #[test]
    fn test_for_loop_over_string() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine
            .add_template("t", "{% for c in s %}{{ c }}|{% endfor %}")
            .unwrap();
        let result = engine.render("t", &context! { s: "ab" }).unwrap();
        assert_eq!(result, "a|b|");
    }

    #[test]
    fn test_index_access() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{{ items[0] }},{{ items[1] }}").unwrap();
        let result = engine.render("t", &context! { items: vec!["a", "b"] }).unwrap();
        assert_eq!(result, "a,b");
    }

    #[test]
    fn test_in_operator_string() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine
            .add_template("t", "{% if \"world\" in \"hello world\" %}found{% endif %}")
            .unwrap();
        let result = engine.render("t", &context! {}).unwrap();
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
        let result = engine.render("t", &context! { obj: context! { a: 1, b: 2 } }).unwrap();
        assert_eq!(result, "2");
    }

    #[test]
    fn test_undefined_template_error() {
        let engine = Engine::new(EscapeMode::Text);
        let result = engine.render("nonexistent", &context! {});
        assert!(result.is_err());
    }

    #[test]
    fn test_division_by_zero() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{{ 1 / 0 }}").unwrap();
        let result = engine.render("t", &context! {});
        assert!(result.is_err());
    }

    #[test]
    fn test_modulo_by_zero() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{{ 10 % 0 }}").unwrap();
        let result = engine.render("t", &context! {});
        assert!(result.is_err());
    }

    #[test]
    fn test_circular_include() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("a", "{% include \"b\" %}").unwrap();
        engine.add_template("b", "{% include \"a\" %}").unwrap();
        let result = engine.render("a", &context! {});
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
        let result = engine.render("a", &context! {});
        assert!(result.is_err());
    }

    #[test]
    fn test_unknown_function() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{{ foobar() }}").unwrap();
        let result = engine.render("t", &context! {});
        assert!(result.is_err());
    }

    #[test]
    fn test_range_with_float() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine
            .add_template("t", "{% for i in range(5.5) %}{{ i }}{% endfor %}")
            .unwrap();
        let result = engine.render("t", &context! {});
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
        let result = engine.render("t", &context! {}).unwrap();
        assert_eq!(result, "before{% inner %}after");
    }

    #[test]
    fn test_raw_block_preserves_trailing_whitespace() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine
            .add_template("t", "before{% raw %}hello   {% endraw %}after")
            .unwrap();
        let result = engine.render("t", &context! {}).unwrap();
        assert_eq!(result, "beforehello   after");
    }

    #[test]
    fn test_range_with_integer() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine
            .add_template("t", "{% for i in range(3) %}{{ i }}{% endfor %}")
            .unwrap();
        let result = engine.render("t", &context! {}).unwrap();
        assert_eq!(result, "012");
    }

    #[test]
    fn test_length_filter_in_html_mode_on_safe() {
        let mut engine = Engine::new(EscapeMode::Html);
        engine.add_template("t", "{{ \"hello\" | escape | length }}").unwrap();
        let result = engine.render("t", &context! {}).unwrap();
        // "hello" escaped is "hello" (no HTML chars), and length is 5 chars
        assert_eq!(result, "5");
    }

    #[test]
    fn test_first_on_safe_string() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{{ \"abc\" | safe | first }}").unwrap();
        let result = engine.render("t", &context! {}).unwrap();
        assert_eq!(result, "a");
    }

    #[test]
    fn test_last_on_safe_string() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{{ \"abc\" | safe | last }}").unwrap();
        let result = engine.render("t", &context! {}).unwrap();
        assert_eq!(result, "c");
    }

    #[test]
    fn test_reverse_on_safe_string() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{{ \"abc\" | safe | reverse }}").unwrap();
        let result = engine.render("t", &context! {}).unwrap();
        assert_eq!(result, "cba");
    }

    #[test]
    fn test_length_on_safe_string() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{{ \"hello\" | safe | length }}").unwrap();
        let result = engine.render("t", &context! {}).unwrap();
        assert_eq!(result, "5");
    }

    #[test]
    fn test_short_circuit_and() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{% if false and 1/0 %}ok{% endif %}").unwrap();
        let result = engine.render("t", &context! {}).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_short_circuit_or() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{% if true or 1/0 %}ok{% endif %}").unwrap();
        let result = engine.render("t", &context! {}).unwrap();
        assert_eq!(result, "ok");
    }

    #[test]
    fn test_range_no_args() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{{ range() }}").unwrap();
        let result = engine.render("t", &context! {});
        assert!(result.is_err());
    }

    #[test]
    fn test_range_extra_args() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine
            .add_template("t", "{% for i in range(1,2,3) %}{{ i }}{% endfor %}")
            .unwrap();
        let result = engine.render("t", &context! {});
        assert!(result.is_err());
    }

    #[test]
    fn test_for_empty_string() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine
            .add_template("t", "{% for c in \"\" %}{{ c }}{% endfor %}")
            .unwrap();
        let result = engine.render("t", &context! {}).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_for_empty_array() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine
            .add_template("t", "{% for i in items %}{{ i }}{% endfor %}")
            .unwrap();
        let result = engine.render("t", &context! { items: Vec::<i32>::new() }).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_for_over_number() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine
            .add_template("t", "{% for i in 42 %}{{ i }}{% endfor %}")
            .unwrap();
        let result = engine.render("t", &context! {}).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_for_over_map() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{% for i in m %}{{ i }}{% endfor %}").unwrap();
        let result = engine.render("t", &context! { m: context! { a: 1 } }).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_missing_include() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{% include \"missing\" %}").unwrap();
        let result = engine.render("t", &context! {});
        assert!(result.is_err());
    }

    #[test]
    fn test_missing_extends() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{% extends \"missing\" %}").unwrap();
        let result = engine.render("t", &context! {});
        assert!(result.is_err());
    }

    #[test]
    fn test_super_without_parent() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{{ super() }}").unwrap();
        let result = engine.render("t", &context! {}).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_set_overwrite() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine
            .add_template("t", "{% set x = 1 %}{% set x = 2 %}{{ x }}")
            .unwrap();
        let result = engine.render("t", &context! {}).unwrap();
        assert_eq!(result, "2");
    }

    #[test]
    fn test_nested_missing() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{{ a.b.c }}").unwrap();
        let result = engine.render("t", &context! {}).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_negative_index_runtime() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{{ items[i] }}").unwrap();
        let result = engine.render("t", &context! { items: vec![1, 2], i: -1 });
        assert!(result.is_err());
    }

    #[test]
    fn test_float_index_runtime() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{{ items[i] }}").unwrap();
        let result = engine.render("t", &context! { items: vec![1, 2], i: 1.5 });
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_map_index() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{{ m.key }}").unwrap();
        let result = engine.render("t", &context! {}).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_float_div_by_zero_inf() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{{ 1.0 / 0.0 }}").unwrap();
        let result = engine.render("t", &context! {}).unwrap();
        assert_eq!(result, "inf");
    }

    #[test]
    fn test_neg_float_div_by_zero_neg_inf() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{{ -1.0 / 0.0 }}").unwrap();
        let result = engine.render("t", &context! {}).unwrap();
        assert_eq!(result, "-inf");
    }

    #[test]
    fn test_float_zero_div_by_zero_nan() {
        let mut engine = Engine::new(EscapeMode::Text);
        engine.add_template("t", "{{ 0.0 / 0.0 }}").unwrap();
        let result = engine.render("t", &context! {}).unwrap();
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
}

//! Template source parser — transforms template text into a `NodeList` AST.

use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};

use memchr::memchr;

use crate::{
    ast::{BlockNode, ElifNode, ForNode, IfNode, Node, NodeList},
    error::Error,
    expr::{ExprParser, lex_expr},
};

pub struct TemplateParser {
    source: String,
    pos: usize,
    line: usize,
    col: usize,
}

impl TemplateParser {
    pub fn new(source: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            pos: 0,
            line: 1,
            col: 1,
        }
    }

    pub fn parse(&mut self) -> Result<NodeList, Error> {
        let nodes = self.parse_nodes(&[])?;
        Ok(nodes)
    }

    fn eof(&self) -> bool {
        self.pos >= self.source.len()
    }

    fn peek(&self) -> Option<char> {
        self.source[self.pos..].chars().next()
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.pos += c.len_utf8();
        if c == '\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
        Some(c)
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_ascii_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn current_position(&self) -> (usize, usize) {
        (self.line, self.col)
    }

    fn expect_tag_close(&mut self, tag_type: char) -> Result<(), Error> {
        let (l, c) = self.current_position();
        self.skip_whitespace();
        if self.peek() != Some(tag_type) {
            return Err(Error::syntax(format!("expected `{tag_type}`"), l, c));
        }
        self.advance();
        if self.peek() != Some('}') {
            return Err(Error::syntax("expected `}`", l, c + 1));
        }
        self.advance();
        Ok(())
    }

    /// Parse nodes until EOF or until a `{% terminator %}` is encountered.
    /// The terminator is NOT consumed.
    fn parse_nodes(&mut self, terminators: &[&str]) -> Result<NodeList, Error> {
        let mut nodes = Vec::new();
        let mut raw_buf = String::new();

        while !self.eof() {
            // Fast-path: use memchr to find the next '{' byte, skipping raw text in bulk
            let haystack = &self.source.as_bytes()[self.pos..];
            match memchr(b'{', haystack) {
                Some(offset) => {
                    // Bulk-copy raw text before the '{'
                    if offset > 0 {
                        raw_buf.push_str(&self.source[self.pos..self.pos + offset]);
                        for &b in &haystack[..offset] {
                            if b == b'\n' {
                                self.line += 1;
                                self.col = 1;
                            } else {
                                self.col += 1;
                            }
                        }
                        self.pos += offset;
                    }

                    let bytes = self.source.as_bytes();
                    if self.pos + 1 < self.source.len() {
                        let next = bytes[self.pos + 1];
                        if next == b'{' || next == b'%' || next == b'#' {
                            if !raw_buf.is_empty() {
                                nodes.push(Node::Raw(core::mem::take(&mut raw_buf)));
                            }

                            if next == b'{' {
                                self.parse_expr_tag(&mut nodes)?;
                            } else if next == b'%' {
                                if self.is_terminator(terminators) {
                                    break;
                                }
                                self.parse_block_tag(&mut nodes)?;
                            } else if next == b'#' {
                                self.parse_comment_tag()?;
                            }
                            continue;
                        }
                    }

                    // Bare '{' not followed by {{, {% or {#
                    raw_buf.push('{');
                    self.pos += 1;
                    self.col += 1;
                }
                None => {
                    // No more '{' — bulk-copy remaining text as raw
                    let rest = &self.source[self.pos..];
                    raw_buf.push_str(rest);
                    for &b in haystack {
                        if b == b'\n' {
                            self.line += 1;
                            self.col = 1;
                        } else {
                            self.col += 1;
                        }
                    }
                    self.pos = self.source.len();
                    break;
                }
            }
        }

        if !raw_buf.is_empty() {
            nodes.push(Node::Raw(raw_buf));
        }

        Ok(nodes)
    }

    /// Peek at the keyword of a `{% ... %}` block without consuming it.
    /// Returns None if we're not at a `{%` start.
    fn peek_block_keyword(&self) -> Option<&str> {
        let bytes = self.source.as_bytes();
        if bytes.get(self.pos).copied() != Some(b'{') {
            return None;
        }
        if bytes.get(self.pos + 1).copied() != Some(b'%') {
            return None;
        }
        let mut i = self.pos + 2;
        while i < bytes.len() && bytes[i].is_ascii_whitespace() {
            i += 1;
        }
        if i < bytes.len() && (bytes[i].is_ascii_alphabetic() || bytes[i] == b'_') {
            let start = i;
            while i < bytes.len() && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
                i += 1;
            }
            Some(&self.source[start..i])
        } else {
            None
        }
    }

    fn is_terminator(&self, terminators: &[&str]) -> bool {
        if let Some(kw) = self.peek_block_keyword() {
            terminators.contains(&kw)
        } else {
            false
        }
    }

    fn parse_expr_tag(&mut self, nodes: &mut NodeList) -> Result<(), Error> {
        self.advance();
        self.advance();
        let (l, c) = self.current_position();
        let content = self.read_until("}}")?;
        self.advance();
        self.advance();
        if content.trim().is_empty() {
            return Err(Error::syntax("empty expression", l, c));
        }

        let tokens = lex_expr(&content).map_err(|e| Error::parse(e))?;
        let parser = ExprParser::new(tokens);
        let expr = parser.parse_all()?;
        nodes.push(Node::Expr(expr));
        Ok(())
    }

    fn parse_comment_tag(&mut self) -> Result<(), Error> {
        self.advance();
        self.advance();
        self.read_until("#}")?;
        self.advance();
        self.advance();
        Ok(())
    }

    fn parse_block_tag(&mut self, nodes: &mut NodeList) -> Result<(), Error> {
        self.advance();
        self.advance();
        self.skip_whitespace();

        let keyword = self.read_ident()?;
        self.skip_whitespace();

        match keyword.as_str() {
            "if" => {
                let node = self.parse_if()?;
                nodes.push(node);
            }
            "for" => {
                let node = self.parse_for()?;
                nodes.push(node);
            }
            "include" => {
                let name = self.read_string_literal()?;
                self.skip_whitespace();
                self.expect_tag_close('%')?;
                nodes.push(Node::Include(name));
            }
            "extends" => {
                if nodes.iter().any(|n| matches!(n, Node::Extends(_))) {
                    let (l, c) = self.current_position();
                    return Err(Error::syntax("multiple `{% extends %}` tags in one template", l, c));
                }
                let name = self.read_string_literal()?;
                self.skip_whitespace();
                self.expect_tag_close('%')?;
                nodes.push(Node::Extends(name));
            }
            "block" => {
                let name = self.read_ident()?;
                self.skip_whitespace();
                self.expect_tag_close('%')?;
                let body = self.parse_nodes(&["endblock"])?;
                self.consume_terminator_tag("endblock")?;
                nodes.push(Node::Block(BlockNode {
                    name,
                    body,
                }));
            }
            "set" => {
                let var_name = self.read_ident()?;
                self.skip_whitespace();
                if self.peek() != Some('=') {
                    let (l, c) = self.current_position();
                    return Err(Error::syntax("expected `=` after variable name", l, c));
                }
                self.advance();
                self.skip_whitespace();
                let (l, c) = self.current_position();
                let content = self.read_until("%}")?;
                self.expect_tag_close('%')?;
                if content.trim().is_empty() {
                    return Err(Error::syntax("empty expression", l, c));
                }
                let tokens = lex_expr(content.trim()).map_err(|e| Error::parse(e))?;
                let parser = ExprParser::new(tokens);
                let expr = parser.parse_all()?;
                nodes.push(Node::Set(var_name, expr));
            }
            "raw" => {
                self.skip_whitespace();
                self.expect_tag_close('%')?;
                let mut content = String::new();
                loop {
                    if self.eof() {
                        let (l, c) = self.current_position();
                        return Err(Error::syntax("unclosed `{% raw %}` block", l, c));
                    }
                    // Use memchr to find the next '{' candidate for {% endraw %}
                    let haystack = &self.source.as_bytes()[self.pos..];
                    match memchr(b'{', haystack) {
                        Some(offset) => {
                            // Bulk-copy everything before the potential endraw
                            if offset > 0 {
                                content.push_str(&self.source[self.pos..self.pos + offset]);
                                for &b in &haystack[..offset] {
                                    if b == b'\n' {
                                        self.line += 1;
                                        self.col = 1;
                                    } else {
                                        self.col += 1;
                                    }
                                }
                                self.pos += offset;
                            }

                            let bytes = self.source.as_bytes();
                            if self.pos + 1 < self.source.len() && bytes[self.pos + 1] == b'%' {
                                let saved_pos = self.pos;
                                let saved_line = self.line;
                                let saved_col = self.col;
                                self.advance(); // {
                                self.advance(); // %
                                self.skip_whitespace();
                                if let Ok(kw) = self.read_ident() {
                                    if kw == "endraw" {
                                        self.skip_whitespace();
                                        if self.peek() == Some('%') {
                                            self.advance();
                                            if self.peek() == Some('}') {
                                                self.advance();
                                                break;
                                            }
                                        }
                                    }
                                }
                                (self.pos, self.line, self.col) = (saved_pos, saved_line, saved_col);
                            }

                            // Not an endraw — add the bare '{' to content
                            content.push('{');
                            self.advance();
                        }
                        None => {
                            // No more '{' — all remaining is raw content
                            let rest = &self.source[self.pos..];
                            content.push_str(rest);
                            for &b in haystack {
                                if b == b'\n' {
                                    self.line += 1;
                                    self.col = 1;
                                } else {
                                    self.col += 1;
                                }
                            }
                            self.pos = self.source.len();
                            let (l, c) = self.current_position();
                            return Err(Error::syntax("unclosed `{% raw %}` block", l, c));
                        }
                    }
                }
                nodes.push(Node::RawBlock(content));
            }
            "elif" | "else" | "endif" | "endfor" | "endblock" => {
                let (l, c) = self.current_position();
                return Err(Error::syntax(
                    format!("unexpected `{keyword}` with no matching opening tag"),
                    l,
                    c,
                ));
            }
            other => {
                let (l, c) = self.current_position();
                return Err(Error::syntax(format!("unknown block tag `{other}`"), l, c));
            }
        }

        Ok(())
    }

    fn parse_if(&mut self) -> Result<Node, Error> {
        let (l, c) = self.current_position();
        let content = self.read_until("%}")?;
        self.expect_tag_close('%')?;
        if content.trim().is_empty() {
            return Err(Error::syntax("empty condition", l, c));
        }
        let condition = {
            let tokens = lex_expr(content.trim()).map_err(|e| Error::parse(e))?;
            let parser = ExprParser::new(tokens);
            parser.parse_all()?
        };

        let body = self.parse_nodes(&["elif", "else", "endif"])?;
        let mut elifs = Vec::new();
        let mut else_body = None;

        self.parse_elif_else_endif(&mut elifs, &mut else_body)?;

        Ok(Node::If(IfNode {
            condition,
            body,
            elifs,
            else_body,
        }))
    }

    fn parse_elif_else_endif(
        &mut self,
        elifs: &mut Vec<ElifNode>,
        else_body: &mut Option<NodeList>,
    ) -> Result<(), Error> {
        let mut else_seen = false;
        loop {
            self.advance(); // {
            self.advance(); // %
            self.skip_whitespace();
            let keyword = self.read_ident()?;
            self.skip_whitespace();

            match keyword.as_str() {
                "elif" => {
                    if else_seen {
                        let (l, c) = self.current_position();
                        return Err(Error::syntax("`elif` after `else`", l, c));
                    }
                    let (l, c) = self.current_position();
                    let cond_content = self.read_until("%}")?;
                    self.expect_tag_close('%')?;
                    if cond_content.trim().is_empty() {
                        return Err(Error::syntax("empty condition", l, c));
                    }
                    let tokens = lex_expr(cond_content.trim()).map_err(|e| Error::parse(e))?;
                    let parser = ExprParser::new(tokens);
                    let condition = parser.parse_all()?;
                    let body = self.parse_nodes(&["elif", "else", "endif"])?;
                    elifs.push(ElifNode {
                        condition,
                        body,
                    });
                }
                "else" => {
                    if else_seen {
                        let (l, c) = self.current_position();
                        return Err(Error::syntax("multiple `else` in `if` block", l, c));
                    }
                    else_seen = true;
                    self.expect_tag_close('%')?;
                    let body = self.parse_nodes(&["endif"])?;
                    *else_body = Some(body);
                }
                "endif" => {
                    self.expect_tag_close('%')?;
                    return Ok(());
                }
                other => {
                    let (l, c) = self.current_position();
                    return Err(Error::syntax(
                        format!("expected `elif`, `else`, or `endif`, got `{other}`"),
                        l,
                        c,
                    ));
                }
            }
        }
    }

    fn parse_for(&mut self) -> Result<Node, Error> {
        let var_name = self.read_ident()?;
        self.skip_whitespace();

        let kw = self.read_ident()?;
        if kw.as_str() != "in" {
            let (l, c) = self.current_position();
            return Err(Error::syntax(format!("expected `in`, got `{kw}`"), l, c));
        }
        self.skip_whitespace();

        let (l, c) = self.current_position();
        let iter_content = self.read_until("%}")?;
        self.expect_tag_close('%')?;
        if iter_content.trim().is_empty() {
            return Err(Error::syntax("empty iterable", l, c));
        }

        let tokens = lex_expr(iter_content.trim()).map_err(|e| Error::parse(e))?;
        let parser = ExprParser::new(tokens);
        let iterable = parser.parse_all()?;

        let body = self.parse_nodes(&["endfor"])?;

        self.consume_terminator_tag("endfor")?;

        Ok(Node::For(ForNode {
            var_name,
            iterable,
            body,
        }))
    }

    fn consume_terminator_tag(&mut self, expected: &str) -> Result<(), Error> {
        self.advance(); // {
        self.advance(); // %
        self.skip_whitespace();
        let kw = self.read_ident()?;
        if kw.as_str() != expected {
            let (l, c) = self.current_position();
            return Err(Error::syntax(format!("expected `{expected}`, got `{kw}`"), l, c));
        }
        self.skip_whitespace();
        self.expect_tag_close('%')?;
        Ok(())
    }

    fn read_until(&mut self, delimiter: &str) -> Result<String, Error> {
        let start_line = self.line;
        let start_col = self.col;
        let start_pos = self.pos;

        if let Some(idx) = self.source[self.pos..].find(delimiter) {
            let end_pos = self.pos + idx;
            let content = self.source[start_pos..end_pos].to_string();
            // Count newlines in the skipped region using bytes (avoids UTF-8 decoding)
            for &b in self.source.as_bytes()[start_pos..end_pos].iter() {
                if b == b'\n' {
                    self.line += 1;
                    self.col = 1;
                } else {
                    self.col += 1;
                }
            }
            self.pos = end_pos;
            return Ok(content);
        }

        // Not found, advance to end
        for &b in self.source.as_bytes()[self.pos..].iter() {
            if b == b'\n' {
                self.line += 1;
                self.col = 1;
            } else {
                self.col += 1;
            }
        }
        self.pos = self.source.len();

        Err(Error::syntax(
            format!("unclosed tag, expected `{delimiter}`"),
            start_line,
            start_col,
        ))
    }

    fn read_ident(&mut self) -> Result<String, Error> {
        let mut ident = String::new();
        let (l, c) = self.current_position();

        if let Some(ch) = self.peek() {
            if !ch.is_ascii_alphabetic() && ch != '_' {
                return Err(Error::syntax("expected identifier", l, c));
            }
            ident.push(self.advance().unwrap());
        } else {
            return Err(Error::syntax("expected identifier", l, c));
        }

        while let Some(ch) = self.peek() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                ident.push(self.advance().unwrap());
            } else {
                break;
            }
        }

        Ok(ident)
    }

    fn read_string_literal(&mut self) -> Result<String, Error> {
        let (l, c) = self.current_position();
        self.skip_whitespace();
        match self.peek() {
            Some('\'') | Some('"') => {
                let quote = self.advance().unwrap();
                let mut s = String::new();
                while let Some(ch) = self.peek() {
                    if ch == '\\' && self.pos + 1 < self.source.len() {
                        self.advance();
                        let escaped = self.advance().unwrap();
                        match escaped {
                            'n' => s.push('\n'),
                            't' => s.push('\t'),
                            'r' => s.push('\r'),
                            '\\' => s.push('\\'),
                            '\'' => s.push('\''),
                            '"' => s.push('"'),
                            c => {
                                s.push('\\');
                                s.push(c);
                            }
                        }
                    } else if ch == quote {
                        self.advance();
                        return Ok(s);
                    } else {
                        s.push(self.advance().unwrap());
                    }
                }
                Err(Error::syntax("unclosed string literal", l, c))
            }
            Some(_) => Err(Error::syntax("expected string literal", l, c)),
            None => Err(Error::syntax("expected string literal", l, c)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Expr;

    #[test]
    fn test_variable_output() {
        let nodes = TemplateParser::new("Hello, {{ name }}!").parse().unwrap();
        assert_eq!(nodes.len(), 3);
        match &nodes[0] {
            Node::Raw(s) => assert_eq!(s, "Hello, "),
            _ => panic!("expected Raw"),
        }
        match &nodes[1] {
            Node::Expr(_) => {}
            _ => panic!("expected Expr"),
        }
        match &nodes[2] {
            Node::Raw(s) => assert_eq!(s, "!"),
            _ => panic!("expected Raw"),
        }
    }

    #[test]
    fn test_if_block() {
        let nodes = TemplateParser::new("{% if active %}yes{% endif %}").parse().unwrap();
        assert_eq!(nodes.len(), 1);
        match &nodes[0] {
            Node::If(if_node) => {
                assert_eq!(if_node.body.len(), 1);
                match &if_node.body[0] {
                    Node::Raw(s) => assert_eq!(s, "yes"),
                    _ => panic!("expected Raw"),
                }
                assert!(if_node.else_body.is_none());
                assert!(if_node.elifs.is_empty());
            }
            _ => panic!("expected If"),
        }
    }

    #[test]
    fn test_if_else() {
        let nodes = TemplateParser::new("{% if show %}a{% else %}b{% endif %}")
            .parse()
            .unwrap();
        assert_eq!(nodes.len(), 1);
        match &nodes[0] {
            Node::If(if_node) => {
                assert_eq!(if_node.body.len(), 1);
                assert_eq!(if_node.else_body.as_ref().unwrap().len(), 1);
            }
            _ => panic!("expected If"),
        }
    }

    #[test]
    fn test_if_elif_else() {
        let nodes = TemplateParser::new("{% if a %}1{% elif b %}2{% elif c %}3{% else %}4{% endif %}")
            .parse()
            .unwrap();
        assert_eq!(nodes.len(), 1);
        match &nodes[0] {
            Node::If(if_node) => {
                assert_eq!(if_node.elifs.len(), 2);
                assert_eq!(if_node.else_body.as_ref().unwrap().len(), 1);
            }
            _ => panic!("expected If"),
        }
    }

    #[test]
    fn test_for_loop() {
        let nodes = TemplateParser::new("{% for item in items %}{{ item }}{% endfor %}")
            .parse()
            .unwrap();
        assert_eq!(nodes.len(), 1);
        match &nodes[0] {
            Node::For(for_node) => {
                assert_eq!(for_node.var_name, "item");
                assert_eq!(for_node.body.len(), 1);
            }
            _ => panic!("expected For"),
        }
    }

    #[test]
    fn test_include() {
        let nodes = TemplateParser::new("{% include \"header.html\" %}").parse().unwrap();
        assert_eq!(nodes.len(), 1);
        match &nodes[0] {
            Node::Include(name) => assert_eq!(name, "header.html"),
            _ => panic!("expected Include"),
        }
    }

    #[test]
    fn test_comment() {
        let nodes = TemplateParser::new("before{# comment #}after").parse().unwrap();
        assert_eq!(nodes.len(), 2);
        match &nodes[0] {
            Node::Raw(s) => assert_eq!(s, "before"),
            _ => panic!("expected Raw"),
        }
        match &nodes[1] {
            Node::Raw(s) => assert_eq!(s, "after"),
            _ => panic!("expected Raw"),
        }
    }

    #[test]
    fn test_extend_template() {
        let nodes = TemplateParser::new("{% extends \"base.html\" %}{% block body %}content{% endblock %}")
            .parse()
            .unwrap();
        assert_eq!(nodes.len(), 2);
        match &nodes[0] {
            Node::Extends(name) => assert_eq!(name, "base.html"),
            _ => panic!("expected Extends"),
        }
        match &nodes[1] {
            Node::Block(block) => {
                assert_eq!(block.name, "body");
                assert_eq!(block.body.len(), 1);
            }
            _ => panic!("expected Block"),
        }
    }

    #[test]
    fn test_raw_block() {
        let nodes = TemplateParser::new("{% raw %}{{ not processed }}{% endraw %}")
            .parse()
            .unwrap();
        assert_eq!(nodes.len(), 1);
        match &nodes[0] {
            Node::RawBlock(s) => assert_eq!(s, "{{ not processed }}"),
            _ => panic!("expected RawBlock"),
        }
    }

    #[test]
    fn test_dotted_access() {
        let nodes = TemplateParser::new("{{ user.email }}").parse().unwrap();
        match &nodes[0] {
            Node::Expr(expr) => match expr {
                Expr::Dot(left, name) => {
                    assert_eq!(name, "email");
                    match left.as_ref() {
                        Expr::Var(v) => assert_eq!(v, "user"),
                        _ => panic!("expected Var"),
                    }
                }
                _ => panic!("expected Dot"),
            },
            _ => panic!("expected Expr"),
        }
    }

    #[test]
    fn test_filter() {
        let nodes = TemplateParser::new("{{ name | upper }}").parse().unwrap();
        match &nodes[0] {
            Node::Expr(expr) => match expr {
                Expr::Filter {
                    name, ..
                } => {
                    assert_eq!(name, "upper");
                }
                _ => panic!("expected Filter"),
            },
            _ => panic!("expected Expr"),
        }
    }

    #[test]
    fn test_set_variable() {
        let nodes = TemplateParser::new("{% set x = 42 %}").parse().unwrap();
        assert_eq!(nodes.len(), 1);
        match &nodes[0] {
            Node::Set(name, _) => assert_eq!(name, "x"),
            _ => panic!("expected Set"),
        }
    }

    #[test]
    fn test_unclosed_expr_tag() {
        let result = TemplateParser::new("{{ name").parse();
        assert!(result.is_err());
    }

    #[test]
    fn test_unclosed_block_tag() {
        let result = TemplateParser::new("{% if true %}hello").parse();
        assert!(result.is_err());
    }

    #[test]
    fn test_unclosed_comment() {
        let result = TemplateParser::new("{# hello world").parse();
        assert!(result.is_err());
    }

    #[test]
    fn test_unclosed_raw_block() {
        let result = TemplateParser::new("{% raw %}hello world").parse();
        assert!(result.is_err());
    }

    #[test]
    fn test_non_ascii_identifiers() {
        let result = TemplateParser::new("{{ café }}").parse();
        assert!(result.is_err());
    }

    #[test]
    fn test_nested_blocks() {
        let nodes = TemplateParser::new("{% for i in items %}{% if i.active %}{{ i.name }}{% endif %}{% endfor %}")
            .parse()
            .unwrap();
        assert_eq!(nodes.len(), 1);
        match &nodes[0] {
            Node::For(_) => {}
            _ => panic!("expected For"),
        }
    }

    #[test]
    fn test_empty_template() {
        let nodes = TemplateParser::new("").parse().unwrap();
        assert!(nodes.is_empty());
    }

    #[test]
    fn test_whitespace_only() {
        let nodes = TemplateParser::new("   \n  \t  ").parse().unwrap();
        assert_eq!(nodes.len(), 1);
        match &nodes[0] {
            Node::Raw(s) => assert!(!s.is_empty()),
            _ => panic!("expected Raw"),
        }
    }

    #[test]
    fn test_multiple_extends_error() {
        let result = TemplateParser::new("{% extends \"a\" %}{% extends \"b\" %}").parse();
        assert!(result.is_err());
    }

    #[test]
    fn test_raw_block_with_inner_tag() {
        let nodes = TemplateParser::new("{% raw %}{% inner %}{% endraw %}").parse().unwrap();
        assert_eq!(nodes.len(), 1);
        match &nodes[0] {
            Node::RawBlock(s) => assert_eq!(s, "{% inner %}"),
            _ => panic!("expected RawBlock"),
        }
    }

    #[test]
    fn test_raw_block_preserves_trailing_whitespace() {
        let nodes = TemplateParser::new("{% raw %}hello   {% endraw %}").parse().unwrap();
        match &nodes[0] {
            Node::RawBlock(s) => assert_eq!(s, "hello   "),
            _ => panic!("expected RawBlock"),
        }
    }

    #[test]
    fn test_trailing_tokens_in_expr() {
        let result = TemplateParser::new("{{ true false }}").parse();
        assert!(result.is_err());
    }

    #[test]
    fn test_unknown_tag() {
        let result = TemplateParser::new("{% unknown %}").parse();
        assert!(result.is_err());
    }

    #[test]
    fn test_unclosed_block() {
        let result = TemplateParser::new("{% block foo %}hello").parse();
        assert!(result.is_err());
    }

    #[test]
    fn test_unclosed_for() {
        let result = TemplateParser::new("{% for i in items %}hello").parse();
        assert!(result.is_err());
    }

    #[test]
    fn test_unclosed_if() {
        let result = TemplateParser::new("{% if true %}hello").parse();
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_expr() {
        let result = TemplateParser::new("{{ }}").parse();
        assert!(result.is_err());
    }

    #[test]
    fn test_incomplete_expr() {
        let result = TemplateParser::new("{{ 1 + }}").parse();
        assert!(result.is_err());
    }

    #[test]
    fn test_missing_in_for() {
        let result = TemplateParser::new("{% for item items %}hello{% endfor %}").parse();
        assert!(result.is_err());
    }

    #[test]
    fn test_missing_iterable_in_for() {
        let result = TemplateParser::new("{% for item in %}hello{% endfor %}").parse();
        assert!(result.is_err());
    }

    #[test]
    fn test_duplicate_block_names() {
        let nodes = TemplateParser::new("{% block foo %}a{% endblock %}{% block foo %}b{% endblock %}")
            .parse()
            .unwrap();
        assert_eq!(nodes.len(), 2);
        match &nodes[0] {
            Node::Block(b) => assert_eq!(b.name, "foo"),
            _ => panic!("expected Block"),
        }
        match &nodes[1] {
            Node::Block(b) => assert_eq!(b.name, "foo"),
            _ => panic!("expected Block"),
        }
    }

    // New edge-case tests

    #[test]
    fn test_empty_if_condition() {
        let result = TemplateParser::new("{% if %}a{% endif %}").parse();
        assert!(result.is_err());
    }

    #[test]
    fn test_elif_after_else() {
        let result = TemplateParser::new("{% if a %}b{% else %}c{% elif d %}e{% endif %}").parse();
        assert!(result.is_err());
    }

    #[test]
    fn test_multiple_else() {
        let result = TemplateParser::new("{% if a %}b{% else %}c{% else %}d{% endif %}").parse();
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_set_expr() {
        let result = TemplateParser::new("{% set x = %}").parse();
        assert!(result.is_err());
    }

    #[test]
    fn test_negative_index_compile_time() {
        let result = TemplateParser::new("{{ items[-1] }}").parse();
        assert!(result.is_err());
    }

    #[test]
    fn test_negative_index_unary() {
        let result = TemplateParser::new("{{ items[-(1)] }}").parse();
        assert!(result.is_err());
    }

    #[test]
    fn test_negative_float_index() {
        let result = TemplateParser::new("{{ items[-1.5] }}").parse();
        assert!(result.is_err());
    }
}

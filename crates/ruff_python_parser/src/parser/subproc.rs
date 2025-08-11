use std::vec;

use ruff_python_ast::name::Name;
use ruff_python_ast::{
    self as ast, AtomicNodeIndex, DictItem, Expr, ExprContext, ExprDict, ExprTuple,
};
use ruff_text_size::{Ranged, TextRange, TextSize};

use crate::ParseErrorType;
use crate::builders::ExprWrap;
use crate::token::TokenFlags;

use crate::{
    parser::{Parser, ParserProgress},
    token::TokenKind,
};

impl Parser<'_> {
    /// Parses a subprocess expression.
    /// This includes various forms of subprocess capture like `$(...)`, `$[...]`, `!(...)`, and `![...]`.
    pub(super) fn parse_subprocs(&mut self, method: impl Into<Name>, closing: TokenKind) -> Expr {
        let start = self.node_start();

        let mut cmd = self
            .xonsh_attr("cmd")
            .call(self.parse_cmd_group(closing), self.node_range(start));
        while self.at(TokenKind::Vbar) {
            let pipe_start = self.node_start();
            self.bump_any();
            cmd = cmd
                .attr("pipe", self.node_range(pipe_start))
                .call(self.parse_cmd_group(closing), self.node_range(pipe_start));
        }
        cmd.attr(method, self.node_range(start))
            .call_empty(self.node_range(start))
            .into()
    }

    /// Parses a subprocess expression like `ls tmp-dir` with ![]
    pub(super) fn parse_bare_proc(&mut self) -> ast::Stmt {
        let start = self.node_start();
        let expr = self.parse_subprocs("hide", TokenKind::Newline);
        ast::Stmt::Expr(ast::StmtExpr {
            range: self.node_range(start),
            value: Box::new(expr),
            node_index: AtomicNodeIndex::dummy(),
        })
    }

    fn parse_cmd_group(&mut self, closing: TokenKind) -> ast::Arguments {
        const REDIR_NAMES: &[&str] = &["o", "out", "e", "err", "a", "all"];
        let start = self.node_start();
        let mut cmds = Vec::new();
        let mut keywords = Vec::new();
        let mut redirects = Vec::new();
        let mut progress = ParserProgress::default();

        loop {
            match self.current_token_kind() {
                tk if tk == closing => {
                    self.bump_any();
                    break;
                }
                TokenKind::Vbar => break,
                TokenKind::Int | TokenKind::Amper if matches!(self.peek(), TokenKind::Greater) => {
                    let result = self.parse_redirection1(closing);
                    redirects.push(result);
                }
                TokenKind::Name
                    if matches!(self.peek(), TokenKind::Greater)
                        && REDIR_NAMES.contains(&&self.source[self.current_token_range()]) =>
                {
                    let result = self.parse_redirection1(closing);
                    redirects.push(result);
                }
                TokenKind::RightShift | TokenKind::Greater | TokenKind::Less => {
                    let result = self.parse_redirection(None, closing);
                    redirects.push(result);
                }
                TokenKind::Amper if self.peek() == closing => {
                    keywords.push(ast::Keyword {
                        arg: Some(self.to_identifier("bg")),
                        value: self.literal_true(),
                        range: self.current_token_range(),
                        node_index: AtomicNodeIndex::dummy(),
                    });
                    self.bump_any(); // skip `&`
                    self.bump(closing); // skip `)`
                    break;
                }
                _ => cmds.push(self.parse_proc_arg(&mut progress, closing)),
            }
        }

        if let Some(first) = redirects.first() {
            if let Some(last) = redirects.last() {
                let range = TextRange::new(first.range().start(), last.range().end());
                let expr = Expr::from(ExprDict {
                    range,
                    items: redirects,
                    node_index: AtomicNodeIndex::dummy(),
                });
                keywords.push(ast::Keyword {
                    arg: Some(self.to_identifier("redirects")),
                    value: expr,
                    range: self.node_range(start),
                    node_index: AtomicNodeIndex::dummy(),
                });
            }
        }

        ast::Arguments {
            range: self.node_range(start),
            args: cmds.into_boxed_slice(),
            keywords: keywords.into_boxed_slice(),
            node_index: AtomicNodeIndex::dummy(),
        }
    }

    /// Parses arguments in a subprocess expression.
    fn parse_proc_arg(&mut self, progress: &mut ParserProgress, closing: TokenKind) -> Expr {
        progress.assert_progressing(self);
        let kind = self.current_token_kind();

        match kind {
            TokenKind::At => self.parse_decorator_or_interpolation(),
            tk if tk.is_macro() => self.parse_proc_macro(closing),
            TokenKind::String
            | TokenKind::FStringStart
            | TokenKind::TStringStart
            | TokenKind::Lpar
            | TokenKind::Dollar
            | TokenKind::DollarLParen
            | TokenKind::AtDollarLParen => self.parse_atom().expr,
            tk if tk.is_proc_op() => {
                let range = self.current_token_range();
                self.bump_any();
                self.to_string_literal(range)
            }
            _ => self.parse_proc_single(closing),
        }
    }
    fn parse_proc_single(&mut self, closing: TokenKind) -> Expr {
//         dbg!(&self.current_token_kind(), &self.current_token_range());
        let start = self.node_start();
        let mut offset = self.node_end();
        let mut nesting = 0;
        self.bump_any();

        while !matches!(self.current_token_kind(), tk if
            tk.is_proc_op() ||
            offset != self.node_start() ||
            (tk == closing && nesting == 0)
        ) {
            if self.current_token_kind() == TokenKind::Lpar {
                nesting += 1;
            }
            offset = self.node_end();
            self.bump_any();
        }

        self.to_string_literal(TextRange::new(start, offset))
    }
    fn parse_redirection1(&mut self, closing: TokenKind) -> DictItem {
        let start = self.node_start();
        self.bump_any(); // skip the name or number
        self.bump_any(); // skip the `>`
        let range = self.node_range(start);
        self.parse_redirection(Some(range), closing)
    }
    fn parse_redirection(&mut self, key_range: Option<TextRange>, closing: TokenKind) -> DictItem {
        let range = if let Some(key_range) = key_range {
            key_range
        } else {
            let range = self.current_token_range();
            self.bump_any();
            range
        };

        let key = Some(self.to_string_literal(range));
        let value = self.parse_proc_single(closing);

        DictItem { key, value }
    }
    pub(super) fn parse_decorator_or_interpolation(&mut self) -> Expr {
        self.bump_any(); // skip the `@`
        match self.current_token_kind() {
            TokenKind::Lpar => {
                let expr = self.parse_atom().expr;
                let range = expr.range();
                self.xonsh_attr("list_of_strs_or_callables")
                    .call0(vec![expr], range)
                    .into()
            }
            TokenKind::Name if self.peek() == TokenKind::String => {
                let start = self.node_start();
                let name = Expr::from(self.parse_name());
                let string = self.parse_strings();
                let range = self.node_range(start);
                self.xonsh_attr("Pattern")
                    .call0(vec![string], range)
                    .attr("invoke", range)
                    .call0(vec![name], range)
                    .star(self.node_range(self.node_start()))
            }
            _ => unreachable!("Expected to parse a name and a string"),
        }
    }
    /// consume any tokens until the closing token or `is_macro_end` and strip whitespace
    pub(super) fn parse_proc_macro(&mut self, closing: TokenKind) -> Expr {
        self.bump_any(); // skip the `!`
        let start = self.node_start();
        let end = if self.at(closing) {
            start
        } else {
            self.take_while(|t, _| !t.is_macro_end(), closing)
        };

        let range = TextRange::new(start, end);

        self.to_string_literal(range)
    }

    #[inline]
    fn take_while(
        &mut self,
        mut f: impl FnMut(TokenKind, i32) -> bool,
        closing: TokenKind,
    ) -> TextSize {
        let mut nesting = 0;
        let mut range = self.current_token_range();
        let is_opening = match closing {
            TokenKind::Rsqb => TokenKind::is_open_square,
            _ => TokenKind::is_open_paren,
        };

        while f(self.current_token_kind(), nesting) {
            if is_opening(&self.current_token_kind()) {
                nesting += 1;
            }
            if self.current_token_kind() == closing {
                if nesting == 0 {
                    break;
                }
                nesting -= 1;
            }
            range = self.current_token_range();
            self.bump_any();
        }
        range.end()
    }

    /// Creates a xonsh attribute expression.
    fn xonsh_attr(&mut self, name: impl Into<Name>) -> ExprWrap {
        self.expr_name("__xonsh__")
            .attr(name, self.current_token_range())
    }
    fn to_identifier(&self, name: impl Into<Name>) -> ast::Identifier {
        ExprWrap::identifier(name, self.current_token_range())
    }
    fn expr_name(&self, name: impl AsRef<str>) -> ExprWrap {
        let val = ast::ExprName {
            range: self.current_token_range(),
            id: Name::new(name),
            ctx: ExprContext::Load,
            node_index: AtomicNodeIndex::dummy(),
        };
        ExprWrap(Expr::Name(val))
    }
    fn to_string_literal(&self, range: TextRange) -> Expr {
        let value = self.source[range].to_string();
        string_literal(range, value)
    }

    pub(super) fn parse_env_name(&mut self) -> Expr {
        self.bump_any();
        let attr = self.xonsh_attr("env");
        let start = self.node_start();
        let slice = if self.at(TokenKind::Name) {
            let range = self.current_token_range();
            self.bump_any();
            self.to_string_literal(range)
        } else {
            self.add_error(
                ParseErrorType::OtherError(format!(
                    "Expected an Environment variable name, got {}",
                    self.current_token_kind()
                )),
                self.current_token_range(),
            );
            self.expr_name("Invalid").into()
        };
        let ast = ast::ExprSubscript {
            value: Box::new(attr.into()),
            slice: Box::new(slice),
            ctx: ExprContext::Load,
            range: self.node_range(start),
            node_index: AtomicNodeIndex::dummy(),
        };
        Expr::Subscript(ast)
    }
    pub(super) fn parse_env_expr(&mut self) -> Expr {
        self.bump(TokenKind::DollarLBrace);
        let attr = self.xonsh_attr("env");

        // Slice range doesn't include the `[` token.
        let slice_start = self.node_start();

        let slice: Expr = if self.eat(TokenKind::Rbrace) {
            // Create an error when receiving an empty slice to parse, e.g. `x[]`
            let slice_range = self.node_range(slice_start);
            self.add_error(
                ParseErrorType::OtherError(format!(
                    "Expected an Environment variable name or expression, got {}",
                    self.current_token_kind()
                )),
                slice_range,
            );
            self.expr_name("Invalid").into()
        } else {
            self.parse_slice(TokenKind::Rbrace)
        };

        self.bump(TokenKind::Rbrace);

        let ast = ast::ExprSubscript {
            value: Box::new(attr.into()),
            slice: Box::new(slice),
            ctx: ExprContext::Load,
            range: self.node_range(slice_start),
            node_index: AtomicNodeIndex::dummy(),
        };
        Expr::Subscript(ast)
    }
    pub(super) fn parse_special_strings(
        &mut self,
        expr: Expr,
        start: TextSize,
        flags: TokenFlags,
    ) -> Expr {
        if flags.intersects(TokenFlags::PATH_STRING) {
            return self
                .xonsh_attr("path_literal")
                .call0(vec![expr], self.node_range(start))
                .into();
        } else if flags.intersects(TokenFlags::GLOB_STRING) {
            return self
                .xonsh_attr("Pattern")
                .call0(vec![expr], self.node_range(start))
                .attr("glob", self.node_range(start))
                .call_empty(self.node_range(start))
                .into();
        } else if flags.intersects(TokenFlags::BACKTICK_STRING) {
            return self
                .xonsh_attr("Pattern")
                .call0(vec![expr], self.node_range(start))
                .attr("regex", self.node_range(start))
                .call_empty(self.node_range(start))
                .into();
        }

        expr
    }

    fn literal_true(&self) -> Expr {
        Expr::BooleanLiteral(ast::ExprBooleanLiteral {
            value: true,
            range: self.node_range(self.node_start()),
            node_index: AtomicNodeIndex::dummy(),
        })
    }

    pub(super) fn parse_help_expr(&mut self, lhs: Expr, start: TextSize) -> Expr {
        self.bump_any();

        let range = self.node_range(start);
        let method = if self.at(TokenKind::Question) {
            self.bump_any();
            "superhelp"
        } else {
            "help"
        };
        let args = vec![lhs];
        self.xonsh_attr(method).call0(args, range).into()
    }
    pub(super) fn parse_call_macro(&mut self, lhs: Expr, start: TextSize) -> Expr {
        self.bump(TokenKind::BangLParen);
        let closing = TokenKind::Rpar;
        let mut progress = ParserProgress::default();

        let mut inner_args = vec![];
        while !self.at(closing) {
            inner_args.push(self.parse_call_macro_arg(closing));
            progress.assert_progressing(self);
        }
        let range = self.node_range(start);
        let args = vec![
            lhs,
            Expr::from(ExprTuple {
                elts: inner_args,
                ctx: ExprContext::Load,
                range,
                parenthesized: false,
                node_index: AtomicNodeIndex::dummy(),
            }),
            self.expr_name("globals").call_empty(range).into(),
            self.expr_name("locals").call_empty(range).into(),
        ];
        self.bump(closing);
        self.xonsh_attr("call_macro").call0(args, range).into()
    }
    #[inline]
    fn parse_call_macro_arg(&mut self, closing: TokenKind) -> Expr {
        let start = self.node_start();
        let end = {
            let mut nesting = vec![];
            let mut end = self.current_token_range().end();

            while !nesting.is_empty() || self.current_token_kind() != closing {
                let tk = self.current_token_kind();
                if let Some(inner) = tk.get_closer() {
                    nesting.push(inner);
                } else if let Some(last) = nesting.last() {
                    if last == &tk {
                        nesting.pop();
                    }
                } else if tk == closing || tk == TokenKind::Comma {
                    break;
                }

                end = self.current_token_range().end();
                self.bump_any();
            }
            end
        };
        if self.at(TokenKind::Comma) {
            self.bump(TokenKind::Comma);
        }
        let range = TextRange::new(start, end);
        self.to_string_literal(range)
    }
    pub(super) fn parse_with_macro(
        &mut self,
        items: Vec<ast::WithItem>,
        start: TextSize,
    ) -> ast::StmtWith {
        if self.at(TokenKind::Newline) {
            self.bump_any();
        }
        let suit_start = self.node_start();
        let min_indent = if self.at(TokenKind::Indent) {
            let length = self.current_token_range().len().to_usize();
            self.bump_any();
            length
        } else {
            usize::MIN
        };

        {
            // loop until dedent
            let mut indent_level = 0;

            loop {
                if self.at(TokenKind::EndOfFile) {
                    break;
                }
                if self.at(TokenKind::Dedent) && indent_level < 1 {
                    break;
                }
                let tk = self.current_token_kind();

                if tk == TokenKind::Indent {
                    indent_level += 1;
                } else if indent_level > 0 && tk == TokenKind::Dedent {
                    indent_level -= 1;
                }
                self.bump_any();
            }
        }
        let range = self.node_range(start);
        let body = {
            let pass = ast::StmtPass {
                range,
                node_index: AtomicNodeIndex::dummy(),
            };
            vec![ast::Stmt::from(pass)]
        };
        let items = {
            let suite = {
                let range = self.node_range(suit_start);
                let string = dedent(&self.source[range], min_indent);
                string_literal(range, string)
            };
            let enter_macro = self.xonsh_attr("enter_macro");
            items
                .into_iter()
                .map(|item| {
                    let expr = item.context_expr;
                    let args = vec![
                        expr,
                        suite.clone(),
                        self.expr_name("globals").call_empty(range).into(),
                        self.expr_name("locals").call_empty(range).into(),
                    ];

                    ast::WithItem {
                        context_expr: enter_macro.clone().call0(args, range).into(),
                        optional_vars: item.optional_vars,
                        range,
                        node_index: AtomicNodeIndex::dummy(),
                    }
                })
                .collect()
        };
        if self.at(TokenKind::Dedent) {
            self.bump_any();
        }
        ast::StmtWith {
            items,
            body,
            is_async: false,
            range,
            node_index: AtomicNodeIndex::dummy(),
        }
    }
}

fn dedent(input: &str, min_indent: usize) -> String {
    let lines: Vec<&str> = input.lines().collect();

    lines
        .iter()
        .map(|line| {
            if line.trim().is_empty() {
                (*line).to_string() // Preserve empty lines
            } else {
                line.chars().skip(min_indent).collect()
            }
        })
        .collect::<Vec<String>>()
        .join("\n")
}

fn string_literal(range: TextRange, value: String) -> Expr {
    let literal = ast::StringLiteral {
        value: value.into_boxed_str(),
        range,
        flags: ast::StringLiteralFlags::empty(),
        node_index: AtomicNodeIndex::dummy(),
    };

    Expr::from(ast::ExprStringLiteral {
        value: ast::StringLiteralValue::single(literal),
        range,
        node_index: AtomicNodeIndex::dummy(),
    })
}

use ruff_python_ast::name::Name;
use ruff_python_ast::{self as ast, Expr, ExprContext};
use ruff_text_size::{TextRange, TextSize};

use crate::ParseErrorType;

use crate::parser::expression::ExpressionContext;
use crate::{
    parser::{Parser, ParserProgress},
    token::TokenKind,
};

impl Parser<'_> {
    /// Parses a subprocess expression.
    /// This includes various forms of subprocess capture like `$(...)`, `$[...]`, `!(...)`, and `![...]`.
    pub(super) fn parse_subprocs(&mut self, method: impl Into<Name>, closing: TokenKind) -> Expr {
        let start = self.node_start();
        self.bump_any(); // skip the `$(`

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
    }

    fn parse_cmd_group(&mut self, closing: TokenKind) -> ast::Arguments {
        let start = self.node_start();
        let mut cmds = Vec::new();
        let mut keywords = Vec::new();
        let mut progress = ParserProgress::default();

        loop {
            match self.current_token_kind() {
                tk if tk == closing => {
                    self.bump_any();
                    break;
                }
                TokenKind::Vbar => break,
                TokenKind::Amper if self.peek() == closing => {
                    keywords.push(ast::Keyword {
                        arg: Some(self.to_identifier("bg")),
                        value: self.literal_true(),
                        range: self.current_token_range(),
                    });
                    self.bump_any(); // skip `&`
                    self.bump(closing); // skip `)`
                    break;
                }
                _ => cmds.push(self.parse_proc_arg(&mut progress, closing)),
            }
        }

        ast::Arguments {
            range: self.node_range(start),
            args: cmds.into_boxed_slice(),
            keywords: keywords.into_boxed_slice(),
        }
    }

    /// Parses arguments in a subprocess expression.
    fn parse_proc_arg(&mut self, progress: &mut ParserProgress, closing: TokenKind) -> Expr {
        progress.assert_progressing(self);
        let kind = self.current_token_kind();

        match kind {
            TokenKind::At => self
                .parse_decorator_or_interpolation()
                .star(self.node_range(self.node_start())),
            tk if tk.is_macro() => self.parse_proc_macro(closing),
            tk if tk.is_proc_atom()
                || matches!(tk, TokenKind::String | TokenKind::FStringStart) =>
            {
                self.parse_atom().expr
            }
            tk if tk.is_proc_op() => {
                let range = self.current_token_range();
                self.bump_any();
                self.to_string_literal(range)
            }
            _ => self.parse_proc_single(closing),
        }
    }
    fn parse_proc_single(&mut self, closing: TokenKind) -> Expr {
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
    pub(super) fn parse_decorator_or_interpolation(&mut self) -> Expr {
        self.bump_any(); // skip the `@`
        let pattern = self.xonsh_attr("Pattern");
        if !self.at(TokenKind::Name) || self.peek() != TokenKind::String {
            unreachable!("Expected to parse a name and a string");
        }
        let start = self.node_start();
        let name = Expr::from(self.parse_name());
        let string = self.parse_strings();
        let range = self.node_range(start);
        pattern
            .call0(vec![string], range)
            .attr("invoke", range)
            .call0(vec![name], range)
    }
    pub(super) fn parse_proc_pyexpr(&mut self) -> Expr {
        self.bump_any();

        let start = self.node_start();
        let name = self.xonsh_attr("list_of_strs_or_callables");
        let expr = self.parse_conditional_expression_or_higher_impl(ExpressionContext::default());
        let range = self.node_range(start);
        let expr = name.call0(vec![expr.expr], range);
        self.bump(TokenKind::Rpar);
        expr
    }
    /// consume any tokens until the closing token or `is_macro_end` and strip whitespace
    pub(super) fn parse_proc_macro(&mut self, closing: TokenKind) -> Expr {
        self.bump_any(); // skip the `!`
        let start = self.node_start();
        let end = if self.at(closing) {
            start
        } else {
            self.take_while(|t| !t.is_macro_end(), closing)
        };

        let range = TextRange::new(start, end);

        self.to_string_literal(range)
    }

    fn take_while(&mut self, mut f: impl FnMut(TokenKind) -> bool, closing: TokenKind) -> TextSize {
        let mut nesting = 0;
        let mut range = self.current_token_range();
        let is_opening = match closing {
            TokenKind::Rpar => TokenKind::is_open_paren,
            TokenKind::Rsqb => TokenKind::is_open_square,
            _ => unreachable!(),
        };

        while f(self.current_token_kind()) {
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
    fn xonsh_attr(&mut self, name: impl Into<Name>) -> Expr {
        let xonsh = self.expr_name("__xonsh__");
        xonsh.attr(name, self.current_token_range())
    }
    fn to_identifier(&self, name: impl Into<Name>) -> ast::Identifier {
        Expr::identifier(name, self.current_token_range())
    }
    fn expr_name(&self, name: impl AsRef<str>) -> Expr {
        let val = ast::ExprName {
            range: self.current_token_range(),
            id: Name::new(name),
            ctx: ExprContext::Load,
        };
        Expr::Name(val)
    }

    fn to_string_literal(&self, range: TextRange) -> Expr {
        let value = self.source[range].to_string();
        let literal = ast::StringLiteral {
            value: value.into_boxed_str(),
            range,
            flags: ast::StringLiteralFlags::default(),
        };

        Expr::from(ast::ExprStringLiteral {
            value: ast::StringLiteralValue::single(literal),
            range,
        })
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
            self.expr_name("Invalid")
        };
        let ast = ast::ExprSubscript {
            value: Box::new(attr),
            slice: Box::new(slice),
            ctx: ExprContext::Load,
            range: self.node_range(start),
        };
        Expr::Subscript(ast)
    }
    pub(super) fn parse_env_expr(&mut self) -> Expr {
        self.bump(TokenKind::DollarLBrace);
        let attr = self.xonsh_attr("env");

        // Slice range doesn't include the `[` token.
        let slice_start = self.node_start();

        let slice = if self.eat(TokenKind::Rbrace) {
            // Create an error when receiving an empty slice to parse, e.g. `x[]`
            let slice_range = self.node_range(slice_start);
            self.add_error(
                ParseErrorType::OtherError(format!(
                    "Expected an Environment variable name or expression, got {}",
                    self.current_token_kind()
                )),
                slice_range,
            );
            self.expr_name("Invalid")
        } else {
            self.parse_slice(TokenKind::Rbrace)
        };

        self.bump(TokenKind::Rbrace);

        let ast = ast::ExprSubscript {
            value: Box::new(attr),
            slice: Box::new(slice),
            ctx: ExprContext::Load,
            range: self.node_range(slice_start),
        };
        Expr::Subscript(ast)
    }
    pub(super) fn parse_special_strings(&mut self, expr: Expr, start: TextSize) -> Expr {
        if let Expr::StringLiteral(s) = &expr {
            if s.value.is_path() {
                return self
                    .xonsh_attr("path_literal")
                    .call0(vec![expr], self.node_range(start));
            } else if s.value.is_regex() {
                return self
                    .xonsh_attr("Pattern")
                    .call0(vec![expr], self.node_range(start))
                    .attr("regex", self.node_range(start))
                    .call_empty(self.node_range(start));
            } else if s.value.is_glob() {
                return self
                    .xonsh_attr("Pattern")
                    .call0(vec![expr], self.node_range(start))
                    .attr("glob", self.node_range(start))
                    .call_empty(self.node_range(start));
            }
        }
        expr
    }

    fn literal_true(&self) -> Expr {
        Expr::BooleanLiteral(ast::ExprBooleanLiteral {
            value: true,
            range: self.node_range(self.node_start()),
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
        self.xonsh_attr(method).call0(args, range)
    }
}

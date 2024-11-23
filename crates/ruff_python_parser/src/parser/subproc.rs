use ruff_python_ast::name::Name;
use ruff_python_ast::{self as ast, Expr, ExprContext};
use ruff_text_size::{TextRange, TextSize};

use crate::ParseErrorType;

use crate::{
    parser::{Parser, ParserProgress},
    token::TokenKind,
};

impl<'a> Parser<'a> {
    /// Parses a subprocess expression.
    /// This includes various forms of subprocess capture like `$(...)`, `$[...]`, `!(...)`, and `![...]`.
    pub(crate) fn parse_subprocs(&mut self, func: impl AsRef<str>) -> Expr {
        let start = self.node_start();
        let attr = self.xonsh_attr(func);
        let mut progress = ParserProgress::default();

        if self.current_token_kind() == TokenKind::Lpar {
            self.bump_any();
        } else {
            return attr;
        }

        let group = std::iter::from_fn(|| self.parse_proc_args(&mut progress)).collect::<Vec<_>>();
        let args = vec![Expr::List(ast::ExprList {
            elts: group,
            ctx: ExprContext::Load,
            range: self.node_range(start),
        })];
        let arguments = ast::Arguments {
            range: self.node_range(start),
            args: args.into_boxed_slice(),
            keywords: vec![].into_boxed_slice(),
        };
        Expr::Call(ast::ExprCall {
            func: Box::new(attr),
            arguments,
            range: self.node_range(start),
        })
    }

    /// Parses arguments in a subprocess expression.
    pub(super) fn parse_proc_args(&mut self, parser_progress: &mut ParserProgress) -> Option<Expr> {
        parser_progress.assert_progressing(self);
        match self.current_token_kind() {
            TokenKind::Rpar => {
                self.bump_any();
                None
            }
            kind => {
                if kind.is_proc_atom()
                    || matches!(kind, TokenKind::String | TokenKind::FStringStart)
                {
                    return Some(self.parse_atom().expr);
                }

                // no need to check next tokens
                if kind.is_proc_op() {
                    let range = self.current_token_range();
                    self.bump_any();
                    return Some(self.to_string_literal(range));
                }

                // current range
                let start = self.node_start();
                let mut offset = self.node_end();
                let mut nesting = 0 as usize;
                self.bump_any(); // move cursor to next token

                // check to see if we need concat next tokens
                loop {
                    if self.current_token_kind().is_proc_op()
                        || (offset != self.node_start())
                        || (self.current_token_kind() == TokenKind::Rpar && nesting == 0)
                    {
                        break;
                    }
                    if self.current_token_kind() == TokenKind::Lpar {
                        nesting += 1;
                    }
                    offset = self.node_end();
                    self.bump_any();
                }

                let range = TextRange::new(start, offset);
                Some(self.to_string_literal(range))
            }
        }
    }

    /// Creates a xonsh attribute expression.
    pub(crate) fn xonsh_attr(&mut self, name: impl AsRef<str>) -> Expr {
        self.bump_value(self.current_token_kind());
        let xonsh = self.expr_name("__xonsh__");
        let name = ast::ExprAttribute {
            range: self.current_token_range(),
            attr: ast::Identifier {
                id: Name::new(name),
                range: self.current_token_range(),
            },
            value: Box::new(xonsh),
            ctx: ExprContext::Load,
        };

        Expr::Attribute(name)
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

    pub(crate) fn parse_env_name(&mut self) -> Expr {
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
    pub(crate) fn parse_env_expr(&mut self) -> Expr {
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
    fn wrap_string(&mut self, attr: &str, string: Expr, start: TextSize) -> Expr {
        let func = self.xonsh_attr(attr);

        let args = vec![string];
        let arguments = ast::Arguments {
            range: self.node_range(start),
            args: args.into_boxed_slice(),
            keywords: vec![].into_boxed_slice(),
        };
        Expr::Call(ast::ExprCall {
            func: Box::new(func),
            arguments,
            range: self.node_range(start),
        })
    }
    pub(crate) fn parse_special_strings(&mut self, expr: Expr, start: TextSize) -> Expr {
        match &expr {
            Expr::StringLiteral(s) => {
                if s.value.is_path() {
                    return self.wrap_string("path_literal", expr, start);
                } else if s.value.is_regex() {
                    return self.wrap_string("regex_literal", expr, start);
                } else if s.value.is_glob() {
                    return self.wrap_string("glob_literal", expr, start);
                }
            }
            _ => (),
        }
        expr
    }

    pub(crate) fn dollar_rule_atom(&mut self) -> Expr {
        let start = self.node_start();

        // Skip the '$' token
        self.bump_any();

        match self.current_token_kind() {
            TokenKind::Lpar => {
                // Handle $(...)
                self.parse_subprocs("__xonsh_subproc_captured_stdout__")
            }
            TokenKind::Lsqb => {
                // Handle $[...]
                self.parse_subprocs("__xonsh_subproc_uncaptured__")
            }
            TokenKind::Name => {
                // Handle $NAME environment variable
                self.parse_env_name()
            }
            TokenKind::Lbrace => {
                // Skip the '{' token
                self.bump_any();
                // Handle ${...} environment expression
                self.parse_env_expr()
            }
            _ => {
                // Error case - unexpected token after $
                self.add_error(
                    ParseErrorType::OtherError(format!(
                        "Expected '(', '[', '{{' or NAME after '$', got {}",
                        self.current_token_kind()
                    )),
                    self.current_token_range(),
                );
                // Return a placeholder invalid expression
                self.expr_name("Invalid")
            }
        }
    }
}

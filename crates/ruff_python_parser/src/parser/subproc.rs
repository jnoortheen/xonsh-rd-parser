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
    pub(super) fn parse_subprocs(&mut self, method: impl AsRef<str>) -> Expr {
        let start = self.node_start();
        let closing = TokenKind::Rpar;
        self.bump_any(); // skip the `$`
        self.bump(TokenKind::Lpar); // skip the `(`

        if self.current_token_kind() == closing {
            self.bump_any();
        }

        let group_call = self.parse_cmd_group(closing);
        let attr = self.to_attr(group_call, method);
        let arguments = ast::Arguments {
            range: self.node_range(start),
            args: vec![].into_boxed_slice(),
            keywords: vec![].into_boxed_slice(),
        };
        Expr::Call(ast::ExprCall {
            func: Box::new(attr),
            arguments,
            range: self.node_range(start),
        })
    }
    fn parse_cmd_group(&mut self, closing: TokenKind) -> Expr {
        let start = self.node_start();
        let mut progress = ParserProgress::default();
        let mut cmds = Vec::new();
        let mut keywords = Vec::new();

        loop {
            if self.at(TokenKind::Rpar) {
                self.bump_any();
                break;
            }
            if self.at(TokenKind::Amper) && self.peek() == closing {
                keywords.push(ast::Keyword {
                    arg: Some(self.to_identifier("bg")),
                    value: self.literal_true(),
                    range: self.current_token_range(),
                });
                self.bump_any(); // skip the `&`
                self.bump(closing); // skip the `)`
                break;
            }
            cmds.push(self.parse_proc_arg(&mut progress));
        }
        let arguments = ast::Arguments {
            range: self.node_range(start),
            args: cmds.into_boxed_slice(),
            keywords: keywords.into_boxed_slice(),
        };
        let attr = self.xonsh_attr("cmd", false);
        Expr::Call(ast::ExprCall {
            func: Box::new(attr),
            arguments,
            range: self.node_range(start),
        })
    }

    /// Parses arguments in a subprocess expression.
    fn parse_proc_arg(&mut self, parser_progress: &mut ParserProgress) -> Expr {
        parser_progress.assert_progressing(self);
        match self.current_token_kind() {
            kind => {
                if kind.is_proc_atom()
                    || matches!(kind, TokenKind::String | TokenKind::FStringStart)
                {
                    return self.parse_atom().expr;
                }

                // no need to check next tokens
                if kind.is_proc_op() {
                    let range = self.current_token_range();
                    self.bump_any();
                    return self.to_string_literal(range);
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
                self.to_string_literal(range)
            }
        }
    }

    /// Creates a xonsh attribute expression.
    fn xonsh_attr(&mut self, name: impl AsRef<str>, advance: bool) -> Expr {
        if advance {
            self.bump_any();
        }
        let xonsh = self.expr_name("__xonsh__");
        self.to_attr(xonsh, name)
    }
    fn to_identifier(&self, name: impl AsRef<str>) -> ast::Identifier {
        ast::Identifier {
            id: Name::new(name),
            range: self.current_token_range(),
        }
    }
    fn to_attr(&self, object: Expr, attr: impl AsRef<str>) -> Expr {
        let name = ast::ExprAttribute {
            range: self.current_token_range(),
            attr: self.to_identifier(attr),
            value: Box::new(object),
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

    pub(super) fn parse_env_name(&mut self) -> Expr {
        let attr = self.xonsh_attr("env", true);
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
        let attr = self.xonsh_attr("env", true);

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
        let func = self.xonsh_attr(attr, true);

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
    pub(super) fn parse_special_strings(&mut self, expr: Expr, start: TextSize) -> Expr {
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

    fn literal_true(&self) -> Expr {
        Expr::BooleanLiteral(ast::ExprBooleanLiteral {
            value: true,
            range: self.node_range(self.node_start()),
        })
    }

    pub(super) fn parse_help_expr(&mut self, lhs: Expr, start: TextSize) -> Expr {
        self.bump_any();
        // let help_name = ast::ExprName {
        //     id: "help".to_string(),
        //     ctx: ast::ExprContext::Load,
        //     range: TextRange::empty(start),
        // };
        // Expr::Call(ast::ExprCall {
        //     func: Box::new(Expr::Name(help_name)),
        //     args: vec![lhs],
        //     keywords: vec![],
        //     range: TextRange::empty(start),
        // })

        let range = self.node_range(start);
        let method = if self.at(TokenKind::Question) {
            self.bump_any();
            "superhelp"
        } else {
            "help"
        };
        let attr = self.xonsh_attr(method, false);
        let args = vec![lhs];
        let arguments = ast::Arguments {
            range: self.node_range(start),
            args: args.into_boxed_slice(),
            keywords: vec![].into_boxed_slice(),
        };
        Expr::Call(ast::ExprCall {
            func: Box::new(attr),
            arguments: arguments,
            range,
        })
    }
}

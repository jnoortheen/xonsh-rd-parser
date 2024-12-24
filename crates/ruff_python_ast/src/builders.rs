#![allow(clippy::return_self_not_must_use)]

use ruff_text_size::TextRange;

use crate::name::Name;
use crate::ExprContext;
use crate::{self as ast, Expr};

impl Expr {
    pub fn identifier(name: impl Into<Name>, range: TextRange) -> ast::Identifier {
        ast::Identifier::new(name, range)
    }
    pub fn call(self, arguments: ast::Arguments, range: TextRange) -> Self {
        Expr::from(ast::ExprCall {
            func: Box::new(self),
            arguments,
            range,
        })
    }
    /// turn expr -> expr(..)
    pub fn call0(self, args: Vec<Expr>, range: TextRange) -> Self {
        let arguments = ast::Arguments {
            range,
            args: args.into_boxed_slice(),
            keywords: vec![].into_boxed_slice(),
        };
        self.call(arguments, range)
    }
    /// turn to a starred expression
    pub fn star(self, range: TextRange) -> Self {
        Expr::from(ast::ExprStarred {
            value: Box::new(self),
            ctx: ExprContext::Load,
            range,
        })
    }
    pub fn call_empty(self, range: TextRange) -> Self {
        self.call0(vec![], range)
    }
    pub fn attr(self, name: impl Into<Name>, range: TextRange) -> Expr {
        let name = ast::ExprAttribute {
            range,
            attr: Expr::identifier(name, range),
            value: Box::new(self),
            ctx: ExprContext::Load,
        };

        Expr::from(name)
    }
}

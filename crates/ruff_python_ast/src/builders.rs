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
            range: range,
        })
    }
    pub fn call_empty(self, range: TextRange) -> Self {
        self.call(
            ast::Arguments {
                range: range,
                args: vec![].into_boxed_slice(),
                keywords: vec![].into_boxed_slice(),
            },
            range,
        )
    }
    pub fn attr(self, name: impl Into<Name>, range: TextRange) -> Expr {
        let name = ast::ExprAttribute {
            range: range,
            attr: Expr::identifier(name, range),
            value: Box::new(self),
            ctx: ExprContext::Load,
        };

        Expr::from(name)
    }
}

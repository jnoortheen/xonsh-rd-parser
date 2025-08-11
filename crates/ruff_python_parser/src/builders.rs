#![allow(clippy::return_self_not_must_use)]
///
/// Helper methods to build AST from expressions.
///
use ruff_python_ast::name::Name;
use ruff_python_ast::{self as ast, Expr};
use ruff_python_ast::{AtomicNodeIndex, ExprContext};
use ruff_text_size::TextRange;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ExprWrap(pub(crate) ast::Expr);

impl From<Expr> for ExprWrap {
    fn from(expr: Expr) -> Self {
        Self(expr)
    }
}

impl From<ExprWrap> for Expr {
    fn from(expr: ExprWrap) -> Self {
        expr.0
    }
}

impl ExprWrap {
    pub(crate) fn identifier(name: impl Into<Name>, range: TextRange) -> ast::Identifier {
        ast::Identifier::new(name, range)
    }
    pub(crate) fn call(self, arguments: ast::Arguments, range: TextRange) -> Self {
        Expr::from(ast::ExprCall {
            func: Box::new(self.0),
            arguments,
            range,
            node_index: AtomicNodeIndex::dummy(),
        })
        .into()
    }
    /// turn expr -> expr(..)
    pub(crate) fn call0(self, args: Vec<Expr>, range: TextRange) -> Self {
        let arguments = ast::Arguments {
            range,
            args: args.into_boxed_slice(),
            keywords: vec![].into_boxed_slice(),
            node_index: AtomicNodeIndex::dummy(),
        };
        self.call(arguments, range)
    }
    /// turn to a starred expression
    pub(crate) fn star(self, range: TextRange) -> Expr {
        Expr::from(ast::ExprStarred {
            value: Box::new(self.0),
            ctx: ExprContext::Load,
            range,
            node_index: AtomicNodeIndex::dummy(),
        })
    }
    pub(crate) fn call_empty(self, range: TextRange) -> Self {
        self.call0(vec![], range)
    }
    pub(crate) fn attr(self, name: impl Into<Name>, range: TextRange) -> Self {
        let name = ast::ExprAttribute {
            range,
            attr: Self::identifier(name, range),
            value: Box::new(self.0),
            ctx: ExprContext::Load,
            node_index: AtomicNodeIndex::dummy(),
        };

        Expr::from(name).into()
    }
}

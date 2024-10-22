use crate::ast_module::{AstModule, Callable};
use crate::to_ast::ToAst;
use pyo3::PyObject;
use ruff_python_ast::*;

type PyResult = pyo3::PyResult<PyObject>;

impl ToAst for Expr {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        match self {
            Expr::BoolOp(expr) => expr.to_ast(module),
            Expr::Named(expr) => expr.to_ast(module),
            Expr::BinOp(expr) => expr.to_ast(module),
            Expr::UnaryOp(expr) => expr.to_ast(module),
            Expr::Lambda(expr) => expr.to_ast(module),
            Expr::If(expr) => expr.to_ast(module),
            Expr::Dict(expr) => expr.to_ast(module),
            Expr::Set(expr) => expr.to_ast(module),
            Expr::List(expr) => expr.to_ast(module),
            Expr::ListComp(expr) => expr.to_ast(module),
            Expr::SetComp(expr) => expr.to_ast(module),
            Expr::DictComp(expr) => expr.to_ast(module),
            Expr::Generator(expr) => expr.to_ast(module),
            Expr::Tuple(expr) => expr.to_ast(module),
            Expr::Slice(expr) => expr.to_ast(module),
            Expr::Call(expr) => expr.to_ast(module),
            Expr::Attribute(expr) => expr.to_ast(module),
            Expr::Subscript(expr) => expr.to_ast(module),
            Expr::Starred(expr) => expr.to_ast(module),
            Expr::Name(expr) => expr.to_ast(module),
            Expr::List(expr) => expr.to_ast(module),
            Expr::Await(expr_await) => expr.to_ast(module),
            Expr::Yield(expr_yield) => expr.to_ast(module),
            Expr::YieldFrom(expr_yield_from) => expr.to_ast(module),
            Expr::Compare(expr_compare) => expr.to_ast(module),
            Expr::FString(expr_fstring) => expr.to_ast(module),
            Expr::StringLiteral(expr_string_literal) => expr.to_ast(module),
            Expr::BytesLiteral(expr_bytes_literal) => expr.to_ast(module),
            Expr::NumberLiteral(expr_number_literal) => expr.to_ast(module),
            Expr::BooleanLiteral(expr_boolean_literal) => expr.to_ast(module),
            Expr::NoneLiteral(expr_none_literal) => expr.to_ast(module),
            Expr::EllipsisLiteral(expr_ellipsis_literal) => expr.to_ast(module),
            Expr::IpyEscapeCommand(expr_ipy_escape_command) => unreachable!(),
        }
    }
}

impl ToAst for ExprBoolOp {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module.attr("BoolOp")?.call_with_loc(
            self.range,
            [
                ("op", self.op.to_ast(module)?),
                ("values", self.values.to_ast(module)?),
            ],
        )
    }
}

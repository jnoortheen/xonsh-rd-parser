use std::vec;

use crate::ast_module::{AstModule, Callable};
use crate::to_ast::ToAst;
use pyo3::types::PyAnyMethods;
use pyo3::{IntoPy, PyObject};
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
impl ToAst for ExprAttribute {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module.attr("Attribute")?.call_with_loc(
            self.range,
            [
                ("value", self.value.to_ast(module)?),
                ("attr", self.attr.to_ast(module)?),
                ("ctx", self.ctx.to_ast(module)?),
            ],
        )
    }
}
impl ToAst for Keyword {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module.attr("keyword")?.call_with_loc(
            self.range,
            [
                ("arg", self.arg.to_ast(module)?),
                ("value", self.value.to_ast(module)?),
            ],
        )
    }
}
impl ToAst for ExprCall {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module.attr("Call")?.call_with_loc(
            self.range,
            [
                ("func", self.func.to_ast(module)?),
                ("args", self.arguments.args.to_ast(module)?),
                ("keywords", self.arguments.keywords.to_ast(module)?),
            ],
        )
    }
}
impl ToAst for ExprSlice {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module.attr("Slice")?.call_with_loc(
            self.range,
            [
                ("lower", self.lower.to_ast(module)?),
                ("upper", self.upper.to_ast(module)?),
                ("step", self.step.to_ast(module)?),
            ],
        )
    }
}
impl ToAst for ExprContext {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        let obj = match self {
            ExprContext::Del => module.attr("Del")?.call0()?.into(),
            ExprContext::Load => module.attr("Load")?.call0()?.into(),
            ExprContext::Store => module.attr("Store")?.call0()?.into(),
            ExprContext::Invalid => todo!(),
        };
        Ok(obj)
    }
}
impl ToAst for ExprTuple {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module.attr("Tuple")?.call_with_loc(
            self.range,
            [
                ("elts", self.elts.to_ast(module)?),
                ("ctx", self.ctx.to_ast(module)?),
            ],
        )
    }
}
impl ToAst for ExprGenerator {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module.attr("GeneratorExp")?.call_with_loc(
            self.range,
            [
                ("elt", self.elt.to_ast(module)?),
                ("generators", self.generators.to_ast(module)?),
            ],
        )
    }
}
impl ToAst for Comprehension {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module.attr("comprehension")?.call_with_loc(
            self.range,
            [
                ("target", self.target.to_ast(module)?),
                ("iter", self.iter.to_ast(module)?),
                ("ifs", self.ifs.to_ast(module)?),
                ("is_async", self.is_async.into_py(module.py)),
            ],
        )
    }
}
impl ToAst for ExprDictComp {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module.attr("DictComp")?.call_with_loc(
            self.range,
            [
                ("key", self.key.to_ast(module)?),
                ("value", self.value.to_ast(module)?),
                ("generators", self.generators.to_ast(module)?),
            ],
        )
    }
}
impl ToAst for ExprSetComp {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module.attr("SetComp")?.call_with_loc(
            self.range,
            [
                ("elt", self.elt.to_ast(module)?),
                ("generators", self.generators.to_ast(module)?),
            ],
        )
    }
}
impl ToAst for ExprListComp {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module.attr("ListComp")?.call_with_loc(
            self.range,
            [
                ("elt", self.elt.to_ast(module)?),
                ("generators", self.generators.to_ast(module)?),
            ],
        )
    }
}
impl ToAst for ExprList {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module
            .attr("List")?
            .call_with_loc(self.range, [("elts", self.elts.to_ast(module)?)])
    }
}
impl ToAst for ExprSet {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module
            .attr("Set")?
            .call_with_loc(self.range, [("elts", self.elts.to_ast(module)?)])
    }
}
impl ToAst for ExprDict {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        let mut keys = vec![];
        let mut values = vec![];
        for item in self.items.iter() {
            keys.push(item.key.to_ast(module)?);
            values.push(item.value.to_ast(module)?);
        }
        module
            .attr("Dict")?
            .call_with_loc(self.range, [("keys", keys), ("values", values)])
    }
}
impl ToAst for ExprIf {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module.attr("If")?.call_with_loc(
            self.range,
            [
                ("test", self.test.to_ast(module)?),
                ("body", self.body.to_ast(module)?),
                ("orelse", self.orelse.to_ast(module)?),
            ],
        )
    }
}
impl ToAst for ExprLambda {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module.attr("Lambda")?.call_with_loc(
            self.range,
            [
                ("args", self.parameters.to_ast(module)?),
                ("body", self.body.to_ast(module)?),
            ],
        )
    }
}
impl ToAst for ExprBinOp {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module.attr("BinOp")?.call_with_loc(
            self.range,
            [
                ("left", self.left.to_ast(module)?),
                ("op", self.op.to_ast(module)?),
                ("right", self.right.to_ast(module)?),
            ],
        )
    }
}
impl ToAst for UnaryOp {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        let obj = match self {
            UnaryOp::Invert => module.attr("Invert")?.call0()?.into(),
            UnaryOp::Not => module.attr("Not")?.call0()?.into(),
            UnaryOp::UAdd => module.attr("UAdd")?.call0()?.into(),
            UnaryOp::USub => module.attr("USub")?.call0()?.into(),
        };
        Ok(obj)
    }
}
impl ToAst for ExprUnaryOp {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module.attr("UnaryOp")?.call_with_loc(
            self.range,
            [
                ("op", self.op.to_ast(module)?),
                ("operand", self.operand.to_ast(module)?),
            ],
        )
    }
}
impl ToAst for Operator {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        let obj = match self {
            Operator::Add => module.attr("Add")?.call0()?.into(),
            Operator::Sub => module.attr("Sub")?.call0()?.into(),
            Operator::Mult => module.attr("Mult")?.call0()?.into(),
            Operator::MatMult => module.attr("MatMult")?.call0()?.into(),
            Operator::Div => module.attr("Div")?.call0()?.into(),
            Operator::Mod => module.attr("Mod")?.call0()?.into(),
            Operator::Pow => module.attr("Pow")?.call0()?.into(),
            Operator::LShift => module.attr("LShift")?.call0()?.into(),
            Operator::RShift => module.attr("RShift")?.call0()?.into(),
            Operator::BitOr => module.attr("BitOr")?.call0()?.into(),
            Operator::BitXor => module.attr("BitXor")?.call0()?.into(),
            Operator::FloorDiv => module.attr("FloorDiv")?.call0()?.into(),
            Operator::BitAnd => module.attr("BitAnd")?.call0()?.into(),
        };
        Ok(obj)
    }
}
impl ToAst for BoolOp {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        let obj = match self {
            BoolOp::And => module.attr("And")?.call0()?.into(),
            BoolOp::Or => module.attr("Or")?.call0()?.into(),
        };
        Ok(obj)
    }
}

impl ToAst for ExprNamed {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module.attr("NamedExpr")?.call_with_loc(
            self.range,
            [
                ("target", self.target.to_ast(module)?),
                ("value", self.value.to_ast(module)?),
            ],
        )
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

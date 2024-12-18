#![allow(clippy::wildcard_imports)]

use super::ToAst;
use crate::ast_module::AstModule;
use crate::impl_to_ast;
use num_complex::Complex;
use pyo3::{IntoPyObjectExt, PyObject};
use ruff_python_ast::str_prefix::StringLiteralPrefix;
use ruff_python_ast::*;
use ruff_text_size::Ranged;
use std::borrow::Cow;
use std::vec;

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
            Expr::Await(expr) => expr.to_ast(module),
            Expr::Yield(expr) => expr.to_ast(module),
            Expr::YieldFrom(expr) => expr.to_ast(module),
            Expr::Compare(expr) => expr.to_ast(module),
            Expr::FString(expr) => expr.to_ast(module),
            Expr::StringLiteral(expr) => expr.to_ast(module),
            Expr::BytesLiteral(expr) => expr.to_ast(module),
            Expr::NumberLiteral(expr) => expr.to_ast(module),
            Expr::BooleanLiteral(expr) => expr.to_ast(module),
            Expr::NoneLiteral(expr) => expr.to_ast(module),
            Expr::EllipsisLiteral(expr) => expr.to_ast(module),
            Expr::IpyEscapeCommand(_expr) => unreachable!(),
        }
    }
}
impl ToAst for ExprNumberLiteral {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        let value = match &self.value {
            Number::Int(value) => value.as_u64().into_py_any(module.py),
            Number::Float(value) => value.into_py_any(module.py),
            Number::Complex { real, imag } => Complex {
                re: *real,
                im: *imag,
            }
            .into_py_any(module.py),
        };
        module.to_const(self.range(), value?.into_py_any(module.py)?)
    }
}
impl_to_ast!(ExprEllipsisLiteral, |module| module.py.Ellipsis());
impl_to_ast!(ExprNoneLiteral, |module| module.py.None());
impl ToAst for ExprBooleanLiteral {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module.to_const(self.range(), self.value)
    }
}
impl ToAst for ExprBytesLiteral {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        let value: Cow<[u8]> = Cow::Owned(self.value.bytes().collect::<Vec<u8>>());
        module.to_const(self.range(), value)
    }
}
impl ToAst for ExprStringLiteral {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        let kind = if self.value.is_unicode() {
            Some("u")
        } else {
            None
        };
        let kwargs = [
            (
                "value",
                self.value.to_str().to_string().into_py_any(module.py)?,
            ),
            ("kind", kind.into_py_any(module.py)?),
        ];
        module.attr("Constant")?.call(self.range(), kwargs)
    }
}
impl ToAst for ExprFString {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        let mut parts = vec![];
        for p in self.value.as_slice() {
            match p {
                FStringPart::Literal(s) => {
                    let kind: PyObject = match s.flags.prefix() {
                        StringLiteralPrefix::Unicode => Some("u"),
                        _ => None,
                    }
                    .into_py_any(module.py)?;
                    let obj = module.attr("Constant")?.call(
                        self.range(),
                        [
                            ("kind", kind),
                            ("value", s.as_str().into_py_any(module.py)?),
                        ],
                    )?;
                    parts.push(obj);
                }
                FStringPart::FString(fs) => {
                    for p in &fs.elements {
                        parts.push(p.to_ast(module)?);
                    }
                }
            }
        }
        module
            .attr("JoinedStr")?
            .call(self.range, [("values", parts)])
    }
}
impl ToAst for ConversionFlag {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        let flag = *self as i8;
        flag.into_py_any(module.py)
    }
}
impl ToAst for FStringFormatSpec {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        let mut values = vec![];
        for value in &self.elements {
            values.push(value.to_ast(module)?);
        }
        module
            .attr("JoinedStr")?
            .call(self.range(), [("values", values.into_py_any(module.py)?)])
    }
}
impl ToAst for FStringExpressionElement {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module.attr("FormattedValue")?.call(
            self.range,
            [
                ("value", self.expression.to_ast(module)?),
                ("conversion", self.conversion.to_ast(module)?),
                ("format_spec", self.format_spec.to_ast(module)?),
            ],
        )
    }
}
impl ToAst for FStringElement {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        let obj = match self {
            FStringElement::Literal(literal) => module.to_const(
                self.range(),
                literal.value.to_string().into_py_any(module.py)?,
            )?,
            FStringElement::Expression(expr) => expr.to_ast(module)?,
        };
        Ok(obj)
    }
}
impl ToAst for CmpOp {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        let obj = match self {
            CmpOp::Eq => module.attr("Eq")?.call0()?,
            CmpOp::NotEq => module.attr("NotEq")?.call0()?,
            CmpOp::Lt => module.attr("Lt")?.call0()?,
            CmpOp::LtE => module.attr("LtE")?.call0()?,
            CmpOp::Gt => module.attr("Gt")?.call0()?,
            CmpOp::GtE => module.attr("GtE")?.call0()?,
            CmpOp::Is => module.attr("Is")?.call0()?,
            CmpOp::IsNot => module.attr("IsNot")?.call0()?,
            CmpOp::In => module.attr("In")?.call0()?,
            CmpOp::NotIn => module.attr("NotIn")?.call0()?,
        };
        Ok(obj)
    }
}
impl_to_ast!(ExprCompare, call "Compare" with fields [
    left,
    ops,
    comparators
]);
impl_to_ast!(ExprYieldFrom, call "YieldFrom" with fields [value]);
impl_to_ast!(ExprYield, call "Yield" with fields [value]);
impl_to_ast!(ExprAwait, call "Await" with fields [value]);
impl_to_ast!(ExprName, call "Name" with fields [id, ctx]);
impl ToAst for name::Name {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        self.as_str().to_string().into_py_any(module.py)
    }
}
impl_to_ast!(ExprStarred, call "Starred" with fields [value, ctx]);
impl_to_ast!(ExprSubscript, call "Subscript" with fields [value, slice, ctx]);
impl_to_ast!(ExprAttribute, call "Attribute" with fields [value, attr, ctx]);
impl_to_ast!(Keyword, call "keyword" with fields [arg, value]);
impl ToAst for ExprCall {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module.attr("Call")?.call(
            self.range,
            [
                ("func", self.func.to_ast(module)?),
                ("args", self.arguments.args.to_ast(module)?),
                ("keywords", self.arguments.keywords.to_ast(module)?),
            ],
        )
    }
}
impl_to_ast!(ExprSlice, call "Slice" with fields [lower, upper, step]);
impl ToAst for ExprContext {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        let obj = match self {
            ExprContext::Del => module.attr("Del")?.call0()?,
            ExprContext::Load => module.attr("Load")?.call0()?,
            ExprContext::Store => module.attr("Store")?.call0()?,
            ExprContext::Invalid => todo!(),
        };
        Ok(obj)
    }
}
impl_to_ast!(ExprTuple, call "Tuple" with fields [elts, ctx]);
impl_to_ast!(ExprGenerator, call "GeneratorExp" with fields [elt, generators]);
impl ToAst for Comprehension {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module.attr("comprehension")?.callk([
            ("target", self.target.to_ast(module)?),
            ("iter", self.iter.to_ast(module)?),
            ("ifs", self.ifs.to_ast(module)?),
            ("is_async", u8::from(self.is_async).into_py_any(module.py)?),
        ])
    }
}
impl_to_ast!(ExprDictComp, call "DictComp" with fields [
    key,
    value,
    generators
]);
impl_to_ast!(ExprSetComp, call "SetComp" with fields [
    elt,
    generators
]);
impl_to_ast!(ExprListComp, call "ListComp" with fields [
    elt,
    generators
]);
impl_to_ast!(ExprList, call "List" with fields [elts, ctx]);
impl_to_ast!(ExprSet, call "Set" with fields [elts]);
impl ToAst for ExprDict {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        let mut keys = vec![];
        let mut values = vec![];
        for item in &self.items {
            keys.push(item.key.to_ast(module)?);
            values.push(item.value.to_ast(module)?);
        }
        module
            .attr("Dict")?
            .call(self.range, [("keys", keys), ("values", values)])
    }
}
impl_to_ast!(ExprIf, call "IfExp" with fields [test, body, orelse]);

struct OptionalParameters(pub Option<Box<Parameters>>);

// special case for Parameters
impl ToAst for OptionalParameters {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        match &self.0 {
            Some(parameters) => parameters.to_ast(module),
            None => module.attr("arguments")?.callk([
                ("posonlyargs", module.empty_list()?),
                ("args", module.empty_list()?),
                ("defaults", module.empty_list()?),
                ("kwonlyargs", module.empty_list()?),
                ("kw_defaults", module.empty_list()?),
                ("vararg", module.py.None()),
                ("kwarg", module.py.None()),
            ]),
        }
    }
}
impl ToAst for ExprLambda {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        let params = OptionalParameters(self.parameters.clone());
        module.attr("Lambda")?.call(
            self.range,
            [
                ("args", params.to_ast(module)?),
                ("body", self.body.to_ast(module)?),
            ],
        )
    }
}
impl_to_ast!(ExprBinOp, call "BinOp" with fields [left, op, right]);
impl ToAst for UnaryOp {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        let obj = match self {
            UnaryOp::Invert => module.attr("Invert")?.call0()?,
            UnaryOp::Not => module.attr("Not")?.call0()?,
            UnaryOp::UAdd => module.attr("UAdd")?.call0()?,
            UnaryOp::USub => module.attr("USub")?.call0()?,
        };
        Ok(obj)
    }
}

impl_to_ast!(ExprUnaryOp, call "UnaryOp" with fields [op, operand]);

impl ToAst for Operator {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        let obj = match self {
            Operator::Add => module.attr("Add")?.call0()?,
            Operator::Sub => module.attr("Sub")?.call0()?,
            Operator::Mult => module.attr("Mult")?.call0()?,
            Operator::MatMult => module.attr("MatMult")?.call0()?,
            Operator::Div => module.attr("Div")?.call0()?,
            Operator::Mod => module.attr("Mod")?.call0()?,
            Operator::Pow => module.attr("Pow")?.call0()?,
            Operator::LShift => module.attr("LShift")?.call0()?,
            Operator::RShift => module.attr("RShift")?.call0()?,
            Operator::BitOr => module.attr("BitOr")?.call0()?,
            Operator::BitXor => module.attr("BitXor")?.call0()?,
            Operator::FloorDiv => module.attr("FloorDiv")?.call0()?,
            Operator::BitAnd => module.attr("BitAnd")?.call0()?,
        };
        Ok(obj)
    }
}
impl ToAst for BoolOp {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        let obj = match self {
            BoolOp::And | BoolOp::And2 => module.attr("And")?.call0()?,
            BoolOp::Or | BoolOp::Or2 => module.attr("Or")?.call0()?,
        };
        Ok(obj)
    }
}
impl_to_ast!(ExprNamed, call "NamedExpr" with fields [target, value]);
impl_to_ast!(ExprBoolOp, call "BoolOp" with fields [op, values]);

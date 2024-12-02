use std::borrow::Cow;
use std::vec;

use super::ToAst;
use crate::ast_module::AstModule;
use num_complex::Complex;
use pyo3::{IntoPyObjectExt, PyObject};
use ruff_python_ast::str_prefix::StringLiteralPrefix;
use ruff_python_ast::*;
use ruff_text_size::Ranged;

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
impl ToAst for ExprEllipsisLiteral {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module.to_const(self.range(), module.py.Ellipsis())
    }
}
impl ToAst for ExprNoneLiteral {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module.to_const(self.range(), module.py.None())
    }
}
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
        for p in self.value.as_slice().iter() {
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
                    for p in fs.elements.iter() {
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
        for value in self.elements.iter() {
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
            CmpOp::Eq => module.attr("Eq")?.call0()?.into(),
            CmpOp::NotEq => module.attr("NotEq")?.call0()?.into(),
            CmpOp::Lt => module.attr("Lt")?.call0()?.into(),
            CmpOp::LtE => module.attr("LtE")?.call0()?.into(),
            CmpOp::Gt => module.attr("Gt")?.call0()?.into(),
            CmpOp::GtE => module.attr("GtE")?.call0()?.into(),
            CmpOp::Is => module.attr("Is")?.call0()?.into(),
            CmpOp::IsNot => module.attr("IsNot")?.call0()?.into(),
            CmpOp::In => module.attr("In")?.call0()?.into(),
            CmpOp::NotIn => module.attr("NotIn")?.call0()?.into(),
        };
        Ok(obj)
    }
}
impl ToAst for ExprCompare {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module.attr("Compare")?.call(
            self.range,
            [
                ("left", self.left.to_ast(module)?),
                ("ops", self.ops.to_ast(module)?),
                ("comparators", self.comparators.to_ast(module)?),
            ],
        )
    }
}
impl ToAst for ExprYieldFrom {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module
            .attr("YieldFrom")?
            .call(self.range, [("value", self.value.to_ast(module)?)])
    }
}
impl ToAst for ExprYield {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module
            .attr("Yield")?
            .call(self.range, [("value", self.value.to_ast(module)?)])
    }
}
impl ToAst for ExprAwait {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module
            .attr("Await")?
            .call(self.range, [("value", self.value.to_ast(module)?)])
    }
}
impl ToAst for ExprName {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module.attr("Name")?.call(
            self.range,
            [
                ("id", self.id.to_ast(module)?),
                ("ctx", self.ctx.to_ast(module)?),
            ],
        )
    }
}
impl ToAst for name::Name {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        self.as_str().to_string().into_py_any(module.py)
    }
}
impl ToAst for ExprStarred {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module.attr("Starred")?.call(
            self.range,
            [
                ("value", self.value.to_ast(module)?),
                ("ctx", self.ctx.to_ast(module)?),
            ],
        )
    }
}
impl ToAst for ExprSubscript {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module.attr("Subscript")?.call(
            self.range,
            [
                ("ctx", self.ctx.to_ast(module)?),
                ("value", self.value.to_ast(module)?),
                ("slice", self.slice.to_ast(module)?),
            ],
        )
    }
}
impl ToAst for ExprAttribute {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module.attr("Attribute")?.call(
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
        module.attr("keyword")?.call(
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
impl ToAst for ExprSlice {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module.attr("Slice")?.call(
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
        module.attr("Tuple")?.call(
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
        module.attr("GeneratorExp")?.call(
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
        module.attr("comprehension")?.callk([
            ("target", self.target.to_ast(module)?),
            ("iter", self.iter.to_ast(module)?),
            ("ifs", self.ifs.to_ast(module)?),
            ("is_async", (self.is_async as u8).into_py_any(module.py)?),
        ])
    }
}
impl ToAst for ExprDictComp {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module.attr("DictComp")?.call(
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
        module.attr("SetComp")?.call(
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
        module.attr("ListComp")?.call(
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
        module.attr("List")?.call(
            self.range,
            [
                ("elts", self.elts.to_ast(module)?),
                ("ctx", self.ctx.to_ast(module)?),
            ],
        )
    }
}
impl ToAst for ExprSet {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module
            .attr("Set")?
            .call(self.range, [("elts", self.elts.to_ast(module)?)])
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
            .call(self.range, [("keys", keys), ("values", values)])
    }
}
impl ToAst for ExprIf {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module.attr("IfExp")?.call(
            self.range,
            [
                ("test", self.test.to_ast(module)?),
                ("body", self.body.to_ast(module)?),
                ("orelse", self.orelse.to_ast(module)?),
            ],
        )
    }
}

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
impl ToAst for ExprBinOp {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module.attr("BinOp")?.call(
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
        module.attr("UnaryOp")?.call(
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
            BoolOp::And | BoolOp::And2 => module.attr("And")?.call0()?.into(),
            BoolOp::Or | BoolOp::Or2 => module.attr("Or")?.call0()?.into(),
        };
        Ok(obj)
    }
}

impl ToAst for ExprNamed {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module.attr("NamedExpr")?.call(
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
        module.attr("BoolOp")?.call(
            self.range,
            [
                ("op", self.op.to_ast(module)?),
                ("values", self.values.to_ast(module)?),
            ],
        )
    }
}

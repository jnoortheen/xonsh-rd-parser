use std::vec;

use super::ToAst;
use crate::ast_module::AstModule;
use pyo3::{IntoPyObjectExt, PyObject};
use ruff_python_ast::*;

type PyResult = pyo3::PyResult<PyObject>;

impl ToAst for Parameter {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module.attr("arg")?.call(
            self.range,
            [
                ("arg", self.name.to_ast(module)?),
                ("annotation", self.annotation.to_ast(module)?),
            ],
        )
    }
}

impl ToAst for ParameterWithDefault {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        self.parameter.to_ast(module)
    }
}

impl ToAst for Parameters {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        let mut defaults = vec![];
        let mut posonlyargs = vec![];
        for arg in self.posonlyargs.iter() {
            posonlyargs.push(arg.to_ast(module)?);
            if let Some(default) = arg.default.as_ref() {
                defaults.push(default.to_ast(module)?)
            }
        }

        let mut args = vec![];
        for arg in self.args.iter() {
            args.push(arg.to_ast(module)?);
            if let Some(default) = arg.default.as_ref() {
                defaults.push(default.to_ast(module)?)
            }
        }

        let mut kw_defaults = vec![];
        let mut kwonlyargs = vec![];
        for arg in self.kwonlyargs.iter() {
            kwonlyargs.push(arg.to_ast(module)?);
            kw_defaults.push(arg.default.to_ast(module)?);
        }

        module.attr("arguments")?.callk(
            // self.range,
            [
                ("posonlyargs", posonlyargs.into_py_any(module.py)?),
                ("args", args.into_py_any(module.py)?),
                ("defaults", defaults.into_py_any(module.py)?),
                ("kwonlyargs", kwonlyargs.into_py_any(module.py)?),
                ("kw_defaults", kw_defaults.into_py_any(module.py)?),
                ("vararg", self.vararg.to_ast(module)?),
                ("kwarg", self.kwarg.to_ast(module)?),
            ],
        )
    }
}

impl ToAst for StmtAssert {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        Ok(module
            .attr("Assert")?
            .call(
                self.range,
                [
                    ("test", self.test.to_ast(module)?),
                    ("msg", self.msg.to_ast(module)?),
                ],
            )?
            .into())
    }
}

impl ToAst for TypeParams {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        self.type_params.to_ast(module)
    }
}
impl ToAst for TypeParam {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        match self {
            TypeParam::TypeVar(param) => param.to_ast(module),
            TypeParam::ParamSpec(param) => param.to_ast(module),
            TypeParam::TypeVarTuple(param) => param.to_ast(module),
        }
    }
}
impl ToAst for TypeParamTypeVar {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module.attr("TypeVar")?.call(
            self.range,
            [
                ("name", self.name.to_ast(module)?),
                ("bound", self.bound.to_ast(module)?),
                ("default_value", self.default.to_ast(module)?),
            ],
        )
    }
}
impl ToAst for TypeParamParamSpec {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module.attr("ParamSpec")?.call(
            self.range,
            [
                ("name", self.name.to_ast(module)?),
                ("default_value", self.default.to_ast(module)?),
            ],
        )
    }
}
impl ToAst for TypeParamTypeVarTuple {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module.attr("TypeVarTuple")?.call(
            self.range,
            [
                ("name", self.name.to_ast(module)?),
                ("default_value", self.default.to_ast(module)?),
            ],
        )
    }
}
impl ToAst for Identifier {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        Ok(self.as_str().to_string().into_py_any(module.py)?)
    }
}
impl ToAst for StmtFunctionDef {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        let obj = if self.is_async {
            module.attr("AsyncFunctionDef")?
        } else {
            module.attr("FunctionDef")?
        };
        obj.call(
            self.range,
            [
                (
                    "name",
                    self.name.as_str().to_owned().into_py_any(module.py)?,
                ),
                ("args", self.parameters.to_ast(module)?),
                ("returns", self.returns.to_ast(module)?),
                ("body", self.body.to_ast(module)?),
                ("decorator_list", self.decorator_list.to_ast(module)?),
                ("type_params", empty_vec(module, self.type_params.as_ref())?),
            ],
        )
    }
}

fn empty_vec<'py, T, U>(module: &AstModule<'py>, obj: Option<U>) -> PyResult
where
    U: std::ops::Deref<Target = T>,
    T: ToAst,
{
    if let Some(obj) = obj {
        obj.deref().to_ast(module)
    } else {
        module.empty_list()
    }
}

impl ToAst for StmtBreak {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module.attr("Break")?.callk(module.location(self.range))
    }
}

impl ToAst for StmtPass {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module.attr("Pass")?.callk(module.location(self.range))
    }
}

impl ToAst for StmtContinue {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module.attr("Continue")?.callk(module.location(self.range))
    }
}
impl ToAst for Alias {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module.attr("alias")?.call(
            self.range,
            [
                ("name", self.name.to_ast(module)?),
                ("asname", self.asname.to_ast(module)?),
            ],
        )
    }
}
impl ToAst for StmtImport {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module
            .attr("Import")?
            .call(self.range, [("names", self.names.to_ast(module)?)])
    }
}
impl ToAst for StmtImportFrom {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module.attr("ImportFrom")?.call(
            self.range,
            [
                ("names", self.names.to_ast(module)?),
                ("module", self.module.to_ast(module)?),
                ("level", self.level.into_py_any(module.py)?),
            ],
        )
    }
}
impl ToAst for StmtReturn {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module
            .attr("Return")?
            .call(self.range, [("value", self.value.to_ast(module)?)])
    }
}

impl ToAst for StmtExpr {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module
            .attr("Expr")?
            .call(self.range, [("value", self.value.to_ast(module)?)])
    }
}

impl ToAst for StmtAugAssign {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module.attr("AugAssign")?.call(
            self.range,
            [
                ("value", self.value.to_ast(module)?),
                ("target", self.target.to_ast(module)?),
                ("op", self.op.to_ast(module)?),
            ],
        )
    }
}
impl ToAst for StmtAnnAssign {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module.attr("AnnAssign")?.call(
            self.range,
            [
                ("value", self.value.to_ast(module)?),
                ("target", self.target.to_ast(module)?),
                ("annotation", self.annotation.to_ast(module)?),
                ("simple", (self.simple as u8).into_py_any(module.py)?),
            ],
        )
    }
}
impl ToAst for StmtFor {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        let cls = if self.is_async {
            module.attr("AsyncFor")?
        } else {
            module.attr("For")?
        };
        cls.call(
            self.range,
            [
                ("target", self.target.to_ast(module)?),
                ("iter", self.iter.to_ast(module)?),
                ("body", self.body.to_ast(module)?),
                ("orelse", self.orelse.to_ast(module)?),
            ],
        )
    }
}

impl ToAst for Stmt {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        match self {
            Stmt::Assert(stmt) => stmt.to_ast(module),
            Stmt::FunctionDef(stmt) => stmt.to_ast(module),
            Stmt::Import(stmt) => stmt.to_ast(module),
            Stmt::ImportFrom(stmt) => stmt.to_ast(module),
            Stmt::Pass(stmt) => stmt.to_ast(module),
            Stmt::Return(stmt) => stmt.to_ast(module),
            Stmt::Expr(stmt) => stmt.to_ast(module),
            Stmt::Assign(stmt) => stmt.to_ast(module),
            Stmt::AugAssign(stmt) => stmt.to_ast(module),
            Stmt::AnnAssign(stmt) => stmt.to_ast(module),
            Stmt::For(stmt) => stmt.to_ast(module),
            Stmt::While(stmt) => stmt.to_ast(module),
            Stmt::If(stmt) => stmt.to_ast(module),
            Stmt::With(stmt) => stmt.to_ast(module),
            Stmt::ClassDef(stmt) => stmt.to_ast(module),
            Stmt::Try(stmt) => stmt.to_ast(module),
            Stmt::Raise(stmt) => stmt.to_ast(module),
            Stmt::Global(stmt) => stmt.to_ast(module),
            Stmt::Nonlocal(stmt) => stmt.to_ast(module),
            Stmt::Break(stmt) => stmt.to_ast(module),
            Stmt::Continue(stmt) => stmt.to_ast(module),
            Stmt::Delete(stmt) => stmt.to_ast(module),
            Stmt::TypeAlias(stmt) => stmt.to_ast(module),
            Stmt::Match(stmt) => stmt.to_ast(module),
            _ => unreachable!(),
        }
    }
}
impl ToAst for Vec<ElifElseClause> {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        if self.is_empty() {
            return module.empty_list();
        }

        // split as first and rest
        let (first, rest) = self.split_first().unwrap();

        if first.test.is_some() {
            let obj = module.attr("If")?.call(
                first.range,
                [
                    ("test", first.test.to_ast(module)?),
                    ("body", first.body.to_ast(module)?),
                    ("orelse", rest.to_vec().to_ast(module)?),
                ],
            );
            Ok(vec![obj?].into_py_any(module.py)?)
        } else {
            first.body.to_ast(module)
        }
    }
}
impl ToAst for StmtIf {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module.attr("If")?.call(
            self.range,
            [
                ("test", self.test.to_ast(module)?),
                ("body", self.body.to_ast(module)?),
                ("orelse", self.elif_else_clauses.to_ast(module)?),
            ],
        )
    }
}
impl ToAst for StmtWhile {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module.attr("While")?.call(
            self.range,
            [
                ("test", self.test.to_ast(module)?),
                ("body", self.body.to_ast(module)?),
                ("orelse", self.orelse.to_ast(module)?),
            ],
        )
    }
}
impl ToAst for StmtGlobal {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module
            .attr("Global")?
            .call(self.range, [("names", self.names.to_ast(module)?)])
    }
}

impl ToAst for StmtNonlocal {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module
            .attr("Nonlocal")?
            .call(self.range, [("names", self.names.to_ast(module)?)])
    }
}
impl ToAst for StmtRaise {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module.attr("Raise")?.call(
            self.range,
            [
                ("exc", self.exc.to_ast(module)?),
                ("cause", self.cause.to_ast(module)?),
            ],
        )
    }
}
impl ToAst for ExceptHandler {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        match self {
            ExceptHandler::ExceptHandler(node) => module.attr("ExceptHandler")?.callk([
                ("type", node.type_.to_ast(module)?),
                ("name", node.name.to_ast(module)?),
                ("body", node.body.to_ast(module)?),
            ]),
        }
    }
}
impl ToAst for StmtTry {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        let cls = if self.is_star {
            module.attr("TryStar")?
        } else {
            module.attr("Try")?
        };
        cls.call(
            self.range,
            [
                ("body", self.body.to_ast(module)?),
                ("handlers", self.handlers.to_ast(module)?),
                ("orelse", self.orelse.to_ast(module)?),
                ("finalbody", self.finalbody.to_ast(module)?),
            ],
        )
    }
}
impl ToAst for StmtClassDef {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module.attr("ClassDef")?.call(
            self.range,
            [
                ("name", self.name.to_ast(module)?),
                ("bases", self.bases().to_ast(module)?),
                ("keywords", self.keywords().to_ast(module)?),
                ("body", self.body.to_ast(module)?),
                ("decorator_list", self.decorator_list.to_ast(module)?),
                ("type_params", empty_vec(module, self.type_params.as_ref())?),
            ],
        )
    }
}
impl ToAst for Decorator {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        self.expression.to_ast(module)
    }
}
impl ToAst for StmtAssign {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module.attr("Assign")?.call(
            self.range,
            [
                ("targets", self.targets.to_ast(module)?),
                ("value", self.value.to_ast(module)?),
            ],
        )
    }
}
impl ToAst for WithItem {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module.attr("withitem")?.callk(
            // self.range,
            [
                ("context_expr", self.context_expr.to_ast(module)?),
                ("optional_vars", self.optional_vars.to_ast(module)?),
            ],
        )
    }
}
impl ToAst for StmtWith {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        let cls = if self.is_async {
            module.attr("AsyncWith")?
        } else {
            module.attr("With")?
        };
        cls.call(
            self.range,
            [
                ("items", self.items.to_ast(module)?),
                ("body", self.body.to_ast(module)?),
            ],
        )
    }
}
impl ToAst for StmtDelete {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module
            .attr("Delete")?
            .call(self.range, [("targets", self.targets.to_ast(module)?)])
    }
}
impl ToAst for StmtMatch {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module.attr("Match")?.call(
            self.range,
            [
                ("subject", self.subject.to_ast(module)?),
                ("cases", self.cases.to_ast(module)?),
            ],
        )
    }
}
impl ToAst for StmtTypeAlias {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module.attr("TypeAlias")?.call(
            self.range,
            [
                ("name", self.name.to_ast(module)?),
                ("type_params", empty_vec(module, self.type_params.as_ref())?),
                ("value", self.value.to_ast(module)?),
            ],
        )
    }
}
impl ToAst for ModModule {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        let type_ignores: Vec<Expr> = vec![];
        module.attr("Module")?.callk([
            ("body", self.body.to_ast(module)?),
            ("type_ignores", type_ignores.to_ast(module)?),
        ])
    }
}

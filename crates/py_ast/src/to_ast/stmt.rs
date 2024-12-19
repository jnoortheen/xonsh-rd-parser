use std::vec;

use super::ToAst;
use crate::{ast_module::AstModule, impl_to_ast};
use pyo3::{IntoPyObjectExt, PyObject};
use ruff_python_ast::{
    Alias, Decorator, ElifElseClause, ExceptHandler, Expr, Identifier, ModModule, Parameter,
    ParameterWithDefault, Parameters, Stmt, StmtAnnAssign, StmtAssert, StmtAssign, StmtAugAssign,
    StmtBreak, StmtClassDef, StmtContinue, StmtDelete, StmtExpr, StmtFor, StmtFunctionDef,
    StmtGlobal, StmtIf, StmtImport, StmtImportFrom, StmtMatch, StmtNonlocal, StmtPass, StmtRaise,
    StmtReturn, StmtTry, StmtTypeAlias, StmtWhile, StmtWith, TypeParam, TypeParamParamSpec,
    TypeParamTypeVar, TypeParamTypeVarTuple, TypeParams, WithItem,
};

type PyResult = pyo3::PyResult<PyObject>;

impl_to_ast!(Parameter, call "arg" with |self, module| {
    "arg" => self.name.to_ast(module)?,
    "annotation" => self.annotation.to_ast(module)?
});
impl ToAst for ParameterWithDefault {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        self.parameter.to_ast(module)
    }
}

impl ToAst for Parameters {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        let mut defaults = vec![];
        let mut posonlyargs = vec![];
        for arg in &self.posonlyargs {
            posonlyargs.push(arg.to_ast(module)?);
            if let Some(default) = arg.default.as_ref() {
                defaults.push(default.to_ast(module)?);
            }
        }

        let mut args = vec![];
        for arg in &self.args {
            args.push(arg.to_ast(module)?);
            if let Some(default) = arg.default.as_ref() {
                defaults.push(default.to_ast(module)?);
            }
        }

        let mut kw_defaults = vec![];
        let mut kwonlyargs = vec![];
        for arg in &self.kwonlyargs {
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
impl_to_ast!(StmtAssert, call "Assert" with fields [
    test,
    msg
]);

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
impl_to_ast!(TypeParamTypeVar, call "TypeVar" with |self, module| {
    "name" => self.name.to_ast(module)?,
    "bound" => self.bound.to_ast(module)?,
    "default_value" => self.default.to_ast(module)?
});
impl_to_ast!(TypeParamParamSpec, call "ParamSpec" with |self, module| {
    "name" => self.name.to_ast(module)?,
    "default_value" => self.default.to_ast(module)?
});
impl_to_ast!(TypeParamTypeVarTuple, call "TypeVarTuple" with |self, module| {
    "name" => self.name.to_ast(module)?,
    "default_value" => self.default.to_ast(module)?
});
impl ToAst for Identifier {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        self.as_str().to_string().into_py_any(module.py)
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

fn empty_vec<T, U>(module: &AstModule<'_>, obj: Option<U>) -> PyResult
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
impl_to_ast!(StmtBreak, call "Break");
impl_to_ast!(StmtPass, call "Pass");
impl_to_ast!(StmtContinue, call "Continue");
impl_to_ast!(Alias, call "alias" with fields [
    name,
    asname
]);
impl_to_ast!(StmtImport, call "Import" with fields [
    names
]);
impl_to_ast!(StmtImportFrom, call "ImportFrom" with fields [
    module,
    names,
    level
]);
impl_to_ast!(StmtReturn, call "Return" with fields [
    value
]);
impl_to_ast!(StmtExpr, call "Expr" with fields [
    value
]);
impl_to_ast!(StmtAugAssign, call "AugAssign" with fields [
    target,
    op,
    value
]);
impl_to_ast!(StmtAnnAssign, call "AnnAssign" with |self, module| {
    "value" => self.value.to_ast(module)?,
    "target" => self.target.to_ast(module)?,
    "annotation" => self.annotation.to_ast(module)?,
    "simple" => u8::from(self.simple).into_py_any(module.py)?
});
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
            Stmt::IpyEscapeCommand(_) => unreachable!(),
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
impl_to_ast!(StmtIf, call "If" with |self, module| {
    "test" => self.test.to_ast(module)?,
    "body" => self.body.to_ast(module)?,
    "orelse" => self.elif_else_clauses.to_ast(module)?
});
impl_to_ast!(StmtWhile, call "While" with fields [
    test,
    body,
    orelse
]);
impl_to_ast!(StmtGlobal, call "Global" with fields [names]);
impl_to_ast!(StmtNonlocal, call "Nonlocal" with fields [names]);
impl_to_ast!(StmtRaise, call "Raise" with fields [
    exc,
    cause
]);
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
impl_to_ast!(StmtAssign, call "Assign" with fields [
    targets,
    value
]);
impl_to_ast!(WithItem, call "withitem" with fields [
    context_expr,
    optional_vars
]);
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
impl_to_ast!(StmtDelete, call "Delete" with fields [
    targets
]);
impl_to_ast!(StmtMatch, call "Match" with |self, module| {
    "subject" => self.subject.to_ast(module)?,
    "cases" => self.cases.to_ast(module)?
});
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

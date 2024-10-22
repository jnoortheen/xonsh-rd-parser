use crate::ast_module::{AstModule, Callable};
/// Convert Ruff's AST to Python AST.

use pyo3::{IntoPy, PyObject};
use ruff_python_ast::*;

type PyResult = pyo3::PyResult<PyObject>;

pub trait ToAst {
    fn to_ast(&self, module: &AstModule) -> PyResult;
}

impl<T: ToAst> ToAst for Option<T> {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        match self {
            Some(param) => param.to_ast(module),
            None => Ok(module.py.None()),
        }
    }
}
impl<T: ToAst> ToAst for Box<T> {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        self.as_ref().to_ast(module)
    }
}
impl ToAst for Parameter {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module.attr("arg")?.call_with_loc(self.range, [
            ("arg", self.name.to_ast(module)?),
            // ("annotation", self.annotation.to_ast(module)?),
            // ("type_comment", defaults.into_py(module.py)),
        ])
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
            if let Some(default) = arg.default.as_ref() {
                kw_defaults.push(default.to_ast(module)?)
            }
        }

        module.attr("arguments")?.call_with_loc(self.range, [
            ("posonlyargs", posonlyargs.into_py(module.py)),
            ("args", args.into_py(module.py)),
            ("defaults", defaults.into_py(module.py)),
            ("kwonlyargs", kwonlyargs.into_py(module.py)),
            ("kw_defaults", kw_defaults.into_py(module.py)),
            ("vararg", self.vararg.to_ast(module)?),
            ("kwarg", self.kwarg.to_ast(module)?),
        ])
    }
}

impl ToAst for StmtAssert {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        Ok(module.attr("Assert")?.call_with_loc(self.range, [
            ("test", self.test.to_ast(module)?),
            ("msg", self.msg.to_ast(module)?),
        ])?.into())
    }
}

impl<T: ToAst> ToAst for Vec<T> {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        let mut body: Vec<PyObject> = vec![];
        for stmt in self {
            body.push(stmt.to_ast(module)?);
        }
        Ok(body.into_py(module.py))
    }
}

impl ToAst for Expr {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        todo!()
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
            TypeParam::TypeVar(param) => {
                todo!()
            }
            TypeParam::ParamSpec(param) => {
                todo!()
            }
            TypeParam::TypeVarTuple(param) => {
                todo!()
            }
        }
    }
}
impl ToAst for Identifier {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module.to_const(self.as_str().to_string())
    }
}

impl ToAst for StmtFunctionDef {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module.attr("FunctionDef")?.call_with_loc(
            self.range,
            [
                ("name", self.name.to_ast(module)?),
                ("args", self.parameters.to_ast(module)?),
                ("returns", self.returns.to_ast(module)?),
                ("body", self.body.to_ast(module)?),
                ("type_params", self.type_params.to_ast(module)?),
            ],
        )
    }
}

impl ToAst for StmtBreak {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module.attr("Break")?.call0_with_loc(self.range)
    }
}

impl ToAst for StmtPass {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module.attr("Pass")?.call0_with_loc(self.range)
    }
}

impl ToAst for StmtContinue {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        Ok(module.attr("Continue")?.call0_with_loc(self.range)?.into())
    }
}
impl ToAst for Alias {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module.attr("alias")?.callk(
            [("name", self.name.to_ast(module)?), ("asname", self.asname.to_ast(module)?)],
        )
    }
}
impl ToAst for StmtImport {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module.attr("Import")?.callk(
            [("names", self.names.to_ast(module)?)]
        )
    }
}
impl ToAst for StmtImportFrom {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module.attr("ImportFrom")?.callk(
            [
                ("names", self.names.to_ast(module)?),
                ("module", self.module.to_ast(module)?),
            ]
        )
    }
}
impl ToAst for StmtReturn {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        todo!()
    }
}

impl ToAst for StmtExpr {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        todo!()
    }
}

impl ToAst for StmtAugAssign {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        todo!()
    }
}

impl ToAst for StmtAnnAssign {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        todo!()
    }
}

impl ToAst for StmtFor {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        todo!()
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
            _ => todo!(),
        }
    }
}

impl ToAst for StmtIf {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        todo!()
    }
}

impl ToAst for StmtWhile {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        todo!()
    }
}

impl ToAst for StmtGlobal {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        todo!()
    }
}

impl ToAst for StmtNonlocal {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        todo!()
    }
}
impl ToAst for StmtRaise {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        todo!()
    }
}
impl ToAst for StmtTry {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        todo!()
    }
}

impl ToAst for StmtClassDef {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        todo!()
    }
}

impl ToAst for StmtAssign {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        todo!()
    }
}

impl ToAst for StmtWith {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        todo!()
    }
}

impl ToAst for StmtDelete {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        todo!()
    }
}
impl ToAst for StmtMatch {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        todo!()
    }
}
impl ToAst for StmtTypeAlias {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        todo!()
    }
}
impl ToAst for ModModule {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module.attr("Module")?.callk(
            [("body", self.body.to_ast(module)?)],
        )
    }
}

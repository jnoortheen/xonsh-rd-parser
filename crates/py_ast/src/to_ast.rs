/// Convert Ruff's AST to Python AST.
use crate::ast_module::AstModule;
use pyo3::{IntoPy, PyObject};

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
impl<T: ToAst> ToAst for Vec<T> {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        let mut body: Vec<PyObject> = vec![];
        for stmt in self {
            body.push(stmt.to_ast(module)?);
        }
        Ok(body.into_py(module.py))
    }
}
impl<T: ToAst> ToAst for [T] {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        let mut body: Vec<PyObject> = vec![];
        for stmt in self {
            body.push(stmt.to_ast(module)?);
        }
        Ok(body.into_py(module.py))
    }
}

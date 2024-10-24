/// Convert Ruff's AST to Python AST.
use crate::ast_module::{AstModule, Callable};
use pyo3::{IntoPy, PyObject, Python};
use ruff_python_ast::Parameters;

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


pub struct OptionalParameters(pub Option<Box<Parameters>>);

// special case for Parameters
impl ToAst for OptionalParameters {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        fn empty_list<'py>(py: Python<'py>) -> PyObject {
            let empty_vec: Vec<i32> = vec![];  // Explicitly specify the type of Vec
            empty_vec.into_py(py)
        }
        match &self.0 {
            Some(parameters) => parameters.to_ast(module),
            None => module.attr("arguments")?.callk(
                [
                    ("posonlyargs", empty_list(module.py)),
                    ("args", empty_list(module.py)),
                    ("defaults", empty_list(module.py)),
                    ("kwonlyargs", empty_list(module.py)),
                    ("kw_defaults", empty_list(module.py)),
                    ("vararg", module.py.None()),
                    ("kwarg", module.py.None()),
                ],
            )
        }
    }
}
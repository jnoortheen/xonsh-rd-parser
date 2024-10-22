use py_ast::to_ast::ToAst;
use py_ast::ast_module::AstModule;
use pyo3::exceptions::PyValueError;
use pyo3::{PyErr, PyObject, PyResult, Python};


pub fn parse_str(py: Python<'_>, src: &str) -> PyResult<PyObject> {
    let parsed = ruff_python_parser::parse_module(src);
    
    if let Ok(parsed) = parsed {
        let tree = parsed.into_syntax();
        let module = AstModule::new(py)?;
        tree.to_ast(&module)
    } else {
        let errors = parsed.unwrap_err();
        return Err(PyErr::new::<PyValueError, _>(format!("{:?}", errors)));
    }
}

use py_ast::ast_module::AstModule;
use py_ast::to_ast::ToAst;
use pyo3::exceptions::PySyntaxError;
use pyo3::{PyObject, PyResult, Python};


pub fn parse_str(py: Python<'_>, src: &str) -> PyResult<PyObject> {
    let parsed = ruff_python_parser::parse_module(src).map_err(|err| {
        let filename = "<unknown>";
        let lineno = err.location.start().to_u32();
        let offset = err.location.end().to_u32();
        // let end_lineno = err.location.end().to_u32();
        // let end_offset = err.location.end().to_u32();
        // let text = src.to_string();
        let msg = format!("{:?}", err.error);
        PySyntaxError::new_err((msg, filename, lineno, offset))
    })?;
    let tree = parsed.into_syntax();
    let module = AstModule::new(py)?;
    tree.to_ast(&module)
}

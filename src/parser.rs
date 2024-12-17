use py_ast::ast_module::AstModule;
use py_ast::to_ast::ToAst;
use pyo3::{PyObject, PyResult, Python};
use ruff_source_file::{LineIndex, SourceCode};

pub fn parse_str<'py>(py: Python<'py>, src: &'py str, filename: &'py str) -> PyResult<PyObject> {
    let parsed = ruff_python_parser::parse_module(src);
    match parsed {
        Ok(parsed) => {
            let line_index = LineIndex::from_source_text(src);
            let source_code = SourceCode::new(src, &line_index);
            let tree = parsed.into_syntax();
            let module = AstModule::new(py, &source_code)?;
            tree.to_ast(&module)
        }
        Err(err) => {
            let err = crate::annotate_src::to_syntax_err(src, filename, &err);
            Err(err)
        }
    }
}

use py_ast::ast_module::AstModule;
use py_ast::to_ast::ToAst;
use pyo3::prelude::*;
use pyo3::types::PyString;
use ruff_source_file::{LineIndex, SourceCode};

#[pyclass(name = "Parser", module = "xonsh_rd_parser")]
pub struct PyParser {
    src: Py<PyString>,
    file: String,
}

#[pymethods]
impl PyParser {
    #[new]
    #[pyo3(signature = (src, file_name = None))]
    fn new(src: Bound<'_, PyString>, file_name: Option<&'_ str>) -> PyResult<Self> {
        let file = file_name.unwrap_or("<code>").to_string();
        Ok(Self {
            src: src.into(),
            file,
        })
    }

    pub fn parse<'py>(&self, py: Python<'py>) -> PyResult<PyObject> {
        let src = self.src.to_str(py)?;
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
                let filename = self.file.as_str();
                let err = crate::annotate_src::to_syntax_err(src, filename, &err);
                Err(err)
            }
        }
    }

    #[staticmethod]
    pub fn parse_file(py: Python<'_>, path: &str) -> PyResult<PyObject> {
        let src = std::fs::read_to_string(path).unwrap();
        let src = PyString::new(py, &src);
        PyParser::new(src, Some(path))?.parse(py)
    }
}

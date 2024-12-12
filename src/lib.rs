mod annotate_src;
mod lexer;
mod parser;

use parser::parse_str;
use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
mod xonsh_rd_parser {
    use super::*;

    #[pyfunction] // This will be part of the module
    #[pyo3(signature = (src, file_name = None))]
    pub fn parse_string<'py>(
        py: Python<'py>,
        src: &str,
        file_name: Option<&str>,
    ) -> PyResult<PyObject> {
        parse_str(py, src, file_name.unwrap_or("<code>"))
    }

    #[pyfunction]
    pub fn parse_file<'py>(py: Python<'py>, path: &str) -> PyResult<PyObject> {
        let src = std::fs::read_to_string(path).unwrap();
        parse_str(py, &src, path)
    }

    #[pymodule_export]
    use lexer::PyLexer;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_string() {
        let src = "
def foo():
    return 42
";
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let parsed = xonsh_rd_parser::parse_string(py, src, None);
            assert!(parsed.is_ok());
        })
    }
}

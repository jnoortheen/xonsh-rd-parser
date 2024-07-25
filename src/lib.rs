use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use ruff_python_ast::ModModule;
use ruff_python_parser as parser;

/// A Python module implemented in Rust.
#[pymodule]
mod xonsh_rd_parser {
    use super::*;

    #[pyfunction] // This will be part of the module
    fn parse_string<'py>(src: &str) -> PyResult<ModModule> {
        let parsed = parser::parse_module(src);
        if let Ok(parsed) = parsed {
            return Ok(parsed.into_syntax());
        } else {
            let errors = parsed.unwrap_err();
            return Err(PyErr::new::<PyValueError, _>(format!("{:?}", errors)));
        }
    }

    #[pyfunction]
    fn parse_file<'py>(path: &str) -> PyResult<ModModule> {
        let src = std::fs::read_to_string(path).unwrap();
        let parsed = parser::parse_module(&src);
        if let Ok(parsed) = parsed {
            return Ok(parsed.into_syntax());
        } else {
            let errors = parsed.unwrap_err();
            return Err(PyErr::new::<PyValueError, _>(format!("{:?}", errors)));
        }
    }
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
        // Python::with_gil(|py| {
            let parsed = xonsh_rd_parser::parse_string(
                // py,
                src);
            assert!(parsed.is_ok());
        // })
    }
}

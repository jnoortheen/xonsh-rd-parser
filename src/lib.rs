mod annotate_src;
mod parser;

use parser::parse_str;
use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
mod xonsh_rd_parser {
    use super::*;

    #[pyfunction] // This will be part of the module
    pub fn parse_string<'py>(py: Python<'py>, src: &str) -> PyResult<PyObject> {
        parse_str(py, src, None)
    }

    #[pyfunction]
    pub fn parse_file<'py>(py: Python<'py>, path: &str) -> PyResult<PyObject> {
        let src = std::fs::read_to_string(path).unwrap();
        parse_str(py, &src, Some(path))
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
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let parsed = xonsh_rd_parser::parse_string(py, src);
            assert!(parsed.is_ok());
        })
    }
}

use pyo3::prelude::*;
use pyo3::types::IntoPyDict;
use ruff_python_parser as parser;
use pyo3::exceptions::PyValueError;


fn get_ast<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyModule>> {
    let module = PyModule::import_bound(py, "ast")?;
    Ok(module)
}

pub fn parse_str(py: Python<'_>, src: &str) -> PyResult<PyObject> {
    let parsed = parser::parse_module(src);
    if let Ok(parsed) = parsed {
        let ast = parsed.into_syntax();
        return Ok("ast".into_py(py));
    } else {
        let errors = parsed.unwrap_err();
        return Err(PyErr::new::<PyValueError, _>(format!("{:?}", errors)));
    }
}


/// A Python module implemented in Rust.
#[pymodule]
mod xonsh_rd_parser {
    use super::*;

    #[pyfunction] // This will be part of the module
    pub fn parse_string<'py>(py: Python<'py>, src: &str) -> PyResult<PyObject> {
        return parse_str(py, src);
    }

    #[pyfunction]
    pub fn parse_file<'py>(py: Python<'py>, path: &str) -> PyResult<PyObject> {
        let src = std::fs::read_to_string(path).unwrap();
        return parse_str(py, &src);
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
        Python::with_gil(|py| {
            let parsed = xonsh_rd_parser::parse_string(
                py,
                src);
            assert!(parsed.is_ok());
        })
    }
}

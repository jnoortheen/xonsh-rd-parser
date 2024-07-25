use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use ruff_python_ast::ModModule;
use ruff_python_parser as parser;

pub fn parse_str(src: &str) -> PyResult<String> {
    let parsed = parser::parse_module(src);
    if let Ok(parsed) = parsed {
        // let ast = parsed.into_syntax();
        let ast = format!("{:?}", &parsed);
        return Ok(ast);
    } else {
        let errors = parsed.unwrap_err();
        return Err(PyErr::new::<PyValueError, _>(format!("{:?}", errors)));
    }
}


/// A Python module implemented in Rust.
#[pymodule]
mod xonsh_rd_parser {
    use ruff_python_ast::AstNode;
    use super::*;

    #[pyfunction] // This will be part of the module
    pub fn parse_string<'py>(src: &str) -> PyResult<String> {
        return parse_str(src);
    }

    #[pyfunction]
    pub fn parse_file<'py>(path: &str) -> PyResult<String> {
        let src = std::fs::read_to_string(path).unwrap();
        return parse_str(&src);
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

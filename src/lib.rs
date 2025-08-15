mod annotate_src;
mod lexer;
mod location;
pub mod parser;
mod parser_test;
pub mod test_utils;

use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
mod xonsh_rd_parser {
    use super::*;

    #[pymodule_export]
    use parser::PyParser;

    /// Formats the sum of two numbers as string.
    #[pyfunction]
    fn get_big_py_file() -> PyResult<String> {
        Ok(test_utils::get_big_py_file())
    }
}

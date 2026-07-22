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

    #[pyfunction]
    #[pyo3(signature = (lines=None, file_name=None))]
    fn get_big_py_file(lines: Option<usize>, file_name: Option<&str>) -> PyResult<String> {
        Ok(test_utils::get_big_py_file(lines, file_name))
    }

    #[pyfunction]
    fn is_debug_build() -> bool {
        cfg!(debug_assertions)
    }

    #[pyfunction]
    fn panic_test() {
        panic!("test panic");
    }
}

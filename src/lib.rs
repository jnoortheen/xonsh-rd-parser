mod annotate_src;
mod lexer;
mod location;
pub mod parser;
mod parser_test;

use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
mod xonsh_rd_parser {
    use super::*;

    #[pymodule_export]
    use parser::PyParser;
}

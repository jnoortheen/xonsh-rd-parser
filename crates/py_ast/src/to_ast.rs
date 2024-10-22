use crate::ast_module::{AST, Callable};
/// Convert Ruff's AST to Python AST.

use pyo3::{IntoPy, PyObject, PyResult, Python};
use ruff_python_ast::{ModModule, Stmt};

pub trait ToAst {
    fn to_ast(&self) -> PyObject;
}
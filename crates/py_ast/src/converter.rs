/// Convert Ruff's AST to Python AST.

use pyo3::{IntoPy, Py, PyErr, PyObject, PyResult, Python};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::PyModule;
use pyo3::types::IntoPyDict;
use crate::ast_module::AST;
use ruff_python_ast::{ModModule, Stmt};




pub struct Converter<'py> {
    // Python ast module
    module: AST<'py>,
    // parsed Ruff's AST
    tree: ModModule,
}


impl<'py> Converter<'py> {
    pub fn new(py: Python<'py>, tree: ModModule) -> PyResult<Self> {
        let module = AST::new(py)?;
        Ok(Self { module, tree })
    }
    
    pub fn convert(&self) -> PyResult<PyObject> {
        let mut body: Vec<PyObject> = vec![];
        for stmt in &self.tree.body {
            let node = match stmt {
                Stmt::FunctionDef(stmt) => {
                    self.module.attr("FunctionDef")
                }
                _ => todo!()
            };
            body.push(node?);
        }
        self.module.to_module(body)
    }
}

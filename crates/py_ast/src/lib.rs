use pyo3::prelude::PyModule;
use pyo3::{Py, PyObject, PyResult, Python};
use pyo3::types::IntoPyDict;

pub struct AST {
    module: Py<PyModule>,
}
impl AST {
    pub fn new(py: Python) -> PyResult<Self> {
        let module = PyModule::import_bound(py, "ast")?.unbind();
        Ok(Self { module })
    }
    
    pub fn to_const(&self, py: Python, value: i64) -> PyResult<PyObject> {
        let constant = self.module.getattr(py, "Constant")?;
        let args = [("value", value)].into_py_dict_bound(py);
        constant.call_bound(py, (), Some(&args))
    }
}

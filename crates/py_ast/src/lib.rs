use pyo3::prelude::PyModule;
use pyo3::{IntoPy, Py, PyObject, PyResult, Python};
use pyo3::types::IntoPyDict;

pub struct AST<'py> {
    module: Py<PyModule>,
    py: Python<'py>,
}

// alias for kwargs
pub type Kwargs<'a> = Vec<(&'a str, PyObject)>;

trait Dispatchable<'py> {
    fn kwargs(&self, py: Python<'py>, kwargs: Kwargs) -> PyResult<PyObject>;
}

impl<'py> Dispatchable<'py> for PyObject {
    fn kwargs(&self, py: Python<'py>, kwargs: Kwargs) -> PyResult<PyObject> {
        let kwargs = kwargs.into_py_dict_bound(py);
        self.call_bound(py, (), Some(&kwargs))
    }
}

impl<'py> AST<'py> {
    pub fn new(py: Python<'py>) -> PyResult<Self> {
        let module = PyModule::import_bound(py, "ast")?.unbind();
        Ok(Self { module, py })
    }

    fn attr(&self, name: &str) -> PyResult<PyObject> {
        self.module.getattr(self.py, name)
    }
    pub fn to_const<T: IntoPy<PyObject>>(&self, value: T) -> PyResult<PyObject> {
        let constant = self.attr("Constant")?;
        constant.kwargs(self.py, vec![("value", value.into_py(self.py))])
    }
    pub fn to_module(&self, body: Vec<PyObject>) -> PyResult<PyObject> {
        let kwargs = [("body", body)].into_py_dict_bound(self.py);
        let ast = self.attr("Module")?.call_bound(
            self.py,
            (),
            Some(&kwargs),
        )?;
        Ok(ast)
    }
    
    pub fn to_pass(&self) -> PyResult<PyObject> {
        self.attr("Pass")?.call0(self.py)
    }
}

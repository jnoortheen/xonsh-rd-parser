/// A wrapper around the Python ast module.
use pyo3::prelude::PyModule;
use pyo3::types::{IntoPyDict, PyAnyMethods};
use pyo3::{IntoPyObject, IntoPyObjectExt, PyObject, PyResult, Python};
use ruff_source_file::SourceCode;
use ruff_text_size::TextRange;

pub struct AstModule<'py> {
    obj: PyObject,
    pub py: Python<'py>,
    source_code: &'py SourceCode<'py, 'py>,
}

impl<'py> AstModule<'py> {
    pub fn new(py: Python<'py>, source_code: &'py SourceCode) -> PyResult<Self> {
        let obj: PyObject = PyModule::import(py, "ast")?.unbind().into();
        Ok(Self {
            obj,
            py,
            source_code,
        })
    }
    pub(crate) fn attr(&self, name: &str) -> PyResult<Self> {
        let obj = self.obj.getattr(self.py, name)?;
        Ok(AstModule {
            obj,
            py: self.py,
            source_code: self.source_code,
        })
    }
    pub fn call<T: IntoPyDict<'py>>(&self, range: TextRange, kwargs: T) -> PyResult<PyObject> {
        let dict = kwargs.into_py_dict(self.py)?;
        for (key, value) in self.location(range) {
            dict.set_item(key, value)?;
        }
        Ok(self.obj.bind(self.py).call((), Some(&dict))?.into())
    }
    pub(crate) fn location(&self, range: TextRange) -> [(&'static str, usize); 4] {
        let start = self.source_code.source_location(range.start());
        let end = self.source_code.source_location(range.end());

        [
            ("lineno", start.row.get()),
            ("col_offset", start.column.get()),
            ("end_lineno", end.row.get()),
            ("end_col_offset", end.column.get()),
        ]
    }
    pub fn to_const<T: IntoPyObject<'py>>(&self, range: TextRange, value: T) -> PyResult<PyObject> {
        self.attr("Constant")?.call(range, [("value", value)])
    }
    pub fn call0(&self) -> PyResult<PyObject> {
        Ok(self.obj.bind(self.py).call0()?.into())
    }
    pub fn callk<T: IntoPyDict<'py>>(&self, kwargs: T) -> PyResult<PyObject> {
        let kwargs = kwargs.into_py_dict(self.py)?;
        Ok(self.obj.bind(self.py).call((), Some(&kwargs))?.into())
    }

    pub fn empty_list(&self) -> PyResult<PyObject> {
        let empty_vec: Vec<i32> = vec![]; // Explicitly specify the type of Vec
        empty_vec.into_py_any(self.py)
    }
}

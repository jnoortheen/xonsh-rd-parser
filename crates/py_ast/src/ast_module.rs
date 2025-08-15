/// A wrapper around the Python ast module.
use pyo3::prelude::PyModule;
use pyo3::types::{IntoPyDict, PyAnyMethods, PyString};
use pyo3::{Bound, IntoPyObject, IntoPyObjectExt, PyAny, PyObject, PyResult, Python, intern};
use ruff_source_file::SourceCode;
use ruff_text_size::TextRange;

pub struct AstModule<'py> {
    obj: Bound<'py, PyAny>,
    source_code: &'py SourceCode<'py, 'py>,
}

impl<'py> AstModule<'py> {
    pub fn py(&self) -> Python<'py> {
        self.obj.py()
    }
    pub fn new(py: Python<'py>, source_code: &'py SourceCode) -> PyResult<Self> {
        let obj = PyModule::import(py, "ast")?;
        Ok(Self {
            obj: obj.into_any(),
            source_code,
        })
    }
    pub(crate) fn attr(&self, name: &str) -> PyResult<Self> {
        let obj = self.obj.getattr(name)?;
        Ok(AstModule {
            obj,
            source_code: self.source_code,
        })
    }
    pub fn call<T: IntoPyDict<'py>>(&self, range: TextRange, kwargs: T) -> PyResult<PyObject> {
        let dict = kwargs.into_py_dict(self.obj.py())?;
        for (key, value) in self.location(range) {
            dict.set_item(key, value)?;
        }
        Ok(self.obj.call((), Some(&dict))?.into())
    }
    pub(crate) fn location(&self, range: TextRange) -> [(&Bound<'_, PyString>, usize); 4] {
        let start = self.source_code.line_column(range.start());
        let end = self.source_code.line_column(range.end());
        let py = self.py();
        [
            (intern!(py, "lineno"), start.line.get()),
            (intern!(py, "col_offset"), start.column.get()),
            (intern!(py, "end_lineno"), end.line.get()),
            (intern!(py, "end_col_offset"), end.column.get()),
        ]
    }
    pub fn to_const<T: IntoPyObject<'py>>(&self, range: TextRange, value: T) -> PyResult<PyObject> {
        self.attr("Constant")?.call(range, [("value", value)])
    }
    pub fn call0(&self) -> PyResult<PyObject> {
        Ok(self.obj.call0()?.into())
    }
    pub fn callk<T: IntoPyDict<'py>>(&self, kwargs: T) -> PyResult<PyObject> {
        let kwargs = kwargs.into_py_dict(self.obj.py())?;
        Ok(self.obj.call((), Some(&kwargs))?.into())
    }

    pub fn empty_list(&self) -> PyResult<PyObject> {
        let empty_vec: Vec<i32> = vec![]; // Explicitly specify the type of Vec
        empty_vec.into_py_any(self.obj.py())
    }
}

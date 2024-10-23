/// A wrapper around the Python ast module.
use pyo3::prelude::PyModule;
use pyo3::types::{IntoPyDict, PyAnyMethods};
use pyo3::{Bound, IntoPy, Py, PyAny, PyObject, PyResult, Python, ToPyObject};
use ruff_python_ast::FStringElement;
use ruff_text_size::TextRange;

use crate::to_ast::ToAst;

pub struct AstModule<'py> {
    module: Py<PyModule>,
    pub py: Python<'py>,
}

fn get_location_fields(range: TextRange) -> [(&'static str, u32); 4] {
    [
        ("lineno", range.start().to_u32()),
        ("col_offset", range.start().to_u32()),
        ("end_lineno", range.end().to_u32()),
        ("end_col_offset", range.end().to_u32()),
    ]
}

pub trait Callable<'py> {
    fn callk<T>(&self, kwargs: T) -> PyResult<PyObject>
    where
        T: IntoPyDict;
    fn call_with_loc<T>(&self, range: TextRange, kwargs: T) -> PyResult<PyObject>
    where
        T: IntoPyDict;
    fn call0_with_loc(&self, range: TextRange) -> PyResult<PyObject> {
        self.callk(get_location_fields(range))
    }
}

impl<'py> Callable<'py> for Bound<'py, PyAny> {
    fn callk<T>(&self, kwargs: T) -> PyResult<PyObject>
    where
        T: IntoPyDict,
    {
        let kwargs = kwargs.into_py_dict_bound(self.py());
        Ok(self.call((), Some(&kwargs))?.into())
    }
    fn call_with_loc<T>(&self, range: TextRange, kwargs: T) -> PyResult<PyObject>
    where
        T: IntoPyDict,
    {
        let kwargs = kwargs.into_py_dict_bound(self.py());
        for (key, value) in get_location_fields(range) {
            kwargs.set_item(key, value)?;
        }
        Ok(self.call((), Some(&kwargs))?.into())
    }
}

impl<'py> AstModule<'py> {
    pub fn new(py: Python<'py>) -> PyResult<Self> {
        let module = PyModule::import_bound(py, "ast")?.unbind();
        Ok(Self { module, py })
    }

    pub fn attr(&self, name: &str) -> PyResult<Bound<'py, PyAny>> {
        Ok(self.module.getattr(self.py, name)?.into_bound(self.py))
    }
    pub fn to_const<T: ToPyObject>(&self, value: T) -> PyResult<PyObject> {
        self.attr("Constant")?.callk([("value", value)])
    }
    pub fn to_module(&self, body: Vec<PyObject>) -> PyResult<PyObject> {
        self.attr("Module")?.callk([("body", body)])
    }
    pub fn to_joined_str<'a>(
        &self,
        range: TextRange,
        elements: impl Iterator<Item = &'a FStringElement>,
    ) -> PyResult<PyObject> {
        let mut values = vec![];
        for value in elements {
            values.push(value.to_ast(self)?);
        }
        self.attr("JoinedStr")?
            .call_with_loc(range, [("values", values.into_py(self.py))])
    }
}

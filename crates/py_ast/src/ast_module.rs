/// A wrapper around the Python ast module.
use pyo3::prelude::PyModule;
use pyo3::types::{IntoPyDict, PyAnyMethods, PyDict};
use pyo3::{Bound, IntoPy, Py, PyAny, PyObject, PyResult, Python, ToPyObject};
use ruff_python_ast::FStringElement;
use ruff_text_size::TextRange;
use bon::bon;
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

#[bon]
impl<'py> AstModule<'py> {
    pub fn new(py: Python<'py>) -> PyResult<Self> {
        let module = PyModule::import_bound(py, "ast")?.unbind();
        Ok(Self { module, py })
    }
    #[builder]
    pub fn caller<T: IntoPyDict>(&self, attr: &str, range: Option<TextRange>, kwargs: Option<T>) -> PyResult<PyObject>
    {
        let func = self.module.getattr(self.py, attr)?.into_bound(self.py);

        if kwargs.is_none() && range.is_none() {
            return Ok(func.call0()?.into());
        }
        let dict = if let Some(kwargs) = kwargs {
            kwargs.into_py_dict_bound(self.py)
        } else {
            PyDict::new_bound(self.py)
        };
        if let Some(range) = range {
            dict.set_item("lineno", range.start().to_u32())?;
            dict.set_item("col_offset", range.start().to_u32())?;
            dict.set_item("end_lineno", range.end().to_u32())?;
            dict.set_item("end_col_offset", range.end().to_u32())?;
        }

        Ok(func.call((), Some(&dict))?.into())
    }
    pub fn attr(&self, name: &str) -> PyResult<Bound<'py, PyAny>> {
        Ok(self.module.getattr(self.py, name)?.into_bound(self.py))
    }
    pub fn to_const<T: ToPyObject>(&self, value: T) -> PyResult<PyObject> {
        self.caller().attr("Constant").kwargs([("value", value)]).call()
    }
    pub fn to_module(&self, body: Vec<PyObject>) -> PyResult<PyObject> {
        self.caller().attr("Module").kwargs([("body", body)]).call()
    }
    pub fn to_joined_str<'a>(
        &self,
        range: TextRange,
        elements: impl Iterator<Item=&'a FStringElement>,
    ) -> PyResult<PyObject> {
        let mut values = vec![];
        for value in elements {
            values.push(value.to_ast(self)?);
        }
        self.caller().attr("JoinedStr")
            .range(range).kwargs([("values", values.into_py(self.py))]).call()
    }
}

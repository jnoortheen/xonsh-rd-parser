mod expr;
mod r#match;
mod stmt;

use crate::ast_module::AstModule;
use pyo3::{IntoPyObjectExt, PyObject};

type PyResult = pyo3::PyResult<PyObject>;

pub trait ToAst {
    fn to_ast(&self, module: &AstModule) -> PyResult;
}
// speciailized implementations
impl<T: ToAst> ToAst for Option<T> {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        match self {
            Some(param) => param.to_ast(module),
            None => Ok(module.py.None()),
        }
    }
}
impl<T: ToAst> ToAst for Box<T> {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        self.as_ref().to_ast(module)
    }
}
// Shared implementation for sequences
fn to_ast_sequence<T: ToAst>(items: &[T], module: &AstModule) -> PyResult {
    let py_objects: Vec<PyObject> = items
        .iter()
        .map(|item| item.to_ast(module))
        .collect::<Result<_, _>>()?;
    py_objects.into_py_any(module.py)
}

impl<T: ToAst> ToAst for Vec<T> {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        to_ast_sequence(self, module)
    }
}

impl<T: ToAst> ToAst for [T] {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        to_ast_sequence(self, module)
    }
}
impl ToAst for u32 {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        self.into_py_any(module.py)
    }
}

#[macro_export]
macro_rules! impl_to_ast {
    // Basic variant for direct value conversion
    ($type:ty, |$module:ident| $value:expr) => {
        impl ToAst for $type {
            fn to_ast(&self, $module: &AstModule) -> PyResult {
                $module.to_const(self.range(), $value)
            }
        }
    };
    ($type:ty, call $attr:literal) => {
        impl ToAst for $type {
            fn to_ast(&self, module: &AstModule) -> PyResult {
                module.attr($attr)?.callk(module.location(self.range))
            }
        }
    };
    // Variant for structs with multiple fields
    ($type:ty, call $attr:literal with fields [$($field:ident),+ $(,)?]) => {
        impl ToAst for $type {
            fn to_ast(&self, module: &AstModule) -> PyResult {
                module.attr($attr)?.call(
                    self.range,
                    [
                        $(
                            (stringify!($field), self.$field.to_ast(module)?),
                        )+
                    ],
                )
            }
        }
    };
    // Variant for structs with explicitly named fields (name-value mapping)
    ($type:ty, call $attr:literal with |$self:ident, $module: ident| {$($field_name:literal => $field_expr:expr),+ $(,)?}) => {
        impl ToAst for $type {
            fn to_ast(&$self, $module: &AstModule) -> PyResult {
                $module.attr($attr)?.call(
                    $self.range,
                    [
                        $(
                            ($field_name, $field_expr),
                        )+
                    ],
                )
            }
        }
    };
}

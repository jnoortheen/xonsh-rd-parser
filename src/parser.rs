use miette::{Diagnostic, NamedSource, Report, SourceSpan};
use py_ast::ast_module::AstModule;
use py_ast::to_ast::ToAst;
use pyo3::exceptions::PySyntaxError;
use pyo3::{PyObject, PyResult, Python};
use ruff_text_size::TextRange;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
#[error("{message}")]
struct PyParserError {
    message: String,
    #[source_code]
    src: NamedSource<String>,
    #[label("{message}")]
    span: SourceSpan,
}

fn annotate_err(msg: String, location: TextRange, src: &str, filename: &str) -> String {
    let offset = location.start().to_usize();
    let end_offset = location.end().to_usize();

    let err = PyParserError {
        message: msg,
        src: NamedSource::new(filename.to_string(), src.to_string()),
        span: (offset..end_offset).into(),
    };

    // Create a report and convert it to string
    let report = Report::new(err);
    format!("{:?}", report)
}

#[test]
fn test_annotation() {
    use ruff_text_size::TextSize;

    let text = r#"
def foo(a: int b: str) -> int:
    return a + b
$
"new line"
    "#;
    let result = annotate_err(
        "exception and error".to_string(),
        TextRange::new(TextSize::new(14), TextSize::new(20)),
        text,
        "test.py",
    );
    println!("{result}");
}

pub fn parse_str(py: Python<'_>, src: &str, filename: Option<&str>) -> PyResult<PyObject> {
    let parsed = ruff_python_parser::parse_module(src).map_err(|err| {
        let msg = format!("{:?}", err.error);
        let msg = annotate_err(msg, err.location, src, filename.unwrap_or("<string>"));
        PySyntaxError::new_err(msg)
    })?;
    let tree = parsed.into_syntax();
    let module = AstModule::new(py)?;
    tree.to_ast(&module)
}

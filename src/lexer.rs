use pyo3::exceptions::PySyntaxError;
use pyo3::{pyclass, PyResult, Python};
use ruff_python_parser::{lexer::Lexer, Mode};
use ruff_text_size::TextSize;

#[pyclass(get_all)]
pub(crate) struct Token {
    kind: String,
    start: usize,
    end: usize,
}

pub fn lex_str<'py>(
    py: Python<'py>,
    src: &'py str,
    filename: Option<&'py str>,
) -> PyResult<Vec<Token>> {
    let mut lexer = Lexer::new(src, Mode::Module, TextSize::default());

    let mut tokens = Vec::new();
    loop {
        let kind = lexer.next_token();
        if kind.is_eof() {
            break;
        }
        let range = lexer.current_range();
        tokens.push(Token {
            kind: format!("{:?}", kind),
            start: range.start().to_usize(),
            end: range.end().to_usize(),
        });
    }
    if let Some(err) = lexer.finish().pop() {
        let filename = filename.unwrap_or("<string>");
        let msg = crate::annotate_src::to_exc_msg(src, filename, &err.into());
        let err = PySyntaxError::new_err(msg);
        Err(err)
    } else {
        Ok(tokens)
    }
}

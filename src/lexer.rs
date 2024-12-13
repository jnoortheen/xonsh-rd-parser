use pyo3::exceptions::PySyntaxError;
use pyo3::prelude::*;
use pyo3::types::PyString;
use pyo3::{pyclass, PyResult, Python};
use ruff_python_parser::TokenKind;
use ruff_python_parser::{lexer::Lexer, Mode};
use ruff_text_size::TextRange;
use ruff_text_size::TextSize;

#[pyclass]
pub(crate) struct Token {
    kind: TokenKind,
    range: TextRange,
}

#[pymethods]
impl Token {
    #[getter]
    fn get_kind(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.kind))
    }
    #[getter]
    fn get_start(&self) -> usize {
        self.range.start().to_usize()
    }
    #[getter]
    fn get_end(&self) -> usize {
        self.range.end().to_usize()
    }
    #[getter]
    fn get_type(&self) -> PyResult<&str> {
        use TokenKind::*;

        // get Python token name
        let name = match self.kind {
            EndOfFile => "ENDMARKER",
            Name => "NAME",
            Int | Float | Complex => "NUMBER",
            String => "STRING",
            FStringStart => "FSTRING_START",
            FStringMiddle => "FSTRING_MIDDLE",
            FStringEnd => "FSTRING_END",
            Newline => "NEWLINE",
            Comment => "COMMENT",
            Indent => "INDENT",
            Dedent => "DEDENT",
            NonLogicalNewline => "NL",
            IpyEscapeCommand => unreachable!(),
            TokenKind::Unknown => "ErrorToken",
            _ => {
                if self.kind.is_operator() {
                    "OP"
                } else if self.kind.is_keyword() {
                    "NAME"
                } else {
                    "UNKNOWN"
                }
            }
        };
        Ok(name)
    }
}

#[pyclass(name = "Lexer", module = "xonsh_rd_parser")]
pub(crate) struct PyLexer {
    src: Py<PyString>,
    file: String,
}

#[pymethods]
impl PyLexer {
    #[new]
    #[pyo3(signature = (src, file_name = None))]
    fn new(src: Bound<'_, PyString>, file_name: Option<&'_ str>) -> PyResult<Self> {
        let file = file_name.unwrap_or("<string>").to_string();
        Ok(Self {
            src: src.into(),
            file,
        })
    }

    fn tokens(&self, py: Python<'_>) -> PyResult<Vec<Token>> {
        let src = self.src.to_str(py)?;
        let mut lexer = Lexer::new(src, Mode::Module, TextSize::default());

        let mut tokens = Vec::new();
        loop {
            let kind = lexer.next_token();
            if kind.is_eof() {
                break;
            }
            let range = lexer.current_range();
            tokens.push(Token { kind, range });
        }
        if let Some(err) = lexer.finish().pop() {
            let filename = self.file.as_str();
            let msg = crate::annotate_src::to_exc_msg(src, filename, &err.into());
            let err = PySyntaxError::new_err(msg);
            Err(err)
        } else {
            Ok(tokens)
        }
    }
}

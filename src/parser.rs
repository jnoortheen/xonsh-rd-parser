use crate::lexer::{LexerExt, Token};
use py_ast::ast_module::AstModule;
use py_ast::to_ast::ToAst;
use pyo3::prelude::*;
use pyo3::types::PyString;
use ruff_python_ast::ModModule;
use ruff_python_parser::{ParseError, Parsed};
use ruff_source_file::{LineIndex, SourceCode};
use ruff_text_size::Ranged;

struct PyParseError<'a>(ParseError, &'a str, &'a str);

impl<'a> PyParseError<'a> {
    fn to_err(err: ParseError, file: &'a str, src: &'a str) -> PyErr {
        PyParseError(err, file, src).into()
    }
}

impl From<PyParseError<'_>> for PyErr {
    fn from(err: PyParseError) -> PyErr {
        let filename = err.1;
        let src = err.2;
        crate::annotate_src::to_syntax_err(src, filename, &err.0)
    }
}

type ParseResult = PyResult<Parsed<ModModule>>;

#[pyclass(name = "Parser", module = "xonsh_rd_parser")]
pub struct PyParser {
    src: Py<PyString>,
    file: String,
}

impl PyParser {
    fn src(&self, py: Python<'_>) -> PyResult<&str> {
        self.src.to_str(py)
    }
    fn parse_module(&self, src: &str) -> ParseResult {
        ruff_python_parser::parse_module(src)
            .map_err(|err| PyParseError::to_err(err, self.file.as_str(), src))
    }
}

#[pymethods]
impl PyParser {
    #[new]
    #[pyo3(signature = (src, file_name = None))]
    fn new(src: Bound<'_, PyString>, file_name: Option<&'_ str>) -> PyResult<Self> {
        let file = file_name.unwrap_or("<code>").to_string();
        Ok(Self {
            src: src.into(),
            file,
        })
    }

    fn parse(&self, py: Python<'_>) -> PyResult<PyObject> {
        let src = self.src(py)?;
        let parsed = self.parse_module(src)?;
        let line_index = LineIndex::from_source_text(src);
        let source_code = SourceCode::new(src, &line_index);
        let tree = parsed.into_syntax();
        let module = AstModule::new(py, &source_code)?;
        tree.to_ast(&module)
    }

    #[staticmethod]
    fn parse_file(py: Python<'_>, path: &str) -> PyResult<PyObject> {
        let src = std::fs::read_to_string(path).unwrap();
        let src = PyString::new(py, &src);
        PyParser::new(src, Some(path))?.parse(py)
    }

    #[pyo3(signature = (tolerant=false))]
    fn tokens(&self, py: Python<'_>, tolerant: Option<bool>) -> PyResult<Vec<Token>> {
        let src = self.src(py)?;
        let tolerant = tolerant.unwrap_or(false);
        let line_index = LineIndex::from_source_text(src);
        let source_code = SourceCode::new(src, &line_index);
        let (tokens, err) = ruff_python_parser::lex_module(src);
        if let Some(err) = err {
            if !tolerant {
                return Err(PyParseError::to_err(err, self.file.as_str(), src));
            }
        }

        let tokens = tokens
            .iter()
            .map(|t| {
                Token::builder()
                    .kind(t.kind())
                    .range(t.range())
                    .source(&source_code)
                    .maybe_src(Some(self.src.clone_ref(py)))
                    .build()
            })
            .collect::<Vec<_>>();
        Ok(tokens)
    }

    #[pyo3(signature = (mincol = None, returnline = None, greedy = None, maxcol = None))]
    fn subproc_toks(
        &mut self,
        py: Python<'_>,
        mincol: Option<i64>,
        returnline: Option<bool>,
        greedy: Option<bool>,
        maxcol: Option<usize>,
    ) -> PyResult<Option<String>> {
        let src = self.src(py)?;
        let maxcol = maxcol.unwrap_or(src.len());
        let mincol = mincol.unwrap_or(-1);
        let returnline = returnline.unwrap_or(false);
        let greedy = greedy.unwrap_or(false);
        let tokens = self.tokens(py, None).ok().unwrap_or_default();
        let result = if let Some(range) = tokens.find_subproc_line(mincol, maxcol, greedy) {
            let line = format!("![{}]", &src[range]);

            if returnline {
                let line = format!(
                    "{}{}{}",
                    &src[..range.start().to_usize()],
                    line,
                    &src[range.end().to_usize()..]
                );
                Some(line)
            } else {
                Some(line)
            }
        } else {
            None
        };
        Ok(result)
    }
    /// Splits a string into a list of strings which are whitespace-separated tokens in proc mode.
    fn split(&self, py: Python<'_>) -> PyResult<Vec<String>> {
        let src = self.src(py)?;
        let result = self.tokens(py, Some(true))?.split_ws(src);
        Ok(result)
    }
}

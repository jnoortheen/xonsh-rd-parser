use crate::annotate_src::to_syntax_err;
use crate::lexer::{LexerExt, Token};
use py_ast::ast_module::AstModule;
use py_ast::to_ast::ToAst;
use pyo3::prelude::*;
use pyo3::types::PyString;
use ruff_python_ast::ModModule;
use ruff_python_parser::{ParseError, Parsed};
use ruff_source_file::{LineIndex, SourceCode};
use ruff_text_size::Ranged;

// type ParseResult = PyResult<Parsed<ModModule>>;

#[pyclass(name = "Parser", module = "xonsh_rd_parser")]
pub struct PyParser {
    src: Py<PyString>,
    file: String,
    line_index: LineIndex,
}

impl PyParser {
    fn code(&self, py: Python<'_>) -> PyResult<SourceCode<'_, '_>> {
        let src = self.src.to_str(py)?;
        let code = SourceCode::new(src, &self.line_index);
        Ok(code)
    }
    fn convert_err(&self, code: &SourceCode, error: &ParseError) -> PyErr {
        to_syntax_err(self.file.as_str(), code, error)
    }
    fn parse_module(&self, src: &SourceCode) -> PyResult<Parsed<ModModule>> {
        ruff_python_parser::parse_module(src.text()).map_err(|err| self.convert_err(src, &err))
    }
}

#[pymethods]
impl PyParser {
    #[new]
    #[pyo3(signature = (src, file_name = None))]
    fn new(src: Bound<'_, PyString>, file_name: Option<&'_ str>) -> PyResult<Self> {
        let file = file_name.unwrap_or("<code>").to_string();
        let line_index = LineIndex::from_source_text(src.to_str()?);
        Ok(Self {
            src: src.into(),
            file,
            line_index,
        })
    }

    fn parse(&self, py: Python<'_>) -> PyResult<PyObject> {
        let source_code = self.code(py)?;
        let parsed = self.parse_module(&source_code)?;
        let tree = parsed.into_syntax();
        let module = AstModule::new(py, &source_code)?;
        tree.to_ast(&module)
    }

    #[staticmethod]
    pub fn parse_file(py: Python<'_>, path: &str) -> PyResult<PyObject> {
        let src = std::fs::read_to_string(path).unwrap();
        let src = PyString::new(py, &src);
        PyParser::new(src, Some(path))?.parse(py)
    }

    #[pyo3(signature = (tolerant=false))]
    fn tokens(&self, py: Python<'_>, tolerant: Option<bool>) -> PyResult<Vec<Token>> {
        let tolerant = tolerant.unwrap_or(false);
        let code = self.code(py)?;
        let (tokens, err) = ruff_python_parser::lex_module(code.text());
        if let Some(err) = err
            && !tolerant
        {
            return Err(self.convert_err(&code, &err));
        }

        let tokens = tokens
            .iter()
            .map(|t| {
                Token::builder()
                    .kind(t.kind())
                    .range(t.range())
                    .source(&code)
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
        let src = self.src.to_str(py)?;
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
        let src = self.src.to_str(py)?;
        let result = self.tokens(py, Some(true))?.split_ws(src);
        Ok(result)
    }
}

use pyo3::exceptions::PySyntaxError;
use pyo3::prelude::*;
use pyo3::types::PyString;
use pyo3::{pyclass, PyResult, Python};
use ruff_python_parser::TokenKind;
use ruff_python_parser::{lexer::Lexer, Mode};
use ruff_text_size::TextRange;
use ruff_text_size::TextSize;

#[derive(Debug, Clone)]
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

    #[pyo3(signature = (suffix = None))]
    fn has_suffix(&self, suffix: Option<&Self>) -> bool {
        dbg!(self, suffix);
        if let Some(next) = suffix {
            return next.range.start() == self.range.end();
        }
        false
    }
    #[pyo3(signature = (prefix = None))]
    fn has_prefix(&self, prefix: Option<&Self>) -> bool {
        if let Some(prefix) = prefix {
            return prefix.range.end() == self.range.start();
        }
        false
    }

    fn is_combinator(&self) -> bool {
        matches!(self.kind, TokenKind::And | TokenKind::Or)
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
        let mut tokens = self.tokens(py).ok().unwrap_or_default();
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
}

trait LexerExt {
    fn find_subproc_line(&mut self, mincol: i64, maxcol: usize, greedy: bool) -> Option<TextRange>;
}

impl LexerExt for Vec<Token> {
    /// Encapsulates tokens in a source code line in a uncaptured
    // subprocess ![] starting at a minimum column. If there are no tokens
    // (ie in a comment line) this returns None. If greedy is True, it will encapsulate
    // normal parentheses. Greedy is False by default.
    fn find_subproc_line(&mut self, mincol: i64, maxcol: usize, greedy: bool) -> Option<TextRange> {
        let mut toks: Vec<Token> = Vec::new();
        let mut lparens = Vec::new();
        let mut saw_macro = false;

        let mut iterator = self.iter().peekable();
        while let Some(token) = iterator.next() {
            let tok = token.kind;
            let pos = token.get_start();

            if pos >= maxcol && !tok.is_proc_end() {
                break;
            }

            if tok == TokenKind::Comment {
                break;
            }

            if !saw_macro && tok.is_macro() {
                saw_macro = true;
            }

            if saw_macro && !tok.is_macro_end() {
                let start = token.range.start();
                let mut end = token.range.end();
                loop {
                    match iterator.peek() {
                        Some(t) if t.kind.is_macro_end() => {
                            break;
                        }
                        Some(t) => {
                            end = t.range.end();
                            iterator.next(); // consume the token
                        }
                        None => break, // End of iterator
                    }
                }

                let range = TextRange::new(start, end);
                let new_token = Token {
                    kind: TokenKind::String,
                    range,
                };
                toks.push(new_token);
                continue;
            }

            if tok.is_open_paren() {
                lparens.push(tok);
            }

            if greedy && !lparens.is_empty() && lparens.contains(&TokenKind::Lpar) {
                toks.push(token.clone());
                if tok.is_rparen() {
                    lparens.pop();
                }
                continue;
            }

            if let Some(last) = toks.last() {
                if last.kind.is_proc_end() {
                    dbg!(&self);
                    if last.is_combinator() && last.has_suffix(Some(&token)) {
                        // pass
                    } else if is_not_lparen_and_rparen(&lparens, &last.kind) {
                        lparens.pop();
                    } else if pos < maxcol && !tok.is_macro_end() {
                        if !greedy {
                            toks.clear();
                        }
                        if tok.is_beg_skip() {
                            continue;
                        }
                    } else {
                        break;
                    }
                }
            } else if tok.is_beg_skip() {
                continue;
            }

            if (pos as i64) < mincol {
                continue;
            }

            toks.push(token.clone());

            // if tok.type_ == "WS" && tok.value == "\\" {
            //     continue;
            // }
            if matches!(tok, TokenKind::Newline | TokenKind::Dedent) {
                break;
            }
            // if matches!(tok, TokenKind::Dedent) {
            //     tok = handle_dedent_token(&mut toks, tok); //Needs Mutability fix
            //     break;
            // }
        }

        if let Some(last) = toks.last() {
            if last.kind.is_any_newline() {
                toks.pop();
            } else if last.kind.is_proc_end() {
                if is_not_lparen_and_rparen(&lparens, &last.kind)
                    || (greedy && last.kind.is_rparen())
                {
                    // pass
                } else if last.is_combinator() && last.has_prefix(toks.get(toks.len() - 2)) {
                    // pass
                } else {
                    toks.pop();
                }
            }
        }

        if let Some(start) = toks.first() {
            let start = start.range.start();
            if let Some(end) = toks.last() {
                let end = end.range.end();
                return Some(TextRange::new(start, end));
            }
        }

        return None;
    }
}

fn is_not_lparen_and_rparen(lparens: &[TokenKind], tok: &TokenKind) -> bool {
    if tok != &TokenKind::Rpar {
        return false;
    }
    for tok in lparens {
        if tok != &TokenKind::Lpar {
            return true;
        }
    }
    return false;
}

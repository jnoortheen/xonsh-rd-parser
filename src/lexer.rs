use crate::location::{HasKind, HasLocation};
use bon::bon;
use pyo3::prelude::*;
use pyo3::types::PyString;
use pyo3::{pyclass, PyResult};
use ruff_python_parser::TokenKind;
use ruff_source_file::{SourceCode, SourceLocation};
use ruff_text_size::{Ranged, TextRange, TextSize};
use std::iter::Peekable;
use std::ops::Range;

#[derive(Debug)]
#[pyclass]
pub(crate) struct Token {
    kind: TokenKind,
    range: TextRange,
    location: Range<SourceLocation>,
    src: Option<Py<PyString>>,
}

#[bon]
impl Token {
    #[builder]
    pub fn new<'a>(
        kind: TokenKind,
        range: TextRange,
        source: &SourceCode<'a, 'a>,
        src: Option<Py<PyString>>,
    ) -> Self {
        let location = {
            let start = source.source_location(range.start());
            let end = source.source_location(range.end());
            start..end
        };
        Self {
            kind,
            range,
            location,
            src,
        }
    }

    fn small(&self) -> SmallToken {
        SmallToken {
            kind: self.kind,
            range: self.range,
        }
    }
}
impl HasKind for SmallToken {
    fn kind(&self) -> TokenKind {
        self.kind
    }
}
impl HasKind for Token {
    fn kind(&self) -> TokenKind {
        self.kind
    }
}

impl HasLocation for Token {
    fn start(&self) -> SourceLocation {
        self.location.start.clone()
    }

    fn end(&self) -> SourceLocation {
        self.location.end.clone()
    }
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
        // get Python token name
        let name = self.type_str();
        Ok(name)
    }
    #[getter]
    fn get_lexpos(&self) -> usize {
        self.col_offset()
    }
    #[getter]
    fn get_lineno(&self) -> usize {
        self.lineno()
    }
    #[getter]
    fn get_value(&self, py: Python<'_>) -> PyResult<Option<&str>> {
        if let Some(src) = &self.src {
            let src = src.to_str(py)?;
            let value = &src[self.range()];
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }
}
impl Ranged for Token {
    fn range(&self) -> TextRange {
        self.range
    }
}

pub trait LexerExt {
    fn find_subproc_line(&mut self, mincol: i64, maxcol: usize, greedy: bool) -> Option<TextRange>;
}

struct SmallToken {
    kind: TokenKind,
    range: TextRange,
}
impl Ranged for SmallToken {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl SmallToken {
    fn has_suffix(&self, suffix: Option<&Self>) -> bool {
        if let Some(next) = suffix {
            return next.start() == self.end();
        }
        false
    }
    fn has_prefix(&self, prefix: Option<&Self>) -> bool {
        if let Some(prefix) = prefix {
            return prefix.end() == self.start();
        }
        false
    }
}
impl LexerExt for Vec<Token> {
    /// Encapsulates tokens in a source code line in an uncaptured
    // subprocess ![] starting at a minimum column. If there are no tokens
    // (ie in a comment line) this returns None. If greedy is True, it will encapsulate
    // normal parentheses. Greedy is False by default.
    fn find_subproc_line(&mut self, mincol: i64, maxcol: usize, greedy: bool) -> Option<TextRange> {
        let mut toks: Vec<SmallToken> = Vec::new();
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
                let end = handle_macro_tokens(&mut iterator, token.range.end());

                let range = TextRange::new(start, end);
                let new_token = SmallToken {
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
                toks.push(token.small());
                if tok.is_rparen() {
                    lparens.pop();
                }
                continue;
            }

            if let Some(last) = toks.last() {
                if last.kind.is_proc_end() {
                    if last.is_combinator() && last.has_suffix(Some(&token.small())) {
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

            toks.push(token.small());

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
                    || (last.is_combinator() && last.has_prefix(toks.get(toks.len() - 2)))
                {
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

        None
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
    false
}

fn handle_macro_tokens<'a, I: Iterator<Item = &'a Token>>(
    iterator: &mut Peekable<I>,
    end: TextSize,
) -> TextSize {
    let mut end = end;
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
    end
}

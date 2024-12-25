use ruff_python_parser::{ParseError, ParseErrorType};
use ruff_source_file::{LineIndex, OneIndexed, SourceCode, SourceLocation};
use ruff_text_size::TextRange;
use std::fmt::Formatter;

use crate::location::HasSrcLocation;
use annotate_snippets::display_list::{DisplayList, FormatOptions};
use annotate_snippets::snippet::{AnnotationType, Slice, Snippet, SourceAnnotation};
use pyo3::exceptions::PySyntaxError;
use pyo3::PyErr;

pub(crate) fn to_syntax_err(src: &str, filename: &str, err: &ParseError) -> PyErr {
    let line_index = LineIndex::from_source_text(src);
    let source_code = SourceCode::new(src, &line_index);
    let code_frame = CodeFrame::new(&source_code, err);
    let msg = format!("{err} in {filename}:\n{code_frame}");
    PySyntaxError::new_err((
        msg,
        (
            filename.to_string(),
            code_frame.lineno(),
            code_frame.col_offset(),
            "".to_string(),
            code_frame.end_lineno(),
            code_frame.end_col_offset(),
        ),
    ))
}
pub(crate) struct CodeFrame<'a> {
    range: TextRange,
    error: &'a ParseErrorType,
    source: &'a SourceCode<'a, 'a>,
}
impl HasSrcLocation for CodeFrame<'_> {
    fn start(&self) -> SourceLocation {
        self.source.source_location(self.range.start())
    }
    fn end(&self) -> SourceLocation {
        self.source.source_location(self.range.end())
    }
}
impl<'a> CodeFrame<'a> {
    pub(crate) fn new(source: &'a SourceCode<'_, '_>, error: &'a ParseError) -> Self {
        CodeFrame {
            range: error.location,
            error: &error.error,
            source,
        }
    }
}

impl std::fmt::Display for CodeFrame<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Copied and modified from ruff_linter/src/message/text.rs
        let content_start_index = self.source.line_index(self.range.start());
        let mut start_index = content_start_index.saturating_sub(2);

        // Trim leading empty lines.
        while start_index < content_start_index {
            if !self.source.line_text(start_index).trim().is_empty() {
                break;
            }
            start_index = start_index.saturating_add(1);
        }

        let content_end_index = self.source.line_index(self.range.end());
        let mut end_index = content_end_index
            .saturating_add(2)
            .min(OneIndexed::from_zero_indexed(self.source.line_count()));

        // Trim trailing empty lines.
        while end_index > content_end_index {
            if !self.source.line_text(end_index).trim().is_empty() {
                break;
            }

            end_index = end_index.saturating_sub(1);
        }

        let start_offset = self.source.line_start(start_index);
        let end_offset = self.source.line_end(end_index);

        let annotation_range = self.range - start_offset;
        let source = self.source.slice(TextRange::new(start_offset, end_offset));

        let start_char = source[TextRange::up_to(annotation_range.start())]
            .chars()
            .count();

        let char_length = source[annotation_range].chars().count();
        let label = format!("Syntax Error: {error}", error = self.error);

        let snippet = Snippet {
            title: None,
            slices: vec![Slice {
                source,
                line_start: start_index.get(),
                annotations: vec![SourceAnnotation {
                    label: &label,
                    annotation_type: AnnotationType::Error,
                    range: (start_char, start_char + char_length),
                }],
                // The origin (file name, line number, and column number) is already encoded
                // in the `label`.
                origin: None,
                fold: false,
            }],
            footer: Vec::new(),
            opt: FormatOptions::default(),
        };

        writeln!(f, "{message}", message = DisplayList::from(snippet))
    }
}

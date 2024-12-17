use ruff_python_parser::TokenKind;
use ruff_source_file::SourceLocation;

pub(crate) trait HasLocation {
    fn start(&self) -> SourceLocation;
    fn end(&self) -> SourceLocation;
    fn lineno(&self) -> usize {
        self.start().row.get()
    }
    fn end_lineno(&self) -> usize {
        self.end().row.get()
    }
    fn col_offset(&self) -> usize {
        self.start().column.get()
    }
    fn end_col_offset(&self) -> usize {
        self.end().column.get()
    }
}

pub(crate) trait HasKind {
    fn kind(&self) -> TokenKind;
    fn type_str(&self) -> &str {
        use TokenKind::*;

        // get Python token name
        match self.kind() {
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
            Unknown => "ErrorToken",
            _ => {
                if self.kind().is_operator() {
                    "OP"
                } else if self.kind().is_keyword() {
                    "NAME"
                } else {
                    "UNKNOWN"
                }
            }
        }
    }
    fn is_combinator(&self) -> bool {
        matches!(self.kind(), TokenKind::And | TokenKind::Or)
    }
}
